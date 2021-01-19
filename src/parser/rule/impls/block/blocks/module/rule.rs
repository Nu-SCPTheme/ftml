/*
 * parser/rule/impls/block/blocks/module/rule.rs
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

use super::mapping::get_module_rule_with_name;
use super::prelude::*;
use crate::tree::Module;

pub const BLOCK_MODULE: BlockRule = BlockRule {
    name: "block-module",
    accepts_names: &["module", "module654"],
    accepts_special: false,
    newline_separator: true,
    parse_fn,
};

fn parse_fn<'r, 't>(
    log: &slog::Logger,
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    special: bool,
    in_head: bool,
) -> ParseResult<'r, 't, Element<'t>> {
    debug!(log, "Parsing module block"; "in-head" => in_head);

    assert_eq!(special, false, "Module doesn't allow special variant");
    assert_block_name(&BLOCK_MODULE, name);

    // Get module name and arguments
    let (subname, arguments) = parser.get_head_name_map(&BLOCK_MODULE, in_head)?;

    // Get the module rule for this name
    let module_rule = match get_module_rule_with_name(subname) {
        Some(rule) => rule,
        None => return Err(parser.make_warn(ParseWarningKind::NoSuchModule)),
    };

    // Prepare to run the module's parsing function
    parser.set_module(module_rule);

    // Run the parse function until the end.
    // This starts after the head and its newline.
    //
    // If the module accepts a body, it should consume it,
    // then the tail. Otherwise it shouldn't move the token pointer.
    let (module, exceptions) =
        (module_rule.parse_fn)(log, parser, subname, arguments)?.into();

    ok!(build_element(module), exceptions)
}

fn build_element(module: Module) -> Element {
    if module == Module::Null {
        Element::Null
    } else {
        Element::Module(module)
    }
}
