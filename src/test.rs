/*
 * test.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
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

//! Retrieves tests from JSON in the root `/test` directory, and runs them.
//!
//! Additionally performs some other tests from the parser which are better
//! in a dedicated test file.

use crate::includes::DebugIncluder;
use crate::parsing::{ParseWarning, ParseWarningKind, Token};
use crate::tree::{Element, SyntaxTree};
use std::borrow::Cow;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use void::ResultVoidExt;

const SKIP_TESTS: &[&str] = &[];

lazy_static! {
    static ref TEST_DIRECTORY: PathBuf = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test");
        path
    };
}

macro_rules! file_name {
    ($entry:expr) => {
        $entry.file_name().to_string_lossy()
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct Test<'a> {
    #[serde(skip)]
    name: String,
    input: String,
    tree: SyntaxTree<'a>,
    warnings: Vec<ParseWarning>,
}

impl Test<'_> {
    pub fn load(path: &Path, name: &str) -> Self {
        assert!(path.is_absolute());

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(error) => panic!("Unable to open file '{}': {}", path.display(), error),
        };

        let mut test: Self = match serde_json::from_reader(&mut file) {
            Ok(test) => test,
            Err(error) => {
                panic!("Unable to parse JSON file '{}': {}", path.display(), error)
            }
        };

        test.name = str!(name);
        test
    }

    pub fn run(&self, log: &slog::Logger) {
        info!(
            &log,
            "Running syntax tree test case";
            "name" => &self.name,
            "input" => &self.input,
        );

        if SKIP_TESTS.contains(&&*self.name) {
            println!("+ {} [SKIPPED]", self.name);
            return;
        }

        println!("+ {}", self.name);

        let (mut text, _pages) =
            crate::include(log, &self.input, DebugIncluder, || unreachable!())
                .void_unwrap();
        crate::preprocess(log, &mut text);
        let tokens = crate::tokenize(log, &text);
        let result = crate::parse(log, &tokens);
        let (tree, warnings) = result.into();

        fn json<T>(object: &T) -> String
        where
            T: serde::Serialize,
        {
            let mut output = serde_json::to_string_pretty(object)
                .expect("Unable to serialize JSON to stdout");

            output.insert_str(0, "Generated JSON: ");
            output
        }

        if tree != self.tree {
            panic!(
                "Running test '{}' failed! AST did not match:\nExpected: {:#?}\nActual: {:#?}\n{}\nWarnings: {:#?}",
                self.name,
                self.tree,
                tree,
                json(&tree),
                &warnings,
            );
        }

        if warnings != self.warnings {
            panic!(
                "Running test '{}' failed! Warnings did not match:\nExpected: {:#?}\nActual: {:#?}\n{}\nTree (correct): {:#?}",
                self.name,
                self.warnings,
                warnings,
                json(&warnings),
                &tree,
            );
        }
    }
}

#[test]
fn ast() {
    let log = crate::build_logger();

    // Warn if any test are being skipped
    if !SKIP_TESTS.is_empty() {
        println!("=========");
        println!(" WARNING ");
        println!("=========");
        println!();
        println!("The following tests are being SKIPPED:");

        for test in SKIP_TESTS {
            println!("- {}", test);
        }

        println!();
    }

    // Load tests from JSON files
    let entries = fs::read_dir(&*TEST_DIRECTORY) //
        .expect("Unable to read directory");

    let tests_iter = entries.filter_map(|entry| {
        let entry = entry.expect("Unable to read directory entry");
        let ftype = entry.file_type().expect("Unable to get file type");
        if !ftype.is_file() {
            println!("Skipping non-file {}", file_name!(entry));
            return None;
        }

        let path = entry.path();
        let stem = path
            .file_stem()
            .expect("Unable to get file stem")
            .to_string_lossy();

        let ext = path.extension().expect("Unable to get file extension");
        if ext != "json" {
            println!("Skipping non-JSON file {}", file_name!(entry));
            return None;
        }

        Some(Test::load(&path, &stem))
    });

    // Sort tests by name
    let mut tests: Vec<Test> = tests_iter.collect();
    tests.sort_by(|a, b| (a.name).cmp(&b.name));

    // Run tests
    println!("Running tests:");
    for test in tests {
        test.run(&log);
    }
}

/// Test the parser's recursion limit.
///
/// Manually implemented test, since this test would be
/// tremendously huge on disk as a JSON file, and
/// also goes past serde_json's recursion limit, lol.
#[test]
fn recursion_depth() {
    let log = crate::build_logger();

    // Build wikitext input
    let mut input = String::new();

    for _ in 0..101 {
        input.push_str("[[div]]\n");
    }

    for _ in 0..101 {
        input.push_str("[[/div]]\n");
    }

    // Run parser steps
    crate::preprocess(&log, &mut input);
    let tokens = crate::tokenize(&log, &input);
    let (tree, warnings) = crate::parse(&log, &tokens).into();

    // Check outputted warnings
    let warning = warnings.get(0).expect("No warnings produced");
    assert_eq!(warning.token(), Token::LeftBlock);
    assert_eq!(warning.rule(), "block-div");
    assert_eq!(warning.span(), 800..802);
    assert_eq!(warning.kind(), ParseWarningKind::RecursionDepthExceeded);

    // Check syntax tree
    //
    // It outputs the entire input string as text

    let SyntaxTree { elements, .. } = tree;
    assert_eq!(elements.len(), 1);

    let element = elements.get(0).expect("No elements produced");
    let input_cow = Cow::Borrowed(input.as_ref());
    assert_eq!(element, &Element::Text(input_cow));
}

/// Test the parser's ability to process large bodies
#[test]
fn large_payload() {
    const ITERATIONS: usize = 50;

    let log = crate::build_logger();

    // Build wikitext input
    let mut input = String::new();

    for _ in 0..ITERATIONS {
        // Lines intentionally broken in weird places
        input.push_str("
[[div]]
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Maecenas sed risus sed ex suscipit ultricies ac quis metus.
Mauris facilisis dui quam, in mollis velit ultrices vitae. Nam pretium accumsan arcu eu ultricies. Sed viverra eleifend elit at blandit. Aenean tempor vitae ipsum vitae lacinia.
Proin eu maximus nulla, id imperdiet libero. Duis convallis posuere arcu vitae sodales. Cras porta ac ligula non porttitor.
Proin et sodales arcu. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Mauris eget ante maximus, tincidunt enim nec, dignissim mi.
Quisque tincidunt convallis faucibus. Praesent vel semper dolor, vel tincidunt mi.

In hac habitasse platea dictumst. Vestibulum fermentum libero nec erat porttitor fermentum. Etiam at convallis odio, gravida commodo ipsum. Phasellus consequat nisl vitae ultricies pulvinar. Integer scelerisque eget nisl id fermentum. Pellentesque pretium, enim non molestie rhoncus, dolor diam porta mauris, eu cursus dolor est condimentum nisi. Phasellus tellus est, euismod non accumsan at, congue eget erat.

% ]] ! $ * -- @< _
[[/div]]
        ");
    }

    // Run parser steps
    crate::preprocess(&log, &mut input);
    let tokens = crate::tokenize(&log, &input);
    let (_tree, warnings) = crate::parse(&log, &tokens).into();

    // Check output
    assert_eq!(warnings.len(), ITERATIONS * 3);
}
