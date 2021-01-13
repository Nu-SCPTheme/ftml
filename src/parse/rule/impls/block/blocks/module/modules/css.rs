/*
 * parse/rule/impls/block/blocks/module/modules/css.rs
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

pub const MODULE_CSS: ModuleRule = ModuleRule {
    name: "module-css",
    accepts_names: &["CSS"],
    parse_fn,
};

fn parse_fn<'r, 't>(
    log: &slog::Logger,
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    arguments: Arguments<'t>,
) -> ParseResult<'r, 't, Module<'t>> {
    debug!(log, "Parsing categories module");

    assert!(
        name.eq_ignore_ascii_case("Categories"),
        "Module doesn't have a valid name",
    );

    let css = parser.get_body_text(&BLOCK_MODULE)?;
    let exceptions = vec![ParseException::Style(cow!(css))];

    ok!(Module::Null, exceptions)
}
