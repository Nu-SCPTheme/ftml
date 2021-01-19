/*
 * parsing/rule/impls/text.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
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

pub const RULE_TEXT: Rule = Rule {
    name: "text",
    try_consume_fn: text,
};

pub const RULE_BAD_TEXT: Rule = Rule {
    name: "text-bad",
    try_consume_fn: text_bad,
};

fn text<'p, 'r, 't>(
    log: &slog::Logger,
    parser: &'p mut Parser<'r, 't>,
) -> ParseResult<'r, 't, Element<'t>> {
    let ExtractedToken { slice, token, .. } = parser.current();

    debug!(log, "Consuming token as plain text element"; "token" => token);

    ok!(text!(slice))
}

fn text_bad<'p, 'r, 't>(
    log: &slog::Logger,
    parser: &'p mut Parser<'r, 't>,
) -> ParseResult<'r, 't, Element<'t>> {
    let ExtractedToken { slice, token, .. } = parser.current();

    info!(log, "Consuming otherwise invalid token as plain text element"; "token" => token);

    ok!(text!(slice))
}
