/*
 * render/mod.rs
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

pub mod html;
mod null;
mod tree;

pub use self::html::HtmlRender;
pub use self::null::NullRender;
pub use self::tree::TreeRender;
pub use crate::{PageInfo, PageInfoOwned};

use crate::{parse, prefilter};
use crate::{RemoteHandle, Result, SyntaxTree};

pub trait Render {
    type Output;

    fn render(&self, tree: &SyntaxTree, info: PageInfo) -> Result<Self::Output>;

    fn transform(
        &self,
        text: &mut String,
        info: PageInfo,
        handle: &dyn RemoteHandle,
    ) -> Result<Self::Output> {
        prefilter(text, handle)?;
        let tree = parse(text)?;
        let output = self.render(&tree, info)?;
        Ok(output)
    }
}
