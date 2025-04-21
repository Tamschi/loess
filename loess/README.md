# Loess

Loess is a parser library and parser generator for proc macros.

Here's what to expect:

- Fast builds. Loess's core is compact, language agnostic, and useful without enabling a premade grammar.

  That said, even in cases where you do enable a grammar module, builds should still be fairly quick.

- A simple, flexible API. Loess is relatively unopinionated about how or what you parse, and you can construct (and destructure) `Input` at any time.

- Shallow parsing (by default). For tokens with groups, like `Visibility`, you can opt into deeper (or customised!) parsing via generics.

- Public fields and one-time validation. The parser checks token specifics once when processing input, but trusts you otherwise.

- A reasonably powerful parser-generator.

  `grammar!` can emit documentation (for enums) and `PeekFrom`, `PopFrom` and `IntoTokens` implementations on grammar types in general.

- **Really** good error reporting from proc macros implemented with Loess, *by default*.

  This includes locating panics relative to the proc macro input, instead of squiggling the whole macro.

- Lenient and partial parsing. The parsers can continue (after reporting an error) when a repeating parse fails in a delimited group.

  You can use this property to still emit as much output as possible, which avoids cascading errors.

- Low-allocation workflow.

  Loess can (usually) move tokens from input to output without cloning them. (You can still clone all included grammar types explicitly.)

- Some bugs. For example, none-delimited groups aren't handled yet, which can cause issues when generating macro input with a `macro_rules!` macro.

Here's what not to expect:

- Complete coverage of Rust's grammar. In fact, Loess really makes no attempt at all in this regard, since I only implement what I need.

  In particular, unstable grammar is generally out of scope of the included parsers. (Loess can help you supply it yourself!)

- A Syn-replacement (at least not soon). While there's no public interaction with Syn, some optional grammar tokens are for now opaque and do defer to Syn when enabled.

- `Debug`-implementations on the included grammars. They aren't that useful here in my experience, but they would increase compile-times.

- Absence of major version bumps. Rust's grammar is a moving target and Loess's grammar tokens aren't marked `#[non_exhaustive]` for ease of use.

  However, shallow parsing should make upgrades fairly painless and errors should alert you specifically to grammar changes that are relevant to you.

## Examples

<details><summary style=cursor:pointer><u>(click to expand code block)</u></summary>

