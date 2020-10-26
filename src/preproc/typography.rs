/*
 * preproc/typography.rs
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

//! Perform Wikidot's typographical modifications.
//! For full information, see the original source file:
//! https://github.com/Nu-SCPTheme/wikidot/blob/master/lib/Text_Wiki/Text/Wiki/Parse/Default/Typography.php
//!
//! The transformations performed here are listed:
//! * `` .. '' to fancy double quotes
//! * ` .. ' to fancy single quotes
//! * ,, .. '' to fancy lowered double quotes
//! * << and >> to fancy French angle quotation marks
//! * ... to an ellipsis

use regex::Regex;

lazy_static! {
    // ‘ - LEFT SINGLE QUOTATION MARK
    // ’ - RIGHT SINGLE QUOTATION MARK
    static ref SINGLE_QUOTES: Replacer = Replacer::RegexSurround {
        regex: Regex::new(r"`(.*?)'").unwrap(),
        begin: "\u{2018}",
        end: "\u{2019}",
    };

    // “ - LEFT DOUBLE QUOTATION MARK
    // ” - RIGHT DOUBLE QUOTATION MARK
    static ref DOUBLE_QUOTES: Replacer = Replacer::RegexSurround {
        regex: Regex::new(r"``(.*?)''").unwrap(),
        begin: "\u{201c}",
        end: "\u{201d}",
    };

    // „ - DOUBLE LOW-9 QUOTATION MARK
    static ref LOW_DOUBLE_QUOTES: Replacer = Replacer::RegexSurround {
        regex: Regex::new(r",,(.*?)''").unwrap(),
        begin: "\u{201e}",
        end: "\u{201d}",
    };

    // « - LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    static ref LEFT_DOUBLE_ANGLE: Replacer = Replacer::StrReplace {
        pattern: "<<",
        replacement: "\u{0ab}",
    };

    // » - RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    static ref RIGHT_DOUBLE_ANGLE: Replacer = Replacer::StrReplace {
        pattern: ">>",
        replacement: "\u{0bb}",
    };

    // … - HORIZONTAL ELLIPSIS
    static ref ELLIPSIS: Replacer = Replacer::RegexReplace {
        regex: Regex::new(r"(?:\.\.\.|\. \. \.)").unwrap(),
        replacement: "\u{2026}",
    };
}

#[derive(Debug)]
pub enum Replacer {
    StrReplace {
        pattern: &'static str,
        replacement: &'static str,
    },
    RegexReplace {
        regex: Regex,
        replacement: &'static str,
    },
    RegexSurround {
        regex: Regex,
        begin: &'static str,
        end: &'static str,
    },
}

impl Replacer {
    fn replace(&self, log: &slog::Logger, text: &mut String, buffer: &mut String) {
        use self::Replacer::*;

        match *self {
            StrReplace {
                pattern,
                replacement,
            } => {
                trace!(
                    log,
                    "Running static string replacement";
                    "type" => "string",
                    "pattern" => pattern,
                    "replacement" => replacement,
                );

                while let Some(idx) = text.find(pattern) {
                    let range = idx..idx + pattern.len();
                    text.replace_range(range, replacement);
                }
            }
            RegexReplace {
                ref regex,
                replacement,
            } => {
                trace!(
                    log,
                    "Running regular expression replacement";
                    "type" => "regex",
                    "pattern" => regex.as_str(),
                    "replacement" => replacement,
                );

                while let Some(capture) = regex.captures(text) {
                    let mtch = capture
                        .get(0)
                        .expect("Regular expression lacks a full match");
                    let range = mtch.start()..mtch.end();

                    text.replace_range(range, replacement);
                }
            }
            RegexSurround {
                ref regex,
                begin,
                end,
            } => {
                trace!(
                    log,
                    "Running regular expression capture replacement";
                    "type" => "surround",
                    "pattern" => regex.as_str(),
                    "begin" => begin,
                    "end" => end,
                );

                while let Some(capture) = regex.captures(text) {
                    let mtch = capture
                        .get(1)
                        .expect("Regular expression lacks a content group");

                    let range = {
                        let mtch = capture
                            .get(0)
                            .expect("Regular expression lacks a full match");

                        mtch.start()..mtch.end()
                    };

                    buffer.clear();
                    buffer.push_str(begin);
                    buffer.push_str(mtch.as_str());
                    buffer.push_str(end);

                    text.replace_range(range, &buffer);
                }
            }
        }
    }
}

pub fn substitute(log: &slog::Logger, text: &mut String) {
    let mut buffer = String::new();

    debug!(log, "Performing typography substitutions"; "text" => &*text);

    macro_rules! replace {
        ($replacer:expr) => {
            $replacer.replace(log, text, &mut buffer)
        };
    }

    // Quotes
    replace!(DOUBLE_QUOTES);
    replace!(LOW_DOUBLE_QUOTES);
    replace!(SINGLE_QUOTES);

    // French quotes
    replace!(LEFT_DOUBLE_ANGLE);
    replace!(RIGHT_DOUBLE_ANGLE);

    // Miscellaneous
    replace!(ELLIPSIS);
}

#[cfg(test)]
const TEST_CASES: [(&str, &str); 4] = [
    (
        "John laughed. ``You'll never defeat me!''\n``That's where you're wrong...''",
        "John laughed. “You'll never defeat me!”\n“That's where you're wrong…”",
    ),
    (
        ",,あんたはばかです！''\n``Ehh?''\n,,ほんと！''\n[[footnoteblock]]",
        "„あんたはばかです！”\n“Ehh?”\n„ほんと！”\n[[footnoteblock]]",
    ),
    (
        "<< [[[SCP-4338]]] | SCP-4339 | [[[SCP-4340]]] >>",
        "« [[[SCP-4338]]] | SCP-4339 | [[[SCP-4340]]] »",
    ),
    (
        "**ENTITY MAKES DRAMATIC MOTION** . . . ",
        "**ENTITY MAKES DRAMATIC MOTION** … ",
    ),
];

#[test]
fn test_regexes() {
    let _ = &*SINGLE_QUOTES;
    let _ = &*DOUBLE_QUOTES;
    let _ = &*LOW_DOUBLE_QUOTES;
    let _ = &*LEFT_DOUBLE_ANGLE;
    let _ = &*RIGHT_DOUBLE_ANGLE;
    let _ = &*ELLIPSIS;
}

#[test]
fn test_substitute() {
    use super::test::test_substitution;

    test_substitution("typography", substitute, &TEST_CASES);
}
