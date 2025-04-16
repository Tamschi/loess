# Loess

Loess is a parser library and parser generator for proc macros.

Here's what to expect:

- A simple, flexible API. Loess is relatively unopinionated about how or what you parse, and you can construct (and destructure) `Input` at any time.

- Shallow parsing (by default). For tokens with groups, like `Visibility`, you can opt into deeper (or customised!) parsing via generics.

- Public fields and one-time validation. The parser checks token specifics once when processing input, but trusts you otherwise.

- A reasonably powerful parser-generator.

  `grammar!` can emit documentation (for enums), `PeekFrom`, `PopFrom` and `IntoTokens` implementations on grammar types.

- **Really** good error reporting from proc macros implemented with Loess, *by default*.

  This includes locating panics relative to the proc macro input, instead of squiggling the whole macro.

- Lenient and partial parsing. The parsers can continue (after reporting an error) when a repeating parse fails in a delimited group.

  You can use this property to still emit as much output as possible, which avoids cascading errors.

- Low-allocation workflow.

  Loess can (usually) move tokens from input to output without cloning them. (You can still clone all grammar types explicitly.)

Here's what not to expect:

- Complete coverage of Rust's grammar. In fact, Loess really makes no attempt at all in this regard, since I only implement what I need.

  In particular, unstable grammar is generally out of scope of the included parsers. (Loess can help you supply it yourself!)

- A Syn-replacement (at least not soon). While there's no public interaction with Syn, some grammar DTOs are for now opaque and defer to Syn.

- `Debug`-implementations. They aren't that useful here in my experience, but they increase compile-times.

- Absence of major version bumps. Rust's grammar is a moving target and Loess's grammar DTOs aren't marked `#[non_exhaustive]` for ease of use.

  However, shallow parsing should make upgrades fairly painless and errors should alert you specifically to grammar changes that are relevant to you.
