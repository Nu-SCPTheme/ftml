/*
 * tree/object.rs
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

use super::Element;
use crate::{ParseError, ParseResult};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct SyntaxTree<'t> {
    /// The list of elements that compose this tree.
    ///
    /// Note that each `Element<'t>` can contain other elements within it,
    /// and these as well, etc. This structure composes the depth of the
    /// syntax tree.
    pub elements: Vec<Element<'t>>,

    /// The list of CSS styles added in this page, in order.
    ///
    /// How the renderer decides to consume these is up to the implementation,
    /// however the recommendation is to combine them all into one large style
    /// rule list.
    pub styles: Vec<Cow<'t, str>>,
}

impl<'t> SyntaxTree<'t> {
    pub(crate) fn from_element_result(
        mut elements: Vec<Element<'t>>,
        errors: Vec<ParseError>,
        styles: Vec<Cow<'t, str>>,
    ) -> ParseResult<Self> {
        // Remove trailing null element
        // This is added because Token::InputEnd is converted into this.
        {
            let last = elements.pop();

            assert_eq!(last, Some(Element::Null), "Last element wasn't null!");
        }

        // Create final SyntaxTree result
        let tree = SyntaxTree { elements, styles };
        ParseResult::new(tree, errors)
    }
}
