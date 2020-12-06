/*
 * parse/test.rs
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

use crate::parse::{ParseError, ParseErrorKind, Token};
use crate::tree::{Container, ContainerType, Element, SyntaxTree};

#[test]
fn ast() {
    let log = crate::build_logger();

    macro_rules! test {
        ($text:expr, $elements:expr, $errors:expr,) => {
            test!($text, $elements, $errors)
        };

        ($text:expr, $elements:expr, $errors:expr) => {{
            let text = $text;
            let expected_elements = $elements;
            let expected_errors = $errors;

            println!("Testing parsing! input: {:?}", text);
            println!("Expected elements: {:#?}", expected_elements);
            println!("Expected errors: {:#?}", expected_errors);

            info!(&log, "Testing AST parsing!"; "text" => text);

            let tokens = crate::tokenize(&log, text);
            let result = crate::parse(&log, &tokens);
            let (tree, errors) = result.into();
            let SyntaxTree { elements } = tree;

            println!("Actual elements: {:#?}", elements);
            println!("Actual errors: {:#?}", errors);

            assert_eq!(
                elements,
                expected_elements,
                "Resultant elements (left) did not match expected (right)",
            );

            assert_eq!(
                errors,
                expected_errors,
                "Resultant error list (left) did not match expected (right)",
            );
        }};
    }

    macro_rules! container {
        // For plain enum container types
        ($type:tt, $elements:expr) => {
            container!(ContainerType::$type; $elements)
        };

        // For container types with added data
        ($type:expr; $elements:expr) => {
            Element::Container(Container::new($type, $elements))
        };

        // Comma variants
        ($type:tt, $elements:expr,) => {
            container!($type, $elements)
        };

        ($type:expr; $elements:expr,) => {
            container!($type; $elements)
        };
    }

    test!("", vec![], vec![]);

    test!(" ", vec![Element::Text(" ")], vec![]);

    test!("abc", vec![Element::Text("abc")], vec![]);

    test!("\n", vec![Element::LineBreak], vec![]);

    test!(
        "**bold** text",
        vec![
            container!(Bold, vec![Element::Text("bold")]),
            Element::Text(" "),
            Element::Text("text"),
        ],
        vec![],
    );

    test!(
        "**fail bold",
        vec![
            Element::Text("**"),
            Element::Text("fail"),
            Element::Text(" "),
            Element::Text("bold"),
        ],
        vec![ParseError::new_raw(
            Token::Bold,
            "fallback",
            0..2,
            ParseErrorKind::NoRulesMatch,
        )],
    );

    test!(
        "//italics// text",
        vec![
            container!(Italics, vec![Element::Text("italics")]),
            Element::Text(" "),
            Element::Text("text"),
        ],
        vec![],
    );

    test!(
        "//fail italics",
        vec![
            Element::Text("//"),
            Element::Text("fail"),
            Element::Text(" "),
            Element::Text("italics"),
        ],
        vec![ParseError::new_raw(
            Token::Italics,
            "fallback",
            0..2,
            ParseErrorKind::NoRulesMatch,
        )],
    );

    test!(
        "__underline__ text",
        vec![
            container!(Underline, vec![Element::Text("underline")]),
            Element::Text(" "),
            Element::Text("text"),
        ],
        vec![],
    );

    test!(
        "__fail underline",
        vec![
            Element::Text("__"),
            Element::Text("fail"),
            Element::Text(" "),
            Element::Text("underline"),
        ],
        vec![ParseError::new_raw(
            Token::Underline,
            "fallback",
            0..2,
            ParseErrorKind::NoRulesMatch,
        )],
    );

    test!(
        "single [!-- stuff here --] comment",
        vec![
            Element::Text("single"),
            Element::Text(" "),
            Element::Null,
            Element::Text(" "),
            Element::Text("comment"),
        ],
        vec![],
    );

    test!(
        "multiline\n[!-- stuff \n here --]\n comment",
        vec![
            Element::Text("multiline"),
            Element::LineBreak,
            Element::Null,
            Element::LineBreak,
            Element::Text(" "),
            Element::Text("comment"),
        ],
        vec![],
    );

    test!(
        "fail [!-- comment",
        vec![
            Element::Text("fail"),
            Element::Text(" "),
            Element::Text("[!--"),
            Element::Text(" "),
            Element::Text("comment"),
        ],
        vec![ParseError::new_raw(
            Token::LeftComment,
            "fallback",
            5..9,
            ParseErrorKind::NoRulesMatch,
        )],
    );

    test!(
        "fail --] comment",
        vec![
            Element::Text("fail"),
            Element::Text(" "),
            Element::Text("--]"),
            Element::Text(" "),
            Element::Text("comment"),
        ],
        vec![ParseError::new_raw(
            Token::RightComment,
            "fallback",
            5..8,
            ParseErrorKind::NoRulesMatch,
        )],
    );

    test!("@@@@", vec![Element::Raw("")], vec![]);

    test!("@@@@@", vec![Element::Raw("@")], vec![]);

    test!("@@@@@@", vec![Element::Raw("@@")], vec![]);

    test!(
        "test @@@@ string",
        vec![
            Element::Text("test"),
            Element::Text(" "),
            Element::Raw(""),
            Element::Text(" "),
            Element::Text("string"),
        ],
        vec![],
    );

    test!(
        "test @@@@@@ string",
        vec![
            Element::Text("test"),
            Element::Text(" "),
            Element::Raw("@@"),
            Element::Text(" "),
            Element::Text("string"),
        ],
        vec![],
    );

    test!("@<>@", vec![Element::Raw("")], vec![],);

    test!(
        "@@raw @< >@ content@@",
        vec![Element::Raw("raw @< >@ content")],
        vec![],
    );

    test!(
        "not @@**@@ bold",
        vec![
            Element::Text("not"),
            Element::Text(" "),
            Element::Raw("**",),
            Element::Text(" "),
            Element::Text("bold"),
        ],
        vec![],
    );

    test!(
        "@<raw @@ content>@",
        vec![Element::Raw("raw @@ content")],
        vec![],
    );

    test!(
        "interrupted @@\n@@",
        vec![
            Element::Text("interrupted"),
            Element::Text(" "),
            Element::Text("@@"),
            Element::LineBreak,
            Element::Text("@@"),
        ],
        vec![
            // From interrupted raw
            ParseError::new_raw(
                Token::Raw, //
                "fallback",
                12..14,
                ParseErrorKind::NoRulesMatch,
            ),
            // Trying the ending raw as an opener
            ParseError::new_raw(
                Token::Raw, //
                "fallback",
                15..17,
                ParseErrorKind::NoRulesMatch,
            ),
        ],
    );

    test!(
        "interrupted @<\n>@",
        vec![
            Element::Text("interrupted"),
            Element::Text(" "),
            Element::Text("@<"),
            Element::LineBreak,
            Element::Text(">@"),
        ],
        vec![
            // From interrupted raw
            ParseError::new_raw(
                Token::LeftRaw,
                "fallback",
                12..14,
                ParseErrorKind::NoRulesMatch,
            ),
            // Trying the ending raw as an opener
            ParseError::new_raw(
                Token::RightRaw,
                "fallback",
                15..17,
                ParseErrorKind::NoRulesMatch,
            ),
        ],
    );

    test!(
        "##blue|text here##",
        vec![container!(
            ContainerType::Color("blue");
            vec![
                Element::Text("text"),
                Element::Text(" "),
                Element::Text("here"),
            ],
        )],
        vec![],
    );

    test!(
        "###ccc|css color!##",
        vec![container!(
            ContainerType::Color("#ccc");
            vec![
                Element::Text("css"),
                Element::Text(" "),
                Element::Text("color"),
                Element::Text("!"),
            ],
        )],
        vec![],
    );

    test!(
        "##not color",
        vec![
            Element::Text("##"),
            Element::Text("not"),
            Element::Text(" "),
            Element::Text("color"),
        ],
        vec![ParseError::new_raw(
            Token::Color,
            "fallback",
            0..2,
            ParseErrorKind::NoRulesMatch,
        )],
    );

    test!(
        "##invalid\n|text##",
        vec![
            Element::Text("##"),
            Element::Text("invalid"),
            Element::LineBreak,
            Element::Text("|"),
            Element::Text("text"),
            Element::Text("##"),
        ],
        vec![
            ParseError::new_raw(
                Token::Color, //
                "fallback",
                0..2,
                ParseErrorKind::NoRulesMatch,
            ),
            ParseError::new_raw(
                Token::Color,
                "fallback",
                15..17,
                ParseErrorKind::NoRulesMatch,
            ),
        ],
    );
}

#[test]
fn json() {
    let log = crate::build_logger();
    let text = "**apple //banana//** cherry";
    let tokens = crate::tokenize(&log, text);
    let result = crate::parse(&log, &tokens);
    println!("{:#?}", result.value());
    println!("Errors: {:#?}", result.errors());

    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("JSON:\n{}", json);
}