```rust
use loess::{
    grammar, parse_all, Input, Errors, PeekFrom, PopFrom, IntoTokens,
    rust_grammar::{ // With the `"rust_grammar"` feature.
        Await, CurlyBraces, Dot, Identifier, Parentheses, Semi, SquareBrackets,
    }
};
use proc_macro2::{Span, TokenStream};

// Generates parsers and pasters, according to the traits written after the type name.
//
// (This macro is hygienic, so you don't have to import the traits for this.)
grammar! {
    pub struct Child: PeekFrom, PopFrom, IntoTokens {
        pub identifier: ChildIdentifier,
        /// Groups are generic (and capture [`TokenStream`] by default.)
        pub new_args: Option<Parentheses>,
        pub dot_await: Option<DotAwait>,
        pub render_args: Option<SquareBrackets>,
        pub children: ChildChildren,
    }

    pub struct DotAwait: PeekFrom, PopFrom, IntoTokens {
        pub dot: Dot,
        pub r#await: Await,
    }

    // It's basic so far, but some documentation can be generated too.
    pub enum ChildIdentifier: doc, IntoTokens {
        Local(Identifier),
        Substrate(Identifier),
        Qualified(TokenStream),
    } else "Expected child identifier.";

    pub enum ChildChildren: PopFrom, IntoTokens {
        Void(Semi),
        Braces(CurlyBraces<Vec<Child>>),
    } else "Expected `;` or `{`.";
}

// Custom logic can be added through simple traits.
impl PeekFrom for ChildIdentifier {
    fn peek_from(input: &Input) -> bool {
        unimplemented!("Just an example.")
    }
}

impl PopFrom for ChildIdentifier {
    // Errors can be emitted even when the parser recovers.
    //
    // This allows multiple errors to be reported at once (subject to priority), and also
    // allows graceful degradation of macro output to avoid cascading errors elsewhere.
    fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
        unimplemented!("Just an example.")
    }
}

// Loess has a flexible, unopinionated API:
fn macro_impl(input: TokenStream) -> TokenStream {
    let mut errors = Errors::new();

    // Turns panics into located errors and checks for exhaustiveness.
    // (Errors for unconsumed input have low priority to avoid distractions.)
    let children: Vec<Child> = parse_all(
            &mut Input {
                // This is a plain `VecDeque<TokenTree>`.
                tokens: input.into_iter().collect(),

                // Used to locate errors if the end of input was reached unexpectedly.
                // Nightly macros can use `Span::end` to get a better error location.
                end: Span::call_site(),
            },
            &mut errors,
        ).collect();

    // You can also step through `Input` via for `parse_once…` functions, but you should
    // always use a `parse_all…` function last to check for unconsumed input.

    let root = TokenStream::new();
    let mut output = TokenStream::new();

    errors.into_tokens(
        // Optional `root` module that reexports dependencies.
        // Mainly for wrapped macros that have access to `$crate`.
        //
        // Iff not empty, `Errors` assumes that `core` is reexported there.
        &root,
        &mut output,
    );

    // You can emit your output step-by-step, or exit early after emitting `errors`.
    children.into_tokens(&root, &mut output);

    output
}

// Alternatively:

fn macro_impl2(input: TokenStream) -> TokenStream {
    let mut errors = Errors::new();

    let root = TokenStream::new();

    grammar! {
        struct Grammar: PopFrom (
            Identifier,
            CurlyBraces<Vec<Child>>,
        );
    }

    let Some(Grammar(name, children)) = parse_all(
            &mut Input {
                // This is a plain `VecDeque<TokenTree>`.
                tokens: input.into_iter().collect(),

                // Used to locate errors if the end of input was reached unexpectedly.
                // Nightly macros can use `Span::end` to get a better error location.
                end: Span::call_site(),
            },
            &mut errors,
        ).next() else { return errors.collect_tokens(&root); };

    let mut output = errors.collect_tokens(&root);

    // Emit your output step-by-step.
    name.into_tokens(&root, &mut output);
    children.into_tokens(&root, &mut output);

    output
}
```

</details>

## Using `$crate` for full caller independence

`loess::IntoTokens`-methods take an (optionally empty) `root: &TokenStream` parameter,
which all emitted fully qualified paths <em style=font-style:normal;font-variant:small-caps>should</em> be prefixed with.

In combination with a wrapper crate: This achieves full isolation regarding caller dependencies:

<details><summary style=cursor:pointer><u>(click to expand code blocks)</u></summary>

<!-- These code blocks also appear on `IntoTokens`,
the first one with some hidden lines to make it compile. -->

```rust ,ignore
// wrapper crate

#[macro_export]
macro_rules! my_macro {
    ($($tt:tt)*) => ( $crate::__::my_macro!([$crate] $($tt)*) );
}

#[doc(hidden)]
pub mod __ {
    pub use core; // Expected by `Errors`.
    pub use my_macro_impl::my_macro;
}
```

```rust
// my_macro_impl (proc macro)

use loess::{
    grammar, parse_once, parse_all,
    Errors, Input, IntoTokens,
    rust_grammar::{SquareBrackets},
};
use proc_macro2::{Span, TokenStream, TokenTree};

// […]

fn macro_impl(input: TokenStream) -> TokenStream {
    let mut input = Input {
        tokens: input.into_iter().collect(),
        end: Span::call_site(),
    };
    let mut errors = Errors::new();

    // `root` is implicitly a `TokenStream`.
    let Ok(SquareBrackets { contents: root, .. }) = parse_once(
            &mut input,
            &mut errors,
        ) else { return errors.collect_tokens(&TokenStream::new()) };

    grammar! {
        /// This represents your complete input grammar.
        /// This here is a placeholder, so it's empty.
        struct Grammar: PopFrom {}
    }

    // Checks for exhaustiveness.
    let parsed = parse_all(&mut input, &mut errors).next();
    let mut output = errors.collect_tokens(&root);

    if let Some(Grammar {}) = parsed {
        // Emit your output here.
    }

    output
}
```

</details>
