/*
 * parse/rule/impls/block/impls/code.rs
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

use super::prelude::*;

pub const BLOCK_CODE: BlockRule = BlockRule {
    name: "block-code",
    accepts_names: &["code"],
    accepts_special: false,
    parse_fn,
};

fn parse_fn<'p, 'r, 't>(
    log: &slog::Logger,
    parser: &'p mut BlockParser<'p, 'r, 't>,
    name: &'t str,
    special: bool,
    in_block: bool,
) -> ParseResult<'r, 't, Element<'t>> {
    assert_eq!(special, false, "Code doesn't allow special variant");
    assert!(
        name.eq_ignore_ascii_case("code"),
        "Code doesn't have a valid name",
    );

    let language = if in_block {
        let mut arguments = parser.get_argument_map()?;
        arguments.get("type")
    } else {
        None
    };

    // The block must be on its own line
    parser.get_line_break()?;

    // Check if it's an empty block
    if let Ok(name) = parser.get_end_block() {
        if name.eq_ignore_ascii_case("code") {
            return ok!(Element::Code {
                contents: cow!(""),
                language,
            });
        }
    }

    let start = parser.current();
    let end;

    // Keep iterating until we find the end.
    // Preserve parse progress if we've hit the end block.
    loop {
        let at_end_block = parser.keep_evaluate_fn(|parser| {
            // Check that "[[/code]]" is on a new line.
            parser.get_line_break()?;

            // Check if it's an end block
            //
            // This will ignore any errors produced,
            // since it's just more code
            let name = parser.get_end_block()?;

            // Check if it's the right kind
            let is_code = name.eq_ignore_ascii_case("code");

            Ok(is_code)
        });

        if let Some(last_token) = at_end_block {
            end = last_token;
            break;
        }

        parser.step()?;
    }

    let code = parser.full_text().slice_partial(log, start, end);
    let element = Element::Code {
        contents: cow!(code),
        language,
    };

    ok!(element)
}
