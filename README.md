## ftml

<p>
  <a href="https://github.com/Nu-SCPTheme/ftml/actions?query=workflow%3A%22Rust+CI%22">
    <img src="https://github.com/Nu-SCPTheme/ftml/workflows/Rust%20CI/badge.svg"
         alt="Rust CI badge">
  </a>
</p>

**Foundation Text Markup Language**

A Rust library to parse Wikidot code into an abstract syntax tree (AST).
This aims to be a replacement for the aging [Text\_Wiki](https://github.com/gabrys/wikidot/tree/master/lib/Text_Wiki/Text) from Wikidot.
This is an experimental branch to try and have a nearly fully compatible parser for Wikidot, including malformed constructs.
The goal is to utilize a lexer generator, and consume the tokens in a custom parser to handle unusual cases with a lax approach.

In addition to providing the speed and safety benefits of Rust, this also improves maintainability, and allows exposing an AST to consumers
for more advanced analysis and transformation.

The lint `#![forbid(unsafe_code)]` is set, and therefore this crate has only safe code. However dependencies may have `unsafe` internals.

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.48.0

```sh
$ cargo build --release
```

You can use this as a dependency by adding the following to your `Cargo.toml`:

```toml
ftml = { git = "https://github.com/NuSCP-Theme/ftml", branch = "next" }
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.

### Usage
There are three exported functions, which correspond to each of the main steps in the wikitext process.

First is `preprocess`, which will perform Wikidot substitutions on the text. It takes a `&dyn Handle` as it also expands `[[include]]`d components for their actual text.

Second is `tokenize`, which takes the input string and returns a list of extracted tokens from it, all borrowing from it.

Then, borrowing a slice of said tokens, `parse` consumes them and produces a `SyntaxTree` representing the full structure of the parsed wikitext.

```rust
fn preprocess(
    log: &slog::Logger,
    text: &mut String,
    includer: &dyn Handle,
)

fn tokenize<'t>(
    log: &slog::Logger,
    text: &'t str,
) -> Vec<ExtractedToken<'t>>

fn parse<'r, 't>(
    log: &slog::Logger,
    mut tokens: &'r [ExtractedToken<'t>],
) -> ParseResult<SyntaxTree<'t>>
```

When performing a parse, you will need to first run `preprocess()`, then run `parse()`
on the fully expanded text.
