/*
 * parse/consume.rs
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

//! Module for look-ahead checking.
//!
//! This contains implementations of eager functions that try to interpret the
//! upcoming tokens as a particular object (e.g. seeing a `[[` and you see if it's a module).
//!
//! The parser is not disambiguous because any string of tokens can be interpreted
//! as raw text as a fallback, which is how Wikidot does it.

use super::rule::{impls::RULE_FALLBACK, rules_for_token};
use super::token::ExtractedToken;
use super::{ParseError, ParseErrorKind, ParseException};
use crate::text::FullText;
use crate::tree::Element;
use std::mem;

/// Main function that consumes tokens to produce a single element, then returns.
pub fn consume<'r, 't>(
    log: &slog::Logger,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    full_text: FullText<'t>,
) -> Consumption<'r, 't> {
    let ExtractedToken { token, slice, span } = extracted;
    let log = &log.new(slog_o!(
        "token" => str!(token.name()),
        "slice" => str!(slice),
        "span-start" => span.start,
        "span-end" => span.end,
        "remaining-len" => remaining.len(),
    ));

    debug!(log, "Looking for valid rules");

    for rule in rules_for_token(extracted) {
        info!(log, "Trying rule consumption for tokens"; "rule" => rule);

        let consumption = rule.try_consume(log, extracted, remaining, full_text);
        if consumption.is_success() {
            debug!(log, "Rule matched, returning generated result"; "rule" => rule);

            return consumption;
        }

        // Discard invalid consumption
        mem::drop(consumption);
    }

    debug!(log, "All rules exhausted, using generic text fallback");

    let error = ParseException::Error(ParseError::new(
        ParseErrorKind::NoRulesMatch,
        RULE_FALLBACK,
        extracted,
    ));

    Consumption::warn(text!(slice), remaining, vec![error])
}

#[derive(Debug, Clone)]
pub enum GenericConsumption<'r, 't, T>
where
    T: 't,
    'r: 't,
{
    Success {
        item: T,
        remaining: &'r [ExtractedToken<'t>],
        exceptions: Vec<ParseException<'t>>,
    },
    Failure {
        error: ParseError,
    },
}

impl<'r, 't, T> GenericConsumption<'r, 't, T>
where
    T: 't,
{
    #[inline]
    pub fn ok(item: T, remaining: &'r [ExtractedToken<'t>]) -> Self {
        GenericConsumption::Success {
            item,
            remaining,
            exceptions: Vec::new(),
        }
    }

    #[inline]
    pub fn warn(
        item: T,
        remaining: &'r [ExtractedToken<'t>],
        exceptions: Vec<ParseException<'t>>,
    ) -> Self {
        GenericConsumption::Success {
            item,
            remaining,
            exceptions,
        }
    }

    #[inline]
    pub fn err(error: ParseError) -> Self {
        GenericConsumption::Failure { error }
    }

    #[inline]
    pub fn is_success(&self) -> bool {
        match self {
            GenericConsumption::Success { .. } => true,
            GenericConsumption::Failure { .. } => false,
        }
    }

    #[inline]
    pub fn map<F, U>(self, f: F) -> GenericConsumption<'r, 't, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            GenericConsumption::Failure { error } => {
                GenericConsumption::Failure { error }
            }
            GenericConsumption::Success {
                item,
                remaining,
                exceptions,
            } => {
                let item = f(item);

                GenericConsumption::Success {
                    item,
                    remaining,
                    exceptions,
                }
            }
        }
    }
}

pub type Consumption<'r, 't> = GenericConsumption<'r, 't, Element<'t>>;
