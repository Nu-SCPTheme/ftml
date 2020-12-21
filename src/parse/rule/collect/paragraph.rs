/*
 * parse/rule/collect/paragraph.rs
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

use super::prelude::*;
use crate::parse::ParseStack;

pub fn try_paragraph<'t, 'r>(
    log: &slog::Logger,
    (extracted, remaining, full_text): (
        &'r ExtractedToken<'t>,
        &'r [ExtractedToken<'t>],
        FullText<'t>,
    ),
    rule: Rule,
    close_tokens: &[Token],
    invalid_tokens: &[Token],
    invalid_token_pairs: &[(Token, Token)],
) -> Consumption<'t, 'r> {
    // Log try_paragraph() call
    info!(
        log,
        "Trying to consume tokens to produce paragraph for {:?}", rule,
    );

    // Iterate and consume the tokens into multiple elements
    let mut stack = ParseStack::new();

    let consumption = try_collect(
        log,
        (extracted, remaining, full_text),
        rule,
        close_tokens,
        invalid_tokens,
        invalid_token_pairs,
        |log, extracted, remaining, _| {
            todo!();

            // We are collecting everything in ParseStack,
            // so we return unit consumption so the gathered Vec<_>
            // doesn't actually allocate, but we can still output
            // something success as required for try_collect().
            GenericConsumption::ok((), remaining)
        },
    );

    todo!()
}
