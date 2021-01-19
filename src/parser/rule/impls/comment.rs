/*
 * parser/rule/impls/comment.rs
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

use super::prelude::*;
use crate::span_wrap::SpanWrap;

pub const RULE_COMMENT: Rule = Rule {
    name: "comment",
    try_consume_fn,
};

fn try_consume_fn<'p, 'r, 't>(
    log: &slog::Logger,
    parser: &'p mut Parser<'r, 't>,
) -> ParseResult<'r, 't, Element<'t>> {
    debug!(log, "Consuming tokens until end of comment");

    check_step(parser, Token::LeftComment)?;

    loop {
        let ExtractedToken { token, span, slice } = parser.current();

        debug!(
            log,
            "Received token inside comment";
            "token" => token,
            "slice" => slice,
            "span" => SpanWrap::from(span),
        );

        match token {
            // Hit the end of the comment, return
            Token::RightComment => {
                trace!(log, "Reached end of comment, returning");
                parser.step()?;
                return ok!(Element::Null);
            }

            // Hit the end of the input, abort
            Token::InputEnd => {
                trace!(log, "Reached end of input, aborting");

                return Err(parser.make_warn(ParseWarningKind::EndOfInput));
            }

            // Consume any other token
            _ => {
                trace!(log, "Token inside comment received. Discarding.");
                parser.step()?;
            }
        }
    }
}
