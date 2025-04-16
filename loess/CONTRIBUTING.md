# Contributing to Loess

Please create a GitHub issue for your idea before starting work on a merge-request.  
Doing so makes it easier to see if the change is likely to be merged once complete.

When you start work on your change, please create a draft PR for ongoing discussion.

## Guidelines

Don't use procedural macros in and (in most cases) don't add dependencies to Loess.

The `rust_grammar` module is organised roughly like [<cite>The Rust Reference</cite>](https://doc.rust-lang.org/stable/reference/), with opaque implementations split into a separate subfolder. Type names <em style=font-style:normal;font-variant:small-caps>should</em> correspond directly to named grammar tokens or other applicable names in <cite>The Rust Reference</cite>.

Grammar tokens <em style=font-style:normal;font-variant:small-caps>should</em> be documented to show their immediate pattern in the module overview.
(See `rust_grammar` for examples. `grammar!` can generate this documentation for you in some cases.) This is not required for temporary opaque implementations, which instead should identify themselves as such.

**I'm open to adding additional feature-gated grammars** for common text file formats,
as long as they can be parsed accurately on stable Rust. (That means no YAML, but JSON would be okay.)  
Type names <em style=font-style:normal;font-variant:small-caps>may</em> be reused between distinct grammars, but <em style=font-style:normal;font-variant:small-caps>must not</em> be reused within the same grammar. Where a name collides with a Rust keyword, the respective raw identifier (`r#…`) should be used if available.

You <em style=font-style:normal;font-variant:small-caps>may</em> introduce new `macro_rules!` macros to avoid large amounts of repetitive code. Please keep them reasonably simple (but try to match my standards, which mainly means optional trailing repeat-separators <em style=font-style:normal;font-variant:small-caps>should</em> be supported in the input).

Try to focus on what you need as a consume and don't aim for completeness for completeness's sake. That saves work for us both 🙂

## Formatting

Please format with:

```sh
cargo +nightly fmt -p loess
```

## Testing

Please test with:

```sh
cargo test -p loess --features rust_grammar
cargo test -p loess --features opaque_rust_grammar
```

Tests outside the opaque grammar module <em style=font-style:normal;font-variant:small-caps>must not</em> require Syn.

## Publishing

Publish with:

```sh
cargo publish --dry-run --locked -p loess -F opaque_rust_grammar
cargo publish --locked -p loess -F opaque_rust_grammar
```
