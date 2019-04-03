/*
 * parse/rules/footnote.rs
 *
 * wikidot-html - Convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith for Project Foundation
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

//! Processing rules for footnotes and the footnote block.

use crate::{ParseState, Result, Token};
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref FOOTNOTE: Regex = {
        RegexBuilder::new(r"\s*\[\[footnote\]\](?P<contents>.*?)\[\[/footnote\]\]")
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    };

    static ref FOOTNOTE_BLOCK: Regex = {
        RegexBuilder::new(r"\[\[footnoteblock\]\]")
            .build()
            .unwrap()
    };
}

pub fn rule_footnote(state: &mut ParseState) -> Result<()> {
    while let Some(capture) = FOOTNOTE.captures(state.text()) {
        let contents = capture["contents"].to_string();
        let token = Token::Footnote { contents };
        state.push_token(token, &*FOOTNOTE);
    }

    while let Some(_) = FOOTNOTE_BLOCK.find(state.text()) {
        let token = Token::FootnoteBlock;
        state.push_token(token, &*FOOTNOTE_BLOCK);
    }

    Ok(())
}

#[test]
fn test_footnote() {
    let mut state = ParseState::new("And then they died!  [[footnote]]They did not actually die.[[/footnote]]".into());
    rule_footnote(&mut state).unwrap();
    assert_eq!(state.text(), "And then they died!\00\0");
    assert_eq!(state.tokens().len(), 1);

    let mut state = ParseState::new("[[footnote]]banana[[/footnote]] [[footnoteblock]]".into());
    rule_footnote(&mut state).unwrap();
    assert_eq!(state.text(), "\00\0 \01\0");
    assert_eq!(state.tokens().len(), 2);
}
