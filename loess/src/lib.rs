use std::{
	collections::VecDeque,
	sync::atomic::{AtomicU64, Ordering},
};

use proc_macro2::{Literal, Punct, Span, TokenStream, TokenTree};

mod proc_macro2_impls;

pub mod rust_reference;

pub struct Error {
	message: String,
	spans: Vec<Span>,
}

impl Error {
	pub fn new(message: impl Into<String>, spans: impl IntoIterator<Item = Span>) -> Self {
		Self {
			message: message.into(),
			spans: spans.into_iter().collect(),
		}
	}
}

impl ToTokens for Error {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let message = Literal::string(&self.message);
		let span = self
			.spans
			.iter()
			.copied()
			.map(Some)
			.reduce(|a, b| a.as_ref().zip(b).map(|(a, b)| a.join(b)).flatten())
			.flatten()
			.or_else(|| self.spans.first().copied())
			.unwrap_or_else(Span::mixed_site);
		quote_spanned! {span=>
			::core::compile_error!(#message);
		}
		.to_tokens(tokens);
	}
}

pub trait PopFrom {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Vec<Error>) -> Result<Self, ()>
	where
		Self: Sized;
}

impl<T: PopFrom> PopFrom for Vec<T> {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Vec<Error>) -> Result<Self, ()> {
		let mut this = Self::new();
		while !input.is_empty() {
			let before_len = input.len();

			this.push(T::pop_from(input, errors)?);

			if input.len() == before_len {
				let rest = input.iter().cloned().collect::<TokenStream>();

				errors.push(Error::new(
					format!("Unconsumed tokens: `{rest}`"),
					rest.into_iter().map(|t| t.span()),
				));
				break;
			}
		}

		Ok(this)
	}
}

pub trait PopOrReplaceExt {
	fn pop_or_replace<'a, T, const N: usize>(
		&'a mut self,
		f: impl FnOnce([TokenTree; N]) -> Result<T, [TokenTree; N]>,
	) -> Result<T, impl 'a + IntoIterator<Item = Span>>;
}

impl PopOrReplaceExt for VecDeque<TokenTree> {
	fn pop_or_replace<'a, T, const N: usize>(
		&'a mut self,
		f: impl FnOnce([TokenTree; N]) -> Result<T, [TokenTree; N]>,
	) -> Result<T, impl 'a + IntoIterator<Item = Span>> {
		// This is optimisable to be essentially a no-op iff `Err`.
		//TODO: Handle none-delimiter groups.
		if self.len() < N {
			Err(self.iter().map(|t| t.span()).collect::<Vec<_>>())
		} else {
			match f([(); N].map(|()| self.pop_front().expect("unreachable"))) {
				Ok(value) => Ok(value),
				Err(ts) => {
					let spans = ts.iter().map(|t| t.span()).collect();
					for t in ts.into_iter().rev() {
						self.push_front(t);
					}
					Err(spans)
				}
			}
		}
	}
}

trait WithSpanExt {
	fn with_span(self, span: Span) -> Self;
}

impl WithSpanExt for Punct {
	fn with_span(mut self, span: Span) -> Self {
		self.set_span(span);
		self
	}
}

static PLACEHOLDER_COUNT: AtomicU64 = AtomicU64::new(1);

pub fn next_placeholder_number() -> u64 {
	PLACEHOLDER_COUNT.fetch_add(1, Ordering::Relaxed)
}

pub trait Placeholder {
	fn placeholder() -> Self;
}

pub trait SimpleSpanned {
	fn span(&self) -> Span;
}

mod __ {
	use crate::Placeholder;

	trait Sealed {}

	#[allow(private_bounds)]
	pub trait UnwrapOrPlaceholder: Sealed {
		type Output: Placeholder;
		fn unwrap_or_placeholder(self) -> Self::Output;
	}

	impl<T: Placeholder> Sealed for Option<T> {}
	impl<T: Placeholder> UnwrapOrPlaceholder for Option<T> {
		type Output = T;

		fn unwrap_or_placeholder(self) -> Self::Output {
			self.unwrap_or_else(T::placeholder)
		}
	}

	impl<T: Placeholder> Sealed for Result<T, ()> {}
	impl<T: Placeholder> UnwrapOrPlaceholder for Result<T, ()> {
		type Output = T;

		fn unwrap_or_placeholder(self) -> Self::Output {
			self.unwrap_or_else(|()| T::placeholder())
		}
	}
}

pub use __::UnwrapOrPlaceholder;
use quote::{ToTokens, quote_spanned};
