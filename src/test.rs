/*
 * test.rs
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

//! Retrieves tests from JSON in the root `/test` directory, and runs them.

use crate::parse::ParseError;
use crate::tree::SyntaxTree;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

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
    name: String,
    input: String,
    tree: SyntaxTree<'a>,
    errors: Vec<ParseError>,
}

impl Test<'_> {
    pub fn load(path: &Path) -> Self {
        assert!(path.is_absolute());

        let mut file = File::open(path).expect("Unable to open file");
        serde_json::from_reader(&mut file).expect("Unable to parse JSON")
    }

    pub fn run(&self, log: &slog::Logger) {
        info!(
            &log,
            "Running syntax tree test case";
            "name" => &self.name,
            "input" => &self.input,
        );

        println!("+ {}", self.name);

        let tokens = crate::tokenize(log, &self.input);
        let result = crate::parse(log, &tokens);
        let (tree, errors) = result.into();

        if tree != self.tree {
            panic!(
                "Running test '{}' failed! AST did not match:\nExpected: {:#?}\nActual: {:#?}",
                self.name,
                self.tree,
                tree,
            );
        }

        if errors != self.errors {
            panic!(
                "Running test '{}' failed! Errors did not match:\nExpected: {:#?}\nActual: {:#?}",
                self.name,
                self.errors,
                errors,
            );
        }
    }
}

#[test]
fn ast() {
    let log = crate::build_logger();

    // Load tests from JSON files
    let entries = fs::read_dir(&*TEST_DIRECTORY) //
        .expect("Unable to read directory");

    let tests = entries.filter_map(|entry| {
        let entry = entry.expect("Unable to read directory entry");
        let ftype = entry.file_type().expect("Unable to get file type");
        if !ftype.is_file() {
            println!("Skipping non-file {}", file_name!(entry));
            return None;
        }

        let path = entry.path();
        let ext = path.extension().expect("Unable to get file extension");

        if ext != "json" {
            println!("Skipping non-JSON file {}", file_name!(entry));
            return None;
        }

        Some(Test::load(&path))
    });

    // Run tests
    for test in tests {
        test.run(&log);
    }
}
