//! Contains tokens representing the stable Rust programming language,
//! closely following [The Rust Reference](https://doc.rust-lang.org/stable/reference/).
//!
//! Corrections in that regard are not automatically considered breaking changes,
//! unless they became necessary due to a change in Rust **and** reduce what is considered valid.
//!
//! Breaking changes to the API are considered breaking as normal.
//!
//! *Note that unstable grammar may be accidentally accepted in some cases, especially where parsing is temporarily delegated to [Syn](`syn`).*  
//! ***Ceasing to accept unstable grammar is not by itself considered a breaking change for Loess.***

mod expressions;
pub use expressions::*;

mod lexical_structure;
pub use lexical_structure::*;

mod names;
pub use names::*;

mod patterns;
pub use patterns::*;

mod statements;
pub use statements::*;
