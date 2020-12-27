/*
 * parse/rule/impls/block/mod.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Meta-rule for all block constructs.
//!
//! This matches `[[` or `[[*` and runs the block parsing
//! against the upcoming tokens in accordance to how the
//! various blocks define themselves.

use crate::parse::consume::{Consumption, GenericConsumption};
use crate::parse::rule::collect::try_merge;
use crate::parse::rule::Rule;
use crate::parse::token::{ExtractedToken, Token};
use crate::parse::{ParseError, ParseErrorKind, ParseException};
use crate::text::FullText;
use crate::tree::Element;
use std::borrow::Cow;
use std::collections::HashMap;

mod mapping;
mod rule;

pub mod impls;

pub use self::rule::{RULE_BLOCK, RULE_BLOCK_SPECIAL};

#[derive(Debug)]
pub struct BlockParser<'l, 'r, 't> {
    log: &'l slog::Logger,
    special: bool,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    full_text: FullText<'t>,
    rule: Rule,
}

impl<'l, 'r, 't> BlockParser<'l, 'r, 't>
where
    'r: 't,
{
    #[inline]
    pub fn new(
        log: &'l slog::Logger,
        special: bool,
        extracted: &'r ExtractedToken<'t>,
        remaining: &'r [ExtractedToken<'t>],
        full_text: FullText<'t>,
    ) -> Self {
        debug!(
            log, "Creating block parser";
            "special" => special,
            "remaining-len" => remaining.len(),
        );

        let rule = if special {
            RULE_BLOCK_SPECIAL
        } else {
            RULE_BLOCK
        };

        BlockParser {
            log,
            special,
            extracted,
            remaining,
            full_text,
            rule,
        }
    }

    // Pointer state and manipulation
    pub fn step(&mut self) -> Result<(), ParseError> {
        trace!(self.log, "Stepping to the next token");

        match self.remaining.split_first() {
            Some((extracted, remaining)) => {
                self.extracted = extracted;
                self.remaining = remaining;
                Ok(())
            }
            None => Err(self.make_error(ParseErrorKind::EndOfInput)),
        }
    }

    #[inline]
    pub fn update(
        &mut self,
        remaining: &'r [ExtractedToken<'t>],
    ) -> Result<(), ParseError> {
        trace!(self.log, "Updating token pointer to new value");

        self.remaining = remaining;
        self.step()
    }

    // Parsing methods
    fn get_token(
        &mut self,
        token: Token,
        kind: ParseErrorKind,
    ) -> Result<&'t str, ParseError> {
        trace!(
            self.log,
            "Looking for token {:?} (error {:?})",
            token,
            kind;
            "token" => token,
            "error-kind" => kind,
        );

        if self.extracted.token == token {
            let text = self.extracted.slice;
            self.step()?;
            Ok(text)
        } else {
            Err(self.make_error(kind))
        }
    }

    #[inline]
    pub fn get_identifier(
        &mut self,
        kind: ParseErrorKind,
    ) -> Result<&'t str, ParseError> {
        trace!(self.log, "Looking for identifier");

        self.get_token(Token::Identifier, kind)
    }

    pub fn get_optional_space(&mut self) -> Result<(), ParseError> {
        trace!(self.log, "Looking for optional space");

        if self.extracted.token == Token::Whitespace {
            self.step()?;
        }

        Ok(())
    }

    // Block argument parsing
    pub fn get_argument_map(
        &mut self,
    ) -> Result<HashMap<&'t str, Cow<'t, str>>, ParseError> {
        trace!(self.log, "Looking for key value arguments, then ']]'");

        todo!()
    }

    pub fn get_argument_value(&mut self) -> Result<&'t str, ParseError> {
        trace!(self.log, "Looking for a value argument, then ']]'");

        let consumption = try_merge(
            self.log,
            (self.extracted, self.remaining, self.full_text),
            self.rule,
            &[Token::RightBlock],
            &[Token::ParagraphBreak, Token::LineBreak],
            &[],
        );

        // We ignore exceptions because try_merge() produces none.
        match consumption {
            GenericConsumption::Failure { error } => Err(error),
            GenericConsumption::Success {
                item: value,
                remaining,
                exceptions: _,
            } => {
                self.update(remaining)?;

                Ok(value)
            }
        }
    }

    pub fn get_argument_none(&mut self) -> Result<(), ParseError> {
        trace!(self.log, "No arguments, looking for ']]'");

        self.get_optional_space()?;
        self.get_token(Token::RightBlock, ParseErrorKind::BlockMissingCloseBrackets)?;
        Ok(())
    }

    // Utilities
    #[inline]
    pub fn set_block(&mut self, block_rule: &BlockRule) {
        info!(
            self.log,
            "Running block rule {} for these tokens",
            block_rule.name;
        );

        self.rule = block_rule.rule();
    }

    #[cold]
    #[inline]
    pub fn make_error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError::new(kind, self.rule, self.extracted)
    }
}

/// Define a rule for how to parse a block.
#[derive(Clone)]
pub struct BlockRule {
    /// The code name of the block.
    ///
    /// As this is an internal structure, we can assert the following things:
    /// * It is in kebab-case.
    /// * It is globally unique.
    /// * It is prefixed with `block-`.
    name: &'static str,

    /// Which names you can use this block with. Case-insensitive.
    /// Will panic if empty.
    accepts_names: &'static [&'static str],

    /// Whether this block accepts `*` as a modifier.
    ///
    /// For instance, user can be invoked as both
    /// `[[user aismallard]]` and `[[*user aismallard]]`.
    accepts_special: bool,

    /// Function which implements the processing for this rule.
    parse_fn: BlockParseFn,
}

impl BlockRule {
    /// Produces a pseudo parse `Rule` associated with this `BlockRule`.
    ///
    /// It should not be invoked, it is for error construction.
    #[cold]
    pub fn rule(&self) -> Rule {
        // Stubbed try_consume_fn implementation for the Rule.
        fn try_consume_fn<'r, 't>(
            _: &slog::Logger,
            _: &'r ExtractedToken<'t>,
            _: &'r [ExtractedToken<'t>],
            _: FullText<'t>,
        ) -> Consumption<'r, 't> {
            panic!("Pseudo rule for this block should not be executed directly!");
        }

        Rule {
            name: self.name,
            try_consume_fn,
        }
    }
}

#[derive(Debug)]
pub struct BlockParseOutcome<'r, 't>
where
    'r: 't,
{
    element: Element<'t>,
    remaining: &'r [ExtractedToken<'t>],
    exceptions: Vec<ParseException<'t>>,
}

impl<'r, 't> Into<Consumption<'r, 't>> for BlockParseOutcome<'r, 't> {
    #[inline]
    fn into(self) -> Consumption<'r, 't> {
        let BlockParseOutcome {
            element,
            remaining,
            exceptions,
        } = self;

        Consumption::warn(element, remaining, exceptions)
    }
}

pub type BlockParseFn =
    for<'l, 'r, 't> fn(
        &'l slog::Logger,
        &mut BlockParser<'l, 'r, 't>,
        &'t str,
        bool,
    ) -> Result<BlockParseOutcome<'r, 't>, ParseError>;
