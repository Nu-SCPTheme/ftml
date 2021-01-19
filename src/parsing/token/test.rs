/*
 * parsing/token/test.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
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

use super::*;

#[test]
fn tokens() {
    let log = crate::build_logger();

    macro_rules! test {
        ($input:expr, $expected:expr,) => {
            test!($input, $expected)
        };
        ($input:expr, $expected:expr) => {{
            info!(&log, "Testing tokens!"; "input" => $input);

            let expected: Vec<ExtractedToken> = $expected;
            let result = {
                let tokenization = crate::tokenize(&log, $input);
                let mut tokens: Vec<ExtractedToken> = tokenization.into();

                let last = tokens.pop().expect("No final element in resultant tokens");

                assert_eq!(last.token, Token::InputEnd, "Final token wasn't Token::InputEnd");
                assert_eq!(last.slice, "", "Final slice wasn't an empty string");

                tokens
            };

            // Manually implement "assert_eq!" here so we can use full, {:#?} formatting

            if result != expected {
                panic!(
                    "Extracted tokens from lexer do not match expected!\n\nExpected: {:#?}\nActual: {:#?}",
                    result,
                    expected,
                );
            }
        }};
    }

    // Test cases:

    test!("", vec![]);

    test!(
        "text",
        vec![ExtractedToken {
            token: Token::Identifier,
            slice: "text",
            span: 0..4,
        }],
    );

    test!(
        "-- doubleDash",
        vec![
            ExtractedToken {
                token: Token::DoubleDash,
                slice: "--",
                span: 0..2,
            },
            ExtractedToken {
                token: Token::Whitespace,
                slice: " ",
                span: 2..3,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "doubleDash",
                span: 3..13,
            },
        ],
    );

    test!(
        "--doubleDash",
        vec![
            ExtractedToken {
                token: Token::DoubleDash,
                slice: "--",
                span: 0..2,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "doubleDash",
                span: 2..12,
            },
        ],
    );

    test!(
        "__[[*user }}",
        vec![
            ExtractedToken {
                token: Token::Underline,
                slice: "__",
                span: 0..2,
            },
            ExtractedToken {
                token: Token::LeftBlockSpecial,
                slice: "[[*",
                span: 2..5,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "user",
                span: 5..9,
            },
            ExtractedToken {
                token: Token::Whitespace,
                slice: " ",
                span: 9..10,
            },
            ExtractedToken {
                token: Token::RightMonospace,
                slice: "}}",
                span: 10..12,
            },
        ],
    );

    test!(
        r#"[[> unsure = "malformed \string"#,
        vec![
            ExtractedToken {
                token: Token::LeftBlock,
                slice: "[[",
                span: 0..2,
            },
            ExtractedToken {
                token: Token::Quote,
                slice: "> ",
                span: 2..4,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "unsure",
                span: 4..10,
            },
            ExtractedToken {
                token: Token::Whitespace,
                slice: " ",
                span: 10..11,
            },
            ExtractedToken {
                token: Token::Equals,
                slice: "=",
                span: 11..12,
            },
            ExtractedToken {
                token: Token::Whitespace,
                slice: " ",
                span: 12..13,
            },
            ExtractedToken {
                token: Token::Other,
                slice: "\"",
                span: 13..14,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "malformed",
                span: 14..23,
            },
            ExtractedToken {
                token: Token::Whitespace,
                slice: " ",
                span: 23..24,
            },
            ExtractedToken {
                token: Token::Other,
                slice: "\\",
                span: 24..25,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "string",
                span: 25..31,
            },
        ],
    );

    test!(
        "[[[[quadLinkTest]]]]",
        vec![
            ExtractedToken {
                token: Token::LeftBracket,
                slice: "[",
                span: 0..1,
            },
            ExtractedToken {
                token: Token::LeftLink,
                slice: "[[[",
                span: 1..4,
            },
            ExtractedToken {
                token: Token::Identifier,
                slice: "quadLinkTest",
                span: 4..16,
            },
            ExtractedToken {
                token: Token::RightLink,
                slice: "]]]",
                span: 16..19,
            },
            ExtractedToken {
                token: Token::RightBracket,
                slice: "]",
                span: 19..20,
            },
        ],
    );
}
