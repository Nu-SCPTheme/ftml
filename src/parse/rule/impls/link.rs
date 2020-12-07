/*
 * parse/rule/impls/link.rs
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
use crate::enums::{AnchorTarget, LinkLabel};

pub const RULE_LINK: Rule = Rule {
    name: "link",
    try_consume_fn: link,
};

pub const RULE_LINK_TAB: Rule = Rule {
    name: "link",
    try_consume_fn: link_new_tab,
};

fn link<'t, 'r>(
    log: &slog::Logger,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    full_text: FullText<'t>,
) -> Consumption<'t, 'r> {
    trace!(log, "Trying to create a bare link (regular)");

    try_consume_link(
        log,
        extracted,
        remaining,
        full_text,
        RULE_LINK,
        AnchorTarget::Same,
    )
}

fn link_new_tab<'t, 'r>(
    log: &slog::Logger,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    full_text: FullText<'t>,
) -> Consumption<'t, 'r> {
    trace!(log, "Trying to create a bare link (new tab)");

    try_consume_link(
        log,
        extracted,
        remaining,
        full_text,
        RULE_LINK_TAB,
        AnchorTarget::NewTab,
    )
}

fn try_consume_link<'t, 'r>(
    log: &slog::Logger,
    extracted: &'r ExtractedToken<'t>,
    remaining: &'r [ExtractedToken<'t>],
    full_text: FullText<'t>,
    rule: Rule,
    anchor: AnchorTarget,
) -> Consumption<'t, 'r> {
    debug!(log, "Trying to create a bare link"; "anchor" => anchor.name());

    // Gather path for link
    let consumption = try_merge(
        log,
        (extracted, remaining, full_text),
        rule,
        &[Token::Whitespace],
        &[Token::ParagraphBreak, Token::LineBreak, Token::InputEnd],
        &[],
    );

    // Return if failure
    let (url, new_remaining, mut all_errors) = try_consume!(consumption);

    // Get last token so try_container() can match starting on it
    let (extracted, remaining) = (
        last_before_slice(remaining, new_remaining),
        new_remaining,
    );

    debug!(
        log,
        "Retrieved URL for link, now building element";
        "url" => url,
    );

    // Gather label for link
    let consumption = try_merge(
        log,
        (extracted, remaining, full_text),
        rule,
        &[Token::RightBracket],
        &[Token::ParagraphBreak, Token::LineBreak, Token::InputEnd],
        &[],
    );

    // Append errors, or return if failure
    let (label, remaining, mut errors) = try_consume!(consumption);

    // Add on new errors
    all_errors.append(&mut errors);

    // Build link element
    let label = LinkLabel::Text(label);
    let element = Element::Link {
        url,
        label,
        anchor,
    };

    // Return result
    Consumption::warn(element, remaining, all_errors)
}
