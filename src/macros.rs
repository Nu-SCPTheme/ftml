/*
 * macros.rs
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

/// Unwrap the current pair object into its component pairs.
///
/// This pattern is used in cases where a Pairs object wraps a
/// single Pair object. For instance the Page rule only has one
/// possibility, and the outer layer is not useful for parsing,
/// so we discard it.
macro_rules! get_inner_pairs {
    ($pairs:expr) => {
        $pairs.next().expect("Item has no more pairs").into_inner()
    };
}

/// Retrieve the first pair object inside this pair.
///
/// This pattern is used to convert the first Pair object within
/// a Pair. Thus, it retrieves its inner Pairs iterator and then
/// asserts the first item exists.
macro_rules! get_first_pair {
    ($pair:expr) => {
        $pair.into_inner().next().expect("Inner pairs is empty")
    };
}

macro_rules! get_nth_pair {
    ($pair:expr, $index:expr) => {
        $pair
            .clone()
            .into_inner()
            .nth($index)
            .expect("Couldn't find nth inner pair")
    };
}
