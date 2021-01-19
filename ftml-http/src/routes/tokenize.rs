/*
 * routes/tokenize.rs
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

use super::prelude::*;
use ftml::ExtractedToken;

#[derive(Serialize, Debug)]
struct TokenizeOutput<'a> {
    tokens: Vec<ExtractedToken<'a>>,
    text: &'a str,
    pages_included: Vec<PageRef<'a>>,
}

pub fn route_tokenize(
    log: slog::Logger,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("tokenize"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .map(move |input| {
            let (mut text, pages_included) =
                try_response!(run_include(&log, input)).into();

            ftml::preprocess(&log, &mut text);

            let tokens = ftml::tokenize(&log, &text).into_tokens();
            let resp = Response::ok(TokenizeOutput {
                tokens,
                text: &text,
                pages_included,
            });

            warp::reply::json(&resp)
        })
}
