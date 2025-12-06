//! **Shallow** [Loess](`loess`) grammar representing the stable Rust programming language,
//! closely following [The Rust Reference](https://doc.rust-lang.org/stable/reference/).
//!
//! Corrections in that regard are not automatically considered breaking changes,
//! unless they became necessary due to a change in Rust **and** reduce what is considered valid.
//!
//! Breaking changes to the API are considered breaking as normal.
//!
//! This crate is focused on parsing, so while the types implement [`loess::IntoTokens`],
//! essentially anything slightly complex is tagged with `#[non_exhaustive]` and constructing
//! those types may be difficult. Consider emitting through [`loess::quote_into_mixed_site!`] instead.
//!
//! *Note that unstable grammar may be accidentally accepted in some cases.*  
//! ***Ceasing to accept unstable grammar is not by itself considered a breaking change for Loess.***

//TODO: Make another crate that provides quote-like e.g. `reparse_mixed_site` macros?

pub mod attributes;
pub mod expr;
pub mod ident;
#[path = "items.rs"]
pub mod items;
pub mod lex;
pub mod r#macro;
pub mod paths;
pub mod statement;
pub mod vis;
