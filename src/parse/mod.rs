/*
 * parse/mod.rs
 *
 * ftml - Library to parse Wikidot code
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

mod consume;
mod error;
mod result;
mod rule;
mod token;

use self::consume::consume;
use self::rule::{Consumption, ConsumptionResult};
use crate::tree::SyntaxTree;
use slog::Logger;

pub use self::error::{ParseError, ParseErrorKind};
pub use self::result::ParseResult;
pub use self::token::Token;

pub fn parse<'a>(log: &Logger, text: &'a str) -> ParseResult<SyntaxTree<'a>> {
    let log = &log.new(slog_o!("function" => "parse", "text" => str!(text)));

    info!(log, "Running parser on text");

    let tokens = Token::extract_all(log, text);
    let mut tokens = tokens.as_slice();
    let mut output = ParseResult::default();

    while !tokens.is_empty() {
        // Consume tokens to get next element
        let Consumption { result, error } = {
            let (extracted, next) = tokens
                .split_first() //
                .expect("Tokens list is empty");

            consume(log, extracted, next)
        };

        match result {
            ConsumptionResult::Success { element, remaining } => {
                //TODO log

                // Update remaining tokens
                //
                // The new value is a subslice of tokens,
                // equivalent to &tokens[offset..] but without
                // needing to assert bounds.
                tokens = remaining;

                // Add the new element to the list
                output.push(element);
            }
            ConsumptionResult::Failure => {
                //TODO log
            }
        }

        if let Some(error) = error {
            //TODO log
            output.append_err(error);
        }
    }

    debug!(log, "Finished running parser, returning gathered elements");
    SyntaxTree::from_element_result(output)
}

#[test]
fn ast() {
    let logger = crate::build_logger();
    let text = "test string";
    let tree = parse(&logger, text);
    println!("Tree: {:#?}", tree);
}
