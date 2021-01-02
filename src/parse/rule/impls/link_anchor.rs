/*
 * parse/rule/impls/link_anchor.rs
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

//! Rules for anchor soft-links.
//!
//! A variant on single-bracket links which targets an anchor on the current page,
//! or is a fake link.

use super::prelude::*;
use crate::enums::{AnchorTarget, LinkLabel};
use std::borrow::Cow;
use wikidot_normalize::normalize;

pub const RULE_LINK_ANCHOR: Rule = Rule {
    name: "link-anchor",
    try_consume_fn,
};

fn try_consume_fn<'p, 'r, 't>(
    log: &slog::Logger,
    parser: &'p mut Parser<'r, 't>,
) -> ParseResult<'r, 't, Element<'t>> {
    debug!(log, "Trying to create a single-bracket anchor link");

    assert_eq!(
        parser.current().token,
        Token::LeftAnchor,
        "Current token isn't left anchor",
    );

    // Gather path for link
    let (url, _, mut exceptions) = try_merge(
        log,
        parser,
        RULE_LINK_ANCHOR,
        &[ParseCondition::current(Token::Whitespace)],
        &[
            ParseCondition::current(Token::RightBracket),
            ParseCondition::current(Token::ParagraphBreak),
            ParseCondition::current(Token::LineBreak),
        ],
    )?
    .into();

    // Determine if this is an anchor link or fake link
    let url = if url.is_empty() {
        Cow::Borrowed("javascript:;")
    } else {
        // Make URL "#name", where 'name' is normalized.
        let mut url = str!(url);
        normalize(&mut url);
        url.insert(0, '#');

        Cow::Owned(url)
    };

    // Gather label for link
    let label = try_merge(
        log,
        parser,
        RULE_LINK_ANCHOR,
        &[ParseCondition::current(Token::RightBracket)],
        &[
            ParseCondition::current(Token::ParagraphBreak),
            ParseCondition::current(Token::LineBreak),
        ],
    )?
    .chain(&mut exceptions);

    debug!(
        log,
        "Retrieved label for link, now build element";
        "label" => label,
    );

    // Trim label
    let label = label.trim();

    // Build link element
    let element = Element::Link {
        url,
        label: LinkLabel::Text(cow!(label)),
        anchor: AnchorTarget::Same,
    };

    // Return result
    ok!(element, exceptions)
}
