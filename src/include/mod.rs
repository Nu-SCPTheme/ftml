/*
 * include/mod.rs
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

mod includer;
mod object;

pub use self::includer::{Includer, NullIncluder};
pub use self::object::{IncludeRef, PageRef};

use crate::span_wrap::SpanWrap;
use pest::Parser;
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref INCLUDE_REGEX: Regex = {
        RegexBuilder::new(r"\[\[\s*include\s*.+\]\]")
            .case_insensitive(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    };
}

#[derive(Parser, Debug)]
#[grammar = "include/grammar.pest"]
struct IncludeParser;

pub fn include<'t, I, E>(
    log: &slog::Logger,
    text: &'t mut String,
    mut includer: I,
) -> Result<Vec<PageRef<'t>>, E>
    where
        I: Includer<'t, Error = E>,
{
    let log = &log.new(slog_o!(
        "filename" => slog_filename!(),
        "lineno" => slog_lineno!(),
        "function" => "include",
        "text" => str!(text),
    ));

    info!(
        log,
        "Finding and replacing all instances of include blocks in text"
    );

    let mut ranges = Vec::new();
    let mut pages = Vec::new();
    let mut includes = Vec::new();

    // Get include references
    for mtch in INCLUDE_REGEX.find_iter(text) {
        let start = mtch.start();
        let end = mtch.end();
        let slice = &text[start..end];

        match IncludeParser::parse(Rule::include, slice) {
            Ok(pairs) => {
                debug!(
                    log,
                    "Parsed include block";
                    "span" => SpanWrap::from(start..end),
                    "slice" => slice,
                );

                for pair in pairs {
                    // TODO
                    println!("rule: {:?}, slice: {:?}", pair.as_rule(), pair.as_str());
                }

                ranges.push(start..end);
                pages.push(page_ref);
                includes.push(include_ref);
            }
            Err(error) => {
                debug!(
                    log,
                    "Found invalid include block";
                    "error" => str!(error),
                    "span" => SpanWrap::from(start..end),
                    "slice" => slice,
                );
            }
        }
    }

    // Retrieve included pages
    let fetched_pages = includer.include_pages(&includes)?;

    // Substitute inclusions
    //
    // We must iterate backwards for all the indices to be valid

    let ranges_iter = ranges.iter();
    let pages_iter = pages.iter();

    for (range, page_ref) in ranges_iter.zip(pages_iter).rev() {
        debug!(
            log,
            "Replacing range for included page";
            "span" => SpanWrap::from(range),
            "site" => page_ref.site(),
            "page" => page_ref.page(),
        );

        // Get replaced content, or error message
        let replace_with = match fetched_pages.get(page_ref) {
            Some(text) => text,
            None => includer.no_such_include(page_ref),
        };

        // Perform the substitution
        text.replace_range(range, replace_with);
    }

    // Return
    Ok(pages)
}

#[test]
fn test_include() {
    let log = crate::build_logger();

    macro_rules! test {
        ($text:expr, $expected:expr) => {{
            let mut text = str!($text);
            let result = include(&log, &mut text, NullIncluder);
            let actual = result.expect("Fetching pages failed");
            let expected = $expected;

            println!("Input: {:?}", $text);
            println!("Pages (actual): {:?}", actual);
            println!("Pages (expected): {:?}", expected);
            println!();

            assert_eq!(
                &actual, &expected,
                "Actual pages to include doesn't match expected"
            );
        }};
    }

    test!("", vec![]);
    test!("[[include page]]", vec![PageRef::page_only("page")]);

    test!(
        "abc\n[[include page]]\ndef\n[[include page2\narg=1]]\nghi",
        vec![]
    );
}
