/*
 * parse/token/mod.rs
 *
 * ftml - Library to parse Wikidot text
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

#[cfg(test)]
mod test;

use crate::span_wrap::SpanWrap;
use pest::iterators::Pair;
use pest::Parser;
use std::ops::Range;
use strum_macros::IntoStaticStr;

#[derive(Parser, Debug)]
#[grammar = "parse/lexer.pest"]
struct TokenLexer;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ExtractedToken<'a> {
    pub token: Token,
    pub slice: &'a str,
    pub span: Range<usize>,
}

#[derive(
    Serialize, Deserialize, Enum, IntoStaticStr, Debug, Copy, Clone, PartialEq, Eq,
)]
#[serde(rename_all = "kebab-case")]
pub enum Token {
    //
    // Symbols
    //
    LeftBracket,
    LeftBracketAnchor,
    LeftBracketSpecial,
    RightBracket,
    LeftBlock,
    LeftBlockEnd,
    LeftBlockSpecial,
    RightBlock,
    LeftAnchor,
    DoubleDash,
    TripleDash,
    ClearFloatNeutral,
    ClearFloatCenter,
    ClearFloatLeft,
    ClearFloatRight,
    Pipe,
    Equals,
    Underscore,
    Quote,
    Heading,

    //
    // Whitespace
    //
    LineBreak,
    ParagraphBreak,
    Whitespace,

    //
    // Formatting
    //
    Bold,
    Italics,
    Underline,
    Superscript,
    Subscript,
    LeftMonospace,
    RightMonospace,
    Color,
    Raw,
    LeftRaw,
    RightRaw,

    //
    // Links
    //
    LeftLink,
    LeftLinkSpecial,
    RightLink,

    //
    // Tables
    //
    TableColumn,
    TableColumnTitle,

    //
    // Alignment
    //
    RightAlignOpen,
    RightAlignClose,
    LeftAlignOpen,
    LeftAlignClose,
    CenterAlignOpen,
    CenterAlignClose,
    JustifyAlignOpen,
    JustifyAlignClose,

    //
    // Text components
    //
    Identifier,
    Email,
    Url,
    String,

    //
    // Miscellaneous
    //
    LeftComment,
    RightComment,
    InputEnd,

    //
    // Catch-all case
    //
    Other,
}

impl Token {
    pub(crate) fn extract_all<'a>(
        log: &slog::Logger,
        text: &'a str,
    ) -> Vec<ExtractedToken<'a>> {
        debug!(log, "Running lexer on input");

        match TokenLexer::parse(Rule::document, text) {
            Ok(pairs) => {
                info!(log, "Lexer produced pairs for processing");

                pairs.map(|pair| Token::convert_pair(log, pair)).collect()
            }
            Err(error) => {
                // Return all of the input as one big raw text
                // and log this as an error, since it shouldn't be happening

                error!(log, "Error while lexing input in pest: {}", error);

                vec![ExtractedToken {
                    token: Token::Other,
                    slice: text,
                    span: 0..text.len(),
                }]
            }
        }
    }

    /// Converts a single `Pair` from pest into its corresponding `ExtractedToken`.
    fn convert_pair<'a>(log: &slog::Logger, pair: Pair<'a, Rule>) -> ExtractedToken<'a> {
        // Extract values from the Pair
        let rule = pair.as_rule();
        let slice = pair.as_str();
        let start = pair.as_span().start();
        let end = pair.as_span().end();
        let span = start..end;

        // Get matching Token.
        let token = Token::get_from_rule(rule);

        debug!(
            log,
            "Converting pair '{:?}' into token", rule;
            "token" => token.name(),
            "slice" => pair.as_str(),
            "span" => SpanWrap::from(&span),
        );

        ExtractedToken { token, slice, span }
    }

    /// Mapping of a pest `Rule` to its corresponding `Token` enum.
    fn get_from_rule(rule: Rule) -> Token {
        match rule {
            // Symbols
            Rule::left_comment => Token::LeftComment,
            Rule::right_comment => Token::RightComment,
            Rule::left_bracket => Token::LeftBracket,
            Rule::left_bracket_anchor => Token::LeftBracketAnchor,
            Rule::left_bracket_special => Token::LeftBracketSpecial,
            Rule::right_bracket => Token::RightBracket,
            Rule::left_block => Token::LeftBlock,
            Rule::left_block_end => Token::LeftBlockEnd,
            Rule::left_block_special => Token::LeftBlockSpecial,
            Rule::right_block => Token::RightBlock,
            Rule::color => Token::Color,
            Rule::double_dash => Token::DoubleDash,
            Rule::triple_dash => Token::TripleDash,
            Rule::clear_float => Token::ClearFloatNeutral,
            Rule::clear_float_center => Token::ClearFloatCenter,
            Rule::clear_float_left => Token::ClearFloatLeft,
            Rule::clear_float_right => Token::ClearFloatRight,
            Rule::pipe => Token::Pipe,
            Rule::underscore => Token::Underscore,
            Rule::equals => Token::Equals,
            Rule::quote => Token::Quote,
            Rule::heading => Token::Heading,

            // Whitespace
            Rule::line_break => Token::LineBreak,
            Rule::paragraph_break => Token::ParagraphBreak,
            Rule::space => Token::Whitespace,

            // Formatting
            Rule::bold => Token::Bold,
            Rule::italics => Token::Italics,
            Rule::underline => Token::Underline,
            Rule::superscript => Token::Superscript,
            Rule::subscript => Token::Subscript,
            Rule::left_monospace => Token::LeftMonospace,
            Rule::right_monospace => Token::RightMonospace,
            Rule::raw => Token::Raw,
            Rule::left_raw => Token::LeftRaw,
            Rule::right_raw => Token::RightRaw,

            // Links
            Rule::left_link => Token::LeftLink,
            Rule::left_link_special => Token::LeftLinkSpecial,
            Rule::right_link => Token::RightLink,

            // Tables
            Rule::table_column => Token::TableColumn,
            Rule::table_column_title => Token::TableColumnTitle,

            // Alignment
            Rule::open_right_align => Token::RightAlignOpen,
            Rule::open_left_align => Token::LeftAlignOpen,
            Rule::open_center_align => Token::CenterAlignOpen,
            Rule::open_justify_align => Token::JustifyAlignOpen,
            Rule::close_right_align => Token::RightAlignClose,
            Rule::close_left_align => Token::LeftAlignClose,
            Rule::close_center_align => Token::CenterAlignClose,
            Rule::close_justify_align => Token::JustifyAlignClose,

            // Text components
            Rule::identifier => Token::Identifier,
            Rule::email => Token::Email,
            Rule::url => Token::Url,
            Rule::string => Token::String,

            // Other
            Rule::other => Token::Other,
            Rule::EOI => Token::InputEnd,

            // Invalid
            Rule::char | Rule::document | Rule::token => {
                panic!("Received invalid pest rule: {:?}", rule)
            }
        }
    }

    #[inline]
    pub fn name(self) -> &'static str {
        self.into()
    }
}

impl slog::Value for Token {
    fn serialize(
        &self,
        _: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_str(key, self.name())
    }
}
