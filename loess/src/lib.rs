use std::{
	any::type_name,
	collections::VecDeque,
	fmt::Debug,
	marker::PhantomData,
	sync::atomic::{AtomicU64, Ordering},
};

use error_priorities::{UNCONSUMED_AFTER_REPEATS, UNCONSUMED_INPUT};
use proc_macro2::{Literal, Punct, Span, TokenStream, TokenTree};

mod proc_macro2_impls;

pub mod rust_reference;

#[derive(Debug)]
pub struct Error {
	priority: ErrorPriority,
	message: String,
	spans: Vec<Span>,
}

impl Error {
	pub fn new(
		priority: ErrorPriority,
		message: impl Into<String>,
		spans: impl IntoIterator<Item = Span>,
	) -> Self {
		Self {
			priority,
			message: message.into(),
			spans: spans.into_iter().collect(),
		}
	}
}

impl ToTokens for Error {
	fn to_tokens(&self, tokens: &mut TokenStream) {
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

#[derive(Debug)]
pub struct Errors {
	errors: Vec<Error>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ErrorPriority(f64);

pub trait ConstErrorPriority {
	const PRIORITY: ErrorPriority;
}

impl ErrorPriority {
	const fn new(value: f64) -> Self {
		assert!(!value.is_nan());
		Self(value)
	}

	pub const fn next_lower(self) -> Self {
		Self(self.0.next_down())
	}

	const TOKEN: Self = Self::new(0.);
	const GRAMMAR: Self = Self::new(0.);
	const UNCONSUMED_AFTER_REPEATS: Self = Self::new(-1.);
	const UNCONSUMED_IN_DELIMITER: Self = Self::new(-2.);
	const UNCONSUMED_INPUT: Self = Self::new(-3.);
}

pub mod error_priorities {
	#![allow(non_camel_case_types)]

	use crate::{ConstErrorPriority, ErrorPriority};

	#[derive(Debug)]
	pub enum TOKEN {}
	impl ConstErrorPriority for TOKEN {
		const PRIORITY: ErrorPriority = ErrorPriority::TOKEN;
	}

	#[derive(Debug)]
	pub enum GRAMMAR {}
	impl ConstErrorPriority for GRAMMAR {
		const PRIORITY: ErrorPriority = ErrorPriority::GRAMMAR;
	}

	#[derive(Debug)]
	pub enum UNCONSUMED_AFTER_REPEATS {}
	impl ConstErrorPriority for UNCONSUMED_AFTER_REPEATS {
		const PRIORITY: ErrorPriority = ErrorPriority::UNCONSUMED_AFTER_REPEATS;
	}

	#[derive(Debug)]
	pub enum UNCONSUMED_IN_DELIMITER {}
	impl ConstErrorPriority for UNCONSUMED_IN_DELIMITER {
		const PRIORITY: ErrorPriority = ErrorPriority::UNCONSUMED_IN_DELIMITER;
	}

	#[derive(Debug)]
	pub enum UNCONSUMED_INPUT {}
	impl ConstErrorPriority for UNCONSUMED_INPUT {
		const PRIORITY: ErrorPriority = ErrorPriority::UNCONSUMED_INPUT;
	}
}

impl Eq for ErrorPriority {}

impl Ord for ErrorPriority {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).expect("total")
	}
}

impl Errors {
	pub fn new() -> Self {
		Self { errors: vec![] }
	}

	pub fn push(&mut self, error: Error) {
		self.errors.push(error)
	}
}

impl ToTokens for Errors {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Some(highest_priority) = self.errors.iter().map(|error| error.priority).max() else {
			return;
		};

		for error in self.errors.iter() {
			if error.priority == highest_priority {
				error.to_tokens(tokens);
			}
		}
	}
}

pub trait PopFrom {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized;
}

const _: () = {
	use std::collections::VecDeque;
	use std::fmt::Debug;

	use proc_macro2::TokenTree;

	use crate::{EndOfInput, Errors, PopFrom};

	trait VecLike: Default + IntoIterator + Extend<Self::Item> {}
	impl<T> VecLike for Vec<T> {}
	impl<T> VecLike for VecDeque<T> {}
	impl VecLike for TokenStream {}

	/// Sadly, lack of specialisation requires this to be scoped.
	///
	/// This implementation applies to: [`Vec`], [`VecDeque`]
	impl<T: Default + IntoIterator<Item: PopFrom> + Extend<T::Item>> PopFrom for T
	where
		T: VecLike,
		T::Item: 'static + Debug,
	{
		fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()> {
			let mut this = T::default();
			while !input.is_empty() {
				let before_len = input.len();

				dbg!(type_name::<T::Item>());
				match dbg!(T::Item::pop_from(input, errors)) {
					Ok(item) => this.extend([dbg!(item)]),
					Err(()) => {
						EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors).ok();
						return Ok(this);
					}
				}

				if dbg!(input.len() == before_len) {
					EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors)
						.expect_err("because of `while !input.is_empty()`");
					break;
				}
			}

			Ok(this)
		}
	}
};

pub struct Exhaustive<T, P: ConstErrorPriority>(pub T, PhantomData<P>);

impl<T: PopFrom, P: ConstErrorPriority> PopFrom for Exhaustive<T, P> {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()> {
		let value = T::pop_from(input, errors);
		EndOfInput::<UNCONSUMED_INPUT>::pop_from(input, errors).ok();
		Ok(Self(value?, PhantomData))
	}
}

#[derive(Debug)]
pub struct EndOfInput<P: ConstErrorPriority>(PhantomData<P>);

impl<P: ConstErrorPriority> PopFrom for EndOfInput<P> {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()> {
		input
			.is_empty()
			.then_some(Self(PhantomData))
			.ok_or_else(|| {
				let rest = input.iter().cloned().collect::<TokenStream>();
				errors.push(Error::new(
					P::PRIORITY,
					format!("Unconsumed tokens: `{rest}`"),
					rest.into_iter().map(|t| t.span()),
				));
			})
	}
}

pub struct Defaulted<T>(pub T);

impl<T> Defaulted<T> {
	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T: Default + PopFrom> PopFrom for Defaulted<T>
where
	T: Debug,
{
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self(dbg!(T::pop_from(input, errors)).unwrap_or_default()))
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
