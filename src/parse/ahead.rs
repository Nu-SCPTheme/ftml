/*
 * parse/ahead.rs
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

//! Module for look-ahead checking.
//!
//! This contains implementations of eager functions that try to interpret the
//! upcoming tokens as a particular object (e.g. seeing a `[[` and you see if it's a module).
//!
//! The parser is not disambiguous because any string of tokens can be interpreted
//! as raw text as a fallback, which is how Wikidot does it.

use super::stack::Stack;
use super::token::{ExtractedToken, Token};
use crate::tree::{ContainerType, Element};

/// Main function which takes the current stack and upcoming tokens to attempt to match against it.
pub fn consume<'r, 'a>(
    log: &slog::Logger,
    stack: &'r mut Stack<'_, 'a>,
    extract: &ExtractedToken<'a>,
    next: &[ExtractedToken<'a>],
) {
    let ExtractedToken { token, slice, span } = extract;

    debug!(
        log,
        "Attempting to consume tokens in different look-aheads";
        "token" => token,
        "next-len" => next.len(),
        "stack-len" => stack.len(),
    );

    match token {
        /* Plain text */
        Token::Identifier | Token::Text | Token::Whitespace => {
            stack.append(Element::Text(slice));
        }
        /* Formatting */
        Token::Bold => stack.push(ContainerType::Bold),
        Token::Italics => stack.push(ContainerType::Italics),
        Token::Underline => stack.push(ContainerType::Underline),
        Token::Superscript => stack.push(ContainerType::Superscript),
        Token::Subscript => stack.push(ContainerType::Subscript),
        /* Special formatting */
        Token::Color => try_color(),
        Token::LeftMonospace => stack.push(ContainerType::Monospace),
        Token::RightMonospace => {
            // idk, pop off monospace from the stack?
            // TODO
            todo!()
        }
        Token::Raw => try_raw(false),
        Token::LeftRaw => try_raw(true),
        Token::DoubleDash => try_strikethrough(),
        /* Blocks */
        Token::LeftTag => try_block(),
        Token::LeftTagSpecial => try_special_block(),
        /* Other (TODO) */
        _ => todo!(),
    }

    todo!()
}

fn try_strikethrough() {
    // looks for another Token::DoubleDash without spaces or whatever, otherwise just an em dash
    todo!()
}

fn try_color() {
    todo!()
}

fn try_raw(alternate: bool) {
    todo!()
}

fn try_block() {
    todo!()
}

fn try_special_block() {
    todo!()
}
