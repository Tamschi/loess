use std::{
	collections::VecDeque,
	fmt::Debug,
	iter,
	marker::PhantomData,
	sync::atomic::{AtomicU64, Ordering},
};

use error_priorities::{UNCONSUMED_AFTER_REPEATS, UNCONSUMED_INPUT};
use proc_macro2::{Literal, Punct, Span, TokenStream, TokenTree};
use quote::quote_spanned;

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

impl IntoTokens for Error {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
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
			#root::core::compile_error!(#message);
		}
		.into_tokens(root, tokens);
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

	pub const TOKEN: Self = Self::new(0.);
	pub const GRAMMAR: Self = Self::new(0.);
	pub const UNCONSUMED_AFTER_REPEATS: Self = Self::new(-1.);
	pub const UNCONSUMED_IN_DELIMITER: Self = Self::new(-2.);
	pub const UNCONSUMED_INPUT: Self = Self::new(-3.);
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

pub trait IntoTokens {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>);

	fn collect_tokens<T: Default + Extend<TokenTree>>(self, root: &TokenStream) -> T
	where
		Self: Sized,
	{
		let mut tokens = T::default();
		self.into_tokens(root, &mut tokens);
		tokens
	}
}

impl<T: IntoTokens> IntoTokens for Option<T> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		if let Some(value) = self {
			value.into_tokens(root, tokens);
		}
	}
}

impl<T: IntoTokens> IntoTokens for Vec<T> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		for value in self {
			value.into_tokens(root, tokens);
		}
	}
}

impl IntoTokens for TokenTree {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([self])
	}
}

impl IntoTokens for Punct {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([TokenTree::Punct(self)])
	}
}

impl IntoTokens for TokenStream {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend(self);
	}
}

impl IntoTokens for Errors {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		let Some(highest_priority) = self.errors.iter().map(|error| error.priority).max() else {
			return;
		};

		for error in self.errors {
			if error.priority == highest_priority {
				error.into_tokens(root, tokens);
			}
		}
	}
}

pub struct Input {
	pub tokens: VecDeque<TokenTree>,
	pub end: Span,
}

impl Input {
	pub fn pop_or_replace<'a, T, const N: usize>(
		&'a mut self,
		f: impl FnOnce([TokenTree; N]) -> Result<T, [TokenTree; N]>,
	) -> Result<T, impl 'a + IntoIterator<Item = Span>> {
		// This is optimisable to be essentially a no-op iff `Err`.
		//TODO: Handle none-delimiter groups.
		if self.tokens.len() < N {
			Err(self
				.tokens
				.iter()
				.map(|t| t.span())
				.chain(iter::once(self.end))
				.collect::<Vec<_>>())
		} else {
			match f([(); N].map(|()| self.tokens.pop_front().expect("unreachable"))) {
				Ok(value) => Ok(value),
				Err(ts) => {
					let spans = ts.iter().map(|t| t.span()).collect();
					for t in ts.into_iter().rev() {
						self.tokens.push_front(t);
					}
					Err(spans)
				}
			}
		}
	}

	pub fn is_empty(&self) -> bool {
		self.tokens.is_empty()
	}

	pub fn len(&self) -> usize {
		self.tokens.len()
	}

	pub fn front(&self) -> Option<&TokenTree> {
		self.tokens.front()
	}

	pub fn front_span(&self) -> Span {
		self.tokens.front().map(TokenTree::span).unwrap_or(self.end)
	}

	pub fn push_front(&mut self, t: TokenTree) {
		self.tokens.push_front(t)
	}
}

pub trait PopFrom {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized;

	fn peek_pop_from(input: &mut Input, errors: &mut Errors) -> Result<Option<Self>, ()>
	where
		Self: PeekFrom + Sized,
	{
		Option::<Self>::pop_from(input, errors)
	}
}

impl PopFrom for TokenTree {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input.pop_or_replace(|[t]| Ok(t)).map_err(|spans| {
			errors.push(Error::new(ErrorPriority::TOKEN, "Expected token.", spans))
		})
	}
}

impl<T: PeekFrom + PopFrom> PopFrom for Option<T> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		T::peek_from(input)
			.then(|| T::pop_from(input, errors))
			.transpose()
	}
}

