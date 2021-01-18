/*
 * include/object.rs
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

use ref_map::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct PageRef<'t> {
    site: Option<Cow<'t, str>>,
    page: Cow<'t, str>,
}

impl<'t> PageRef<'t> {
    #[inline]
    pub fn page_and_site<S>(site: Option<S>, page: S) -> Self
    where
        S: Into<Cow<'t, str>>,
    {
        let site = site.map(Into::into);
        let page = page.into();

        PageRef { site, page }
    }

    #[inline]
    pub fn page_only<S>(page: S) -> Self
    where
        S: Into<Cow<'t, str>>,
    {
        Self::page_and_site(None, page)
    }

    #[inline]
    pub fn site(&self) -> Option<&str> {
        self.site.ref_map(|s| s.as_ref())
    }

    #[inline]
    pub fn page(&self) -> &str {
        self.page.as_ref()
    }
}
// TODO add parse method

impl<'t> Display for PageRef<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(site) = self.site() {
            write!(f, ":{}:", &site)?;
        }

        write!(f, "{}", &self.page)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct IncludeRef<'t> {
    page: PageRef<'t>,
    variables: HashMap<Cow<'t, str>, Cow<'t, str>>,
}

impl<'t> IncludeRef<'t> {
    #[inline]
    pub fn page_with_args(
        page: PageRef<'t>,
        variables: HashMap<Cow<'t, str>, Cow<'t, str>>,
    ) -> Self {
        IncludeRef { page, variables }
    }

    #[inline]
    pub fn page_only(page: PageRef<'t>) -> Self {
        Self::page_with_args(page, HashMap::new())
    }
}
