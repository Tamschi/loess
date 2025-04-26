//! With `"rust_grammar"`: Tokens representing the stable Rust programming language,
//! closely following [The Rust Reference](https://doc.rust-lang.org/stable/reference/).
//!
//! Corrections in that regard are not automatically considered breaking changes,
//! unless they became necessary due to a change in Rust **and** reduce what is considered valid.
//!
//! Breaking changes to the API are considered breaking as normal.
//!
//! *Note that unstable grammar may be accidentally accepted in some cases, especially where parsing is temporarily delegated to [Syn](https://docs.rs/syn).*  
//! ***Ceasing to accept unstable grammar is not by itself considered a breaking change for Loess.***
//!
//! Additional tokens (opaque implementations) are available with the `"opaque_rust_grammar"` feature,
//! but enabling it pulls in part of [Syn](https://docs.rs/syn),
//! which considerably increases build times.

// TODO (breaking): This should be a separate crate.

mod expressions;
pub use expressions::*;

mod lexical_structure;
pub use lexical_structure::*;

mod names;
pub use names::*;

#[cfg(feature = "opaque_rust_grammar")]
mod opaque_rust_grammar;
#[cfg(feature = "opaque_rust_grammar")]
pub use opaque_rust_grammar::*;
