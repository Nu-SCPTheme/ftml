/*
 * parse/rule/impls/block/blocks/div.rs
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

pub const BLOCK_DIV: BlockRule = BlockRule {
    name: "block-div",
    accepts_names: &["div", "div_"],
    accepts_special: false,
    newline_separator: true,
    parse_fn,
};

fn parse_fn<'r, 't>(
    log: &slog::Logger,
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    special: bool,
    in_block: bool,
) -> ParseResult<'r, 't, Element<'t>> {
    debug!(
        log,
        "Parsing div block";
        "in-block" => in_block,
        "name" => name,
    );

    assert_eq!(special, false, "Div doesn't allow special variant");

    let mut arguments = parser.get_head_map(&BLOCK_DIV, in_block)?;

    // "div" means we wrap in paragraphs, like normal
    // "div_" means we don't wrap it
    let wrap_paragraphs = !name.ends_with('_');

    // Get styling arguments
    let id = arguments.get("id");
    let class = arguments.get("class");
    let style = arguments.get("style");

    // Get body content, based on whether we want paragraphs or not
    let (elements, exceptions) = parser
        .get_body_elements(&BLOCK_DIV, wrap_paragraphs)?
        .into();

    // Build element and return
    let element = Element::Div {
        elements,
        id,
        class,
        style,
    };

    ok!(element, exceptions)
}
