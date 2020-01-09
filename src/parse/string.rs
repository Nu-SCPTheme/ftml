/*
 * parse/string.rs
 *
 * ftml - Convert Wikidot code to HTML
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

use crate::{Error, Result};
use pest::Parser;
use std::borrow::Cow;

#[derive(Debug, Clone, Parser)]
#[grammar = "parse/string.pest"]
pub struct StringParser;

pub fn interp_str<'a>(text: &'a str) -> Result<Cow<'a, str>> {
    let pairs = match StringParser::parse(Rule::string, text) {
        Ok(mut pairs) => get_inner_pairs!(pairs),
        Err(err) => return Err(Error::Msg(format!("Error parsing string value: {:?}", err))),
    };

    // Trim off "s
    let last = text.len() - 1;
    let mut string = Cow::Borrowed(&text[1..last]);
    let mut escaped = 0;

    // Convert escapes, if any
    for pair in pairs {
        debug_assert_eq!(pair.as_rule(), Rule::char);

        let span = pair.as_span();
        let replace = match span.as_str() {
            r#"\""# => Some("\""),
            r"\\" => Some("\\"),
            r"\r" => Some("\r"),
            r"\n" => Some("\n"),
            r"\t" => Some("\t"),
            r"\0" => Some("\0"),
            r"\'" => Some("'"),
            _ => None,
        };

        if let Some(replace) = replace {
            let start = span.start() - escaped - 1;
            let range = start..start + 2;
            string.to_mut().replace_range(range, replace);
            escaped += 1;
        }
    }

    Ok(string)
}

#[test]
fn test_string_parse() {
    let string = interp_str(r#""hello,\nworld!\t""#);
    assert_eq!(
        string.expect("Converted string was None").as_ref(),
        "hello,\nworld!\t"
    );

    let string = interp_str(r#""\nA\tTHOUSAND\0WINDS\rCRY\nFOR \'\'\'VICTORS\'\'\'\n""#);
    assert_eq!(
        string.expect("Converted string was None").as_ref(),
        "\nA\tTHOUSAND\0WINDS\rCRY\nFOR '''VICTORS'''\n"
    );

    let string = interp_str(r#""""#);
    assert_eq!(string.expect("Converted string was None").as_ref(), "");
}
