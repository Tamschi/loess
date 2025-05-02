# Loess-Rust

`loess-rust` is a [Loess](https://crates.io/crates/loess)-grammar that tracks Rust's stable grammar by closely following [The Rust Reference](https://doc.rust-lang.org/stable/reference/).

Here's what to expect:

- Shallow parsing (by default). For tokens with delimited groups, like `Visibility`, you can opt into deeper (or customised!) parsing via generics.

- Public fields and one-time validation. The parser checks token specifics once when processing input, but trusts you otherwise.

- Some bugs. For example, none-delimited groups aren't handled yet, which can cause issues when generating macro input with a `macro_rules!` macro.

Here's what not to expect:

- Complete coverage of Rust's grammar. In fact, `loess-rust` really makes no attempt at all in this regard, since I only implement what I need. In particular, unstable grammar is generally out of scope. (Loess can help you supply it yourself!)

  Temporary opaque implementations of additional grammar tokens are available in the [`loess-rust-opaque`](https://crates.io/crates/loess-rust-opaque) crate.

- `Debug`-implementations. They aren't that useful here in my experience, but they would increase compile-times.

- Absence of major version bumps. Rust's grammar is a moving target and Loess's grammar tokens aren't marked `#[non_exhaustive]` for ease of use.

  However, shallow parsing should make upgrades fairly painless and errors should alert you specifically to grammar changes that are relevant to you.

  I should also be able to use the semver trick each time (reexport compatible new-major-version types in the older version) to keep incompatibilities and overall compile time to a minimum.

How to read the version number:

After the "+", the most-major version of the (most directly) compatible Loess releases is listed.
