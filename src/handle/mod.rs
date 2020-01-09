/*
 * handle/mod.rs
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

mod null;
mod test;

mod prelude {
    pub use super::RemoteHandle;
    pub use crate::data::User;
    pub use crate::{RemoteError, RemoteResult};
    pub use std::borrow::Cow;
    pub use std::collections::HashMap;
}

use self::prelude::*;

pub use self::null::NullHandle;
pub use self::test::TestHandle;

pub trait RemoteHandle {
    fn get_user_by_name(&self, name: &str) -> RemoteResult<Option<User>>;
    fn get_user_by_id(&self, id: u64) -> RemoteResult<Option<User>>;

    fn get_page(
        &self,
        name: &str,
        args: &HashMap<&str, &str>,
    ) -> RemoteResult<Option<Cow<'static, str>>>;
}
