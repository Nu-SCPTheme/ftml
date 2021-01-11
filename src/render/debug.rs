/*
 * render/debug.rs
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

//! A simple renderer that outputs the `SyntaxTree` using Rust's debug formatter.

use super::prelude::*;

#[derive(Debug)]
pub struct DebugRender;

impl Render for DebugRender {
    type Output = String;

    #[inline]
    fn render(&self, tree: &SyntaxTree) -> String {
        format!("{:#?}", tree)
    }
}

#[test]
fn debug() {
    // Expected outputs
    const OUTPUT: &str = r#"SyntaxTree {
    elements: [
        Text(
            "apple",
        ),
        Text(
            " ",
        ),
        Container(
            Container {
                ctype: Bold,
                elements: [
                    Text(
                        "banana",
                    ),
                ],
            },
        ),
    ],
    styles: [
        "span.hidden-text { display: none; }",
    ],
}"#;

    // Syntax tree construction
    let elements = vec![
        text!("apple"),
        text!(" "),
        Element::Container(Container::new(ContainerType::Bold, vec![text!("banana")])),
    ];
    let warnings = vec![];
    let styles = vec![cow!("span.hidden-text { display: none; }")];

    let result = SyntaxTree::from_element_result(elements, warnings, styles);
    let (tree, _) = result.into();

    // Perform rendering
    let output = DebugRender.render(&tree);
    assert_eq!(
        output, OUTPUT,
        "Pretty JSON syntax tree output doesn't match",
    );
}
