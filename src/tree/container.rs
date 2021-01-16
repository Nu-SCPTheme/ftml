/*
 * tree/container.rs
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

//! Representation of generic syntax elements which wrap other elements.

use crate::enums::HeadingLevel;
use crate::tree::Element;
use ref_map::*;
use std::borrow::Cow;
use strum_macros::IntoStaticStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Container<'t> {
    #[serde(rename = "type")]
    ctype: ContainerType,
    elements: Vec<Element<'t>>,
}

impl<'t> Container<'t> {
    #[inline]
    pub fn new(ctype: ContainerType, elements: Vec<Element<'t>>) -> Self {
        Container { ctype, elements }
    }

    #[inline]
    pub fn ctype(&self) -> ContainerType {
        self.ctype
    }

    #[inline]
    pub fn elements(&self) -> &[Element<'t>] {
        &self.elements
    }
}

impl<'t> From<Container<'t>> for Vec<Element<'t>> {
    #[inline]
    fn from(container: Container<'t>) -> Vec<Element<'t>> {
        let Container { elements, .. } = container;

        elements
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct StyledContainer<'t> {
    #[serde(rename = "type")]
    ctype: StyledContainerType,
    elements: Vec<Element<'t>>,
    id: Option<Cow<'t, str>>,
    class: Option<Cow<'t, str>>,
    style: Option<Cow<'t, str>>,
}

impl<'t> StyledContainer<'t> {
    #[inline]
    pub fn new(
        ctype: StyledContainerType,
        elements: Vec<Element<'t>>,
        id: Option<Cow<'t, str>>,
        class: Option<Cow<'t, str>>,
        style: Option<Cow<'t, str>>,
    ) -> Self {
        StyledContainer { ctype, elements, id, class, style }
    }

    #[inline]
    pub fn ctype(&self) -> StyledContainerType {
        self.ctype
    }

    #[inline]
    pub fn elements(&self) -> &[Element<'t>] {
        &self.elements
    }

    #[inline]
    pub fn id(&self) -> Option<&str> {
        self.id.ref_map(|s| s.as_ref())
    }

    #[inline]
    pub fn class(&self) -> Option<&str> {
        self.class.ref_map(|s| s.as_ref())
    }

    #[inline]
    pub fn style(&self) -> Option<&str> {
        self.style.ref_map(|s| s.as_ref())
    }
}

impl<'t> From<StyledContainer<'t>> for Vec<Element<'t>> {
    #[inline]
    fn from(container: StyledContainer<'t>) -> Vec<Element<'t>> {
        let StyledContainer { elements, .. } = container;

        elements
    }
}

#[derive(
    Serialize, Deserialize, IntoStaticStr, Debug, Copy, Clone, Hash, PartialEq, Eq,
)]
#[serde(rename_all = "kebab-case")]
pub enum ContainerType {
    /// Paragraphs. HTML tag `<p>`.
    Paragraph,

    /// Bolded text. HTML tag `<strong>`.
    Bold,

    /// Italicized text. HTML tag `<em>`.
    Italics,

    /// Underlined text. HTML tag `<u>`
    Underline,

    /// Superscript text. HTML tag `<sup>`.
    Superscript,

    /// Subscript text. HTML tag `<sub>`.
    Subscript,

    /// Strikethrough. HTML tag `<s>`.
    Strikethrough,

    /// Monospace or teletype text. HTML tag `<tt>`.
    Monospace,

    /// Header. HTML tags `<h1>` through `<h6>`.
    Header(HeadingLevel),
}

impl ContainerType {
    #[inline]
    pub fn name(self) -> &'static str {
        self.into()
    }
}

impl slog::Value for ContainerType {
    fn serialize(
        &self,
        _: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_str(key, self.name())
    }
}

#[derive(
    Serialize, Deserialize, IntoStaticStr, Debug, Copy, Clone, Hash, PartialEq, Eq,
)]
#[serde(rename_all = "kebab-case")]
pub enum StyledContainerType {
    /// Span of text. HTML tag `<span>`.
    Span,

    /// Division of text. HTML tag `<div>`.
    Div,

    /// Marked or highlighted text. HTML tag `<mark>`.
    Mark,

    /// Inserted text. HTML tag `<ins>`.
    Insertion,

    /// Deleted text. HTML tag `<del>`.
    Deletion,
}

impl StyledContainerType {
    #[inline]
    pub fn name(self) -> &'static str {
        self.into()
    }
}

impl slog::Value for StyledContainerType {
    fn serialize(
        &self,
        _: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_str(key, self.name())
    }
}
