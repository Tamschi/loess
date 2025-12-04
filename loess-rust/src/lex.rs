//! [Lexical structure](https://doc.rust-lang.org/stable/reference/lexical-structure.html)

use loess::{Error, ErrorPriority, Errors, Input, PeekFrom, PopFrom, PopParsedFrom, words};
use proc_macro2::{Ident, TokenTree};

pub mod token;
pub mod keywords;