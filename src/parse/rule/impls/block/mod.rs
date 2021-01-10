/*
 * parse/rule/impls/block/mod.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Ammon Smith
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

use crate::parse::result::ParseResult;
use crate::parse::rule::Rule;
use crate::parse::Parser;
use crate::tree::Element;
use std::fmt::{self, Debug};

mod arguments;
mod mapping;
mod parser;
mod rule;

pub mod impls;

pub use self::arguments::Arguments;
pub use self::rule::{RULE_BLOCK, RULE_BLOCK_SPECIAL};

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
        fn try_consume_fn<'p, 'r, 't>(
            _: &slog::Logger,
            _: &'p mut Parser<'r, 't>,
        ) -> ParseResult<'r, 't, Element<'t>> {
            panic!("Pseudo rule for this block should not be executed directly!");
        }

        Rule {
            name: self.name,
            try_consume_fn,
        }
    }
}

impl Debug for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BlockRule")
            .field("name", &self.name)
            .field("accepts_names", &self.accepts_names)
            .field("parse_fn", &(self.parse_fn as *const ()))
            .finish()
    }
}

/// Function pointer type to implement block parsing.
///
/// The arguments are, in order:
/// * `log` -- Logger instance
/// * `parser` -- Parser instance
/// * `name` -- The name of the block
/// * `special` -- Whether this block is `[[*` (special) or `[[` (regular)
/// * `in_block` -- Whether we're still in the block head, or if it's finished
pub type BlockParseFn = for<'r, 't> fn(
    &slog::Logger,
    &mut Parser<'r, 't>,
    &'t str,
    bool,
    bool,
) -> ParseResult<'r, 't, Element<'t>>;
