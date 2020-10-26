/*
 * lib.rs
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

#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]

//! A library to convert Wikidot text source into HTML.
//!
//! This library aims to be a replacement of Wikidot's Text_Wiki
//! module, but with the goals of providing more modular integration
//! and standalone servicing.
//!
//! While backwards compatibility with Wikidot code is one of the aims
//! of this library, there are constructions which are valid in Wikidot
//! but deliberately invalid in ftml. The total scope of all Wikidot code
//! that is valid would almost require a parser nearly identical to the one
//! attempting to be rewritten to cover every edge case, even if supporting
//! such a case is not very useful or sensible.
//!
//! For instance, the following is valid code:
//! ```text
//! > [[div class="test"]
//! > A man, a plan, a canal, Panama.
//! [[/div]]
//! ```
//!
//! However the actual extent of the blockquote intersects with the div, and
//! it essentially is the HTML equivalent of
//! ```text
//! <div class="outer">
//!   <p class="inner">
//!   </div>
//! </p>
//! ```
//!
//! Which is obviously invalid syntax, and can cause issues.
//!
//! Instead the library's parser defines a grammar, which is designed to be
//! compatible with all common Wikidot constructions, or has extensions for
//! situations that are not directly supported. This largely-overlapping but
//! slightly dissimilar specification ("ftml code") aims at being able to
//! _effectively_ replace Wikidot code with minor human involvement to
//! replace malformed original sources.
//!
//! This crate also provides an executable to convert files from
//! the command-line. See that file for usage documentation.

extern crate chrono;
extern crate either;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate percent_encoding;
extern crate pest;

#[macro_use]
extern crate pest_derive;
extern crate regex;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_repr;

#[macro_use]
extern crate str_macro;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate tinyvec;

#[cfg(test)]
extern crate serde_json;

#[macro_use]
mod macros;

pub mod data;
mod enums;
mod error;
pub mod handle;
mod info;
mod parse;
mod preprocess;

pub use self::error::{Error, RemoteError};
pub use self::handle::Handle;
pub use self::info::{PageInfo, PageInfoOwned};
pub use self::parse::{parse, ImageArguments, Paragraph, SyntaxTree, Word};
pub use self::preprocess::{prefilter, preprocess};

pub mod prelude {
    pub use super::{data, handle, parse, prefilter};
    pub use super::{Error, PageInfo, PageInfoOwned, Result, StdResult, SyntaxTree};
}

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;
pub type RemoteResult<T> = StdResult<T, RemoteError>;
