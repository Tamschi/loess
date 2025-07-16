# Loess

Loess is a parser library and parser generator for proc macros.

For a simple but representative **example** of using Loess, see the [*inline-json5*](https://crates.io/crates/inline-json5) crate.

Premade Rust grammar implementations can be found in [*loess-rust*](https://crates.io/crates/loess-rust), with additional temporary wrappers available in [*loess-rust-opaque*](https://crates.io/crates/loess-rust-opaque).

Here's what to expect:

- **Fast builds.** Loess's core is compact and language agnostic. Direct grammar implementations like *loess-rust* should compile fairly quickly too.

- **A simple, flexible API.** Loess is relatively unopinionated about how or what you parse, and you can construct (and destructure) `Input` at any time.

- ***Really* good error reporting** from proc macros implemented with Loess, *by default*.

  Many parsing errors are easily or automatically recoverable, which means multiple errors can be reported at once while preserving as much regular output as possible (which means no or much fewer cascading errors!).

  Panics inside your parser can also be caught and reported as located errors with the original panic message.

- **A reasonably powerful parser-generator**.

  `grammar!` can emit documentation (for enums) and `PeekFrom`, `PopFrom` and `IntoTokens` implementations on grammar types in general.

- **Powerful `quote_into` macros** that expand efficiently and are language-agnostic.

  You can cleanly loop and/or branch within the template.

- Low-allocation workflow.

  Loess can (usually) move tokens from input to output without cloning them. (You can still clone all included grammar types explicitly, including when pasting in quotes.)

Here's what not to expect:

- A general Syn-replacement (at least not soon).

  Loess is mainly aimed at implementing domain-specific languages that may cite fragments of Rust verbatim in their output. There is currently no focus on parsing Rust code or transforming it *in-depth*.

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
