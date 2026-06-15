mod grammar;
mod lifetimes;
mod punctuation;
mod quotes;
mod scopes;
mod words;

#[doc(hidden)]
pub mod __ {
	#![allow(missing_docs)] // Internal.

	pub use core::{
		cell::Cell,
		clone::Clone,
		compile_error, concat,
		convert::{From, Infallible},
		debug_assert, debug_assert_eq, debug_assert_matches,
		default::Default,
		iter::Extend,
		marker::{PhantomData, Sized},
		option::{Option, Option::None, Option::Some},
		panic::AssertUnwindSafe,
		primitive::bool,
		result::{Result, Result::Err, Result::Ok},
		stringify,
	};

	pub use std::{
		format,
		panic::{catch_unwind, resume_unwind},
		string::ToString,
		thread_local,
	};

	pub use proc_macro2::{
		Delimiter::{Brace, Bracket, Parenthesis},
		Ident, Punct, Spacing, Span, TokenStream, TokenTree,
	};

	pub use super::quotes::{
		Paste, block_directive, grouped, quote_one2, raw, rust_statement_directive, strip_dot, tt,
	};
}
