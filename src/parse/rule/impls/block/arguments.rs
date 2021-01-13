/*
 * parse/rule/impls/block/arguments.rs
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

use crate::parse::{parse_boolean, ParseWarning, ParseWarningKind, Parser};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use unicase::UniCase;

macro_rules! make_warn {
    ($parser:expr) => {
        $parser.make_warn(ParseWarningKind::BlockMalformedArguments)
    };
}

#[derive(Debug, Clone, Default)]
pub struct Arguments<'t> {
    inner: HashMap<UniCase<&'t str>, Cow<'t, str>>,
}

impl<'t> Arguments<'t> {
    #[inline]
    pub fn new() -> Self {
        Arguments::default()
    }

    pub fn insert(&mut self, key: &'t str, value: Cow<'t, str>) {
        let key = UniCase::ascii(key);

        self.inner.insert(key, value);
    }

    pub fn get(&mut self, key: &'t str) -> Option<Cow<'t, str>> {
        let key = UniCase::ascii(key);

        self.inner.remove(&key)
    }

    pub fn get_bool(
        &mut self,
        key: &'t str,
        parser: &Parser<'_, 't>,
    ) -> Result<Option<bool>, ParseWarning> {
        match self.get(key) {
            Some(argument) => match parse_boolean(argument) {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(make_warn!(parser)),
            },
            None => Ok(None),
        }
    }

    pub fn get_value<T: FromStr>(
        &mut self,
        key: &'t str,
        parser: &Parser<'_, 't>,
    ) -> Result<Option<T>, ParseWarning> {
        match self.get(key) {
            Some(argument) => match argument.parse() {
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(make_warn!(parser)),
            },
            None => Ok(None),
        }
    }
}