pub trait PeekFrom {
	fn peek_from(input: &Input) -> bool;
}

impl PeekFrom for TokenStream {
	fn peek_from(_input: &Input) -> bool {
		true
	}
}

/// **Always** succeeds.
impl<T> PeekFrom for Vec<T> {
	fn peek_from(_input: &Input) -> bool {
		true
	}
}

const _: () = {
	use std::collections::VecDeque;

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
	{
		fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
			let mut this = T::default();
			while !input.is_empty() {
				let before_len = input.len();

				match T::Item::pop_from(input, errors) {
					Ok(item) => this.extend([item]),
					Err(()) => {
						EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors).ok();
						return Ok(this);
					}
				}

				if input.len() == before_len {
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
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		let value = T::pop_from(input, errors);
		EndOfInput::<UNCONSUMED_INPUT>::pop_from(input, errors).ok();
		Ok(Self(value?, PhantomData))
	}
}

#[derive(Debug)]
pub struct EndOfInput<P: ConstErrorPriority>(PhantomData<P>);

impl<P: ConstErrorPriority> PopFrom for EndOfInput<P> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.is_empty()
			.then_some(Self(PhantomData))
			.ok_or_else(|| {
				let rest = input.tokens.iter().cloned().collect::<TokenStream>();
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
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self(T::pop_from(input, errors).unwrap_or_default()))
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

impl<T: SimpleSpanned> SimpleSpanned for &T {
	fn span(&self) -> Span {
		(*self).span()
	}
}

pub trait SpanOrFrontOfExt {
	fn span_or_front_of(&self, input: &Input) -> Span;
}

impl<T: SimpleSpanned> SpanOrFrontOfExt for Option<T> {
	fn span_or_front_of(&self, input: &Input) -> Span {
		self.as_ref()
			.map(|t| t.span())
			.unwrap_or(input.front_span())
	}
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

#[macro_export]
macro_rules! grammar {
	{
		$(#[$($attr:tt)*])*
		$vis:vis enum $name:ident$(: $(
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {
			$($variant:ident($($type:ty),*$(,)?)),*$(,)?
		} else $error:expr;

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis enum $name {
			$($variant($($type),*),)*
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopFrom for $name {
			fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::___::Result<Self, ()> {
				$crate::___::Result::Ok($(if let Some(values) = ($(<$type as $crate::PopFrom>::peek_pop_from(input, errors)?),*) {
					Self::$variant(values)
				} else)* {
					return $crate::___::Result::Err(errors.push($crate::Error::new(
						$crate::ErrorPriority::GRAMMAR,
						$error,
						[input.front_span()],
					)));
				})
			}
		}

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::___::TokenStream, tokens: &mut impl $crate::___::Extend<$crate::___::TokenTree>) {
				match self {
					$(Self::$variant(value) => $crate::IntoTokens::into_tokens(value, root, tokens),)*
				}
			}
		}

		$crate::grammar!($($tt)*);
	};
	{
		$(#[$($attr:tt)*])*
		$vis:vis struct $name:ident$(: $(
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {
			$($field_vis:vis $field:ident: $type:ty),*$(,)?
		}

		$($tt:tt)*
	} => {
		$vis struct $name {
			$($field_vis $field: $type,)*
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopFrom for $name {
			fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::___::Result<Self, ()> {
				$crate::___::Result::Ok(Self {
					$($field: <$type as $crate::PopFrom>::pop_from(input, errors)?,)*
				})
			}
		}

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::___::TokenStream, tokens: &mut impl $crate::___::Extend<$crate::___::TokenTree>) {
				let Self {
					$($field,)*
				} = self;
				$($crate::IntoTokens::into_tokens($field, root, tokens);)*
			}
		}

		$crate::grammar!($($tt)*);
	};
	{$t:tt $($tt:tt)*} => {
		// Error
		::core::compile_error!($crate::___::concat!("Unexpected grammar input: ", $crate::___::stringify!($t $($tt)*)));
	};
	{} => {}; // Stop.
}

#[doc(hidden)]
pub mod ___ {
	pub use core::{concat, iter::Extend, result::Result, stringify};
	pub use proc_macro2::{TokenStream, TokenTree};
}
