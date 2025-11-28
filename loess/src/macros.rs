mod grammar;
mod words;

#[doc(hidden)]
pub mod __ {
	#![allow(missing_docs)] // Internal.

	pub use core::{
		clone::Clone, compile_error, concat, iter::Extend, primitive::bool, result::Result,
		stringify,
	};

	pub use proc_macro2::{
		Delimiter::{Brace, Bracket, Parenthesis},
		Ident, Span, TokenStream, TokenTree,
	};

	pub use super::{
		grammar::{
			Paste, block_directive, grouped, quote_one2, raw, rust_statement_directive, strip_dot,
			tt,
		},
		words::{impl_word, words_muncher},
	};
}
