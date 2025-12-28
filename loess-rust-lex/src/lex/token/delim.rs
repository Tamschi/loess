//! [lex.token.delim](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token.delim): Delimiters

use proc_macro2::TokenStream;

pub type CurlyBraces = loess::scaffold::CurlyBraces<TokenStream>;
pub type SquareBrackets = loess::scaffold::SquareBrackets<TokenStream>;
pub type Parentheses = loess::scaffold::Parentheses<TokenStream>;
