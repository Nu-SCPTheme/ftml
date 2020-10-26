/*
 * parse/tree/mod.rs
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

// FIXME to prevent compile spam
#![allow(dead_code)]

// Convenience macro for static regular expressions meant for parsing.
// Retrieves the capture group with the given name and returns as a string.
macro_rules! capture {
    ($capture:expr, $name:expr) => {
        $capture
            .name($name)
            .expect("String from parser didn't match regular expression")
            .as_str()
    };
}

mod misc;
mod object;
mod paragraph;
mod word;

mod prelude {
    lazy_static! {
        pub static ref ARGUMENT_NAME: Regex = Regex::new(r"\s*(?P<name>\w+)\s*=\s*").unwrap();
    }

    pub use super::super::string::interp_str;
    pub use super::super::{Rule, WikidotParser};
    pub use super::convert_internal_paragraphs;
    pub use super::{Paragraph, Tab, TableRow, Word};
    pub use pest::iterators::{Pair, Pairs};
    pub use regex::{Regex, RegexBuilder};
}

pub use self::misc::{Tab, TableRow};
pub use self::object::SyntaxTree;
pub use self::paragraph::convert_internal_paragraphs;
pub use self::paragraph::Paragraph;
pub use self::word::{ImageArguments, Word};
