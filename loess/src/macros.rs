mod grammar;
mod lifetimes;
mod punctuation;
mod quotes;
mod words;

#[doc(hidden)]
pub mod __ {
	#![allow(missing_docs)] // Internal.

	pub use core::{
		clone::Clone, compile_error, concat, debug_assert, iter::Extend, primitive::bool,
		result::Result, stringify,
	};

	pub use std::string::ToString;

	pub use proc_macro2::{
		Delimiter::{Brace, Bracket, Parenthesis},
		Ident, Punct, Spacing, Span, TokenStream, TokenTree,
	};

	pub use super::quotes::{
		Paste, block_directive, grouped, quote_one2, raw, rust_statement_directive, strip_dot, tt,
	};
}
