/*
 * parse/rule/impls/color.rs
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

use super::prelude::*;

pub const RULE_COLOR: Rule = Rule {
    name: "color",
    try_consume_fn,
};

fn try_consume_fn<'t, 'r>(
    log: &slog::Logger,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    full_text: FullText<'t>,
) -> Consumption<'t, 'r> {
    debug!(log, "Trying to create color container");

    assert_eq!(
        extracted.token,
        Token::Color,
        "Current token isn't color marker",
    );

    // The pattern for color is:
    // ## [color-style] | [text to be colored] ##

    // Gather the color name until the separator
    let consumption = try_merge(
        log,
        (extracted, remaining, full_text),
        RULE_COLOR,
        &[Token::Pipe],
        &[Token::ParagraphBreak, Token::LineBreak, Token::InputEnd],
        &[],
    );

    // Return if failure
    let (color, new_remaining, mut all_errors) = try_consume!(consumption);

    // Get last token as the current, "extracted" value.
    // try_container() expects the first token to be signifier, not content
    // In this case, it would be the "|" between color and contents.
    //
    // So we look for the pipe token. Kind of hacky, better than producing
    // a fake ExtractedToken or adding conditional logic to try_container().
    let (extracted, remaining) = {
        // Find the Token::Pipe from the old token pointer.
        // It must exist, as we've already crossed it.
        let extracted = remaining
            .iter()
            .find(|e| e.token == Token::Pipe)
            .expect("Pipe not found after try_merge() succeeded");

        (extracted, new_remaining)
    };

    debug!(
        log,
        "Retrieved color descriptor, now building container";
        "color" => color,
    );

    // Build color container
    let consumption = try_container(
        log,
        (extracted, remaining, full_text),
        (RULE_COLOR, ContainerType::Color(color)),
        (Token::Pipe, Token::Color),
        &[Token::ParagraphBreak, Token::InputEnd],
        &[],
    );

    // Append errors, or return if failure
    let (item, remaining, mut errors) = try_consume!(consumption);

    // Add on new errors
    all_errors.append(&mut errors);

    // Return result
    GenericConsumption::warn(item, remaining, all_errors)
}
