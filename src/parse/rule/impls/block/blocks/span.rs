/*
 * parse/rule/impls/block/blocks/span.rs
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

pub const BLOCK_SPAN: BlockRule = BlockRule {
    name: "block-span",
    accepts_names: &["span", "span_"],
    accepts_special: false,
    newline_separator: false,
    parse_fn,
};

fn parse_fn<'r, 't>(
    log: &slog::Logger,
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    special: bool,
    in_head: bool,
) -> ParseResult<'r, 't, Element<'t>> {
    debug!(
        log,
        "Parsing span block";
        "in-head" => in_head,
        "name" => name,
    );

    assert_eq!(special, false, "Span doesn't allow special variant");
    assert_block_name(&BLOCK_SPAN, name);

    let mut arguments = parser.get_head_map(&BLOCK_SPAN, in_head)?;

    // "span" means we wrap interpret as-is
    // "span_" means we strip out any newlines or paragraph breaks
    let strip_line_breaks = name.ends_with('_');

    // Get styling arguments
    let id = arguments.get("id");
    let class = arguments.get("class");
    let style = arguments.get("style");

    // Get body content, without paragraphs
    let (mut elements, exceptions) = parser.get_body_elements(&BLOCK_SPAN, false)?.into();

    if strip_line_breaks {
        // Remove leading line breaks
        while let Some(element) = elements.first() {
            if !matches!(element, Element::LineBreak | Element::LineBreaks(_)) {
                break;
            }

            elements.remove(0);
        }

        // Remove trailing line breaks
        while let Some(element) = elements.last() {
            if !matches!(element, Element::LineBreak | Element::LineBreaks(_)) {
                break;
            }

            elements.pop();
        }
    }

    let element = Element::Span {
        elements,
        id,
        class,
        style,
    };

    ok!(element, exceptions)
}
