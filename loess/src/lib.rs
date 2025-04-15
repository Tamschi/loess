//! <details><summary>README / Quick Start (click to expand)</summary>
//!
#![doc = include_str!("../README.md")]
//!
//! </details>
//! TODO

#![warn(clippy::pedantic, missing_docs)]

use std::{collections::VecDeque, fmt::Debug, iter, marker::PhantomData};

use error_priorities::UNCONSUMED_AFTER_REPEATS;
use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::quote_spanned;

mod proc_macro2_impls;

pub mod rust_grammar;

/// A [`Span`]-located proc macro error with [`ErrorPriority`].  
/// Usually submitted through [`Errors::push`].
///
/// Opaque, but can be expanded into [`compile_error!`] through [`IntoTokens`].
#[derive(Debug, Clone)]
pub struct Error {
	priority: ErrorPriority,
	message: String,
	spans: Vec<Span>,
}

impl Error {
	#[allow(missing_docs)]
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

/// Emits [`compile_error!`].
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

#[derive(Debug, Clone)]
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

	pub const PANIC: Self = Self::new(0.);
	pub const TOKEN: Self = Self::new(0.);
	pub const GRAMMAR: Self = Self::new(0.);
	pub const UNCONSUMED_AFTER_REPEATS: Self = Self::new(-1.);
	pub const UNCONSUMED_IN_DELIMITER: Self = Self::new(-2.);
	pub const UNCONSUMED_INPUT: Self = Self::new(-3.);
}

/// [`ConstErrorPriority`] types for use with [`Exhaustive`] and [`EndOfInput`].
pub mod error_priorities {
	#![allow(non_camel_case_types)]

	use crate::{ConstErrorPriority, ErrorPriority};

	/// [`ErrorPriority::PANIC`]
	#[derive(Clone)]
	pub enum PANIC {}
	impl ConstErrorPriority for PANIC {
		const PRIORITY: ErrorPriority = ErrorPriority::TOKEN;
	}

	/// [`ErrorPriority::TOKEN`]
	#[derive(Clone)]
	pub enum TOKEN {}
	impl ConstErrorPriority for TOKEN {
		const PRIORITY: ErrorPriority = ErrorPriority::TOKEN;
	}

	/// [`ErrorPriority::GRAMMAR`]
	#[derive(Clone)]
	pub enum GRAMMAR {}
	impl ConstErrorPriority for GRAMMAR {
		const PRIORITY: ErrorPriority = ErrorPriority::GRAMMAR;
	}

	/// [`ErrorPriority::UNCONSUMED_AFTER_REPEATS`]
	#[derive(Clone)]
	pub enum UNCONSUMED_AFTER_REPEATS {}
	impl ConstErrorPriority for UNCONSUMED_AFTER_REPEATS {
		const PRIORITY: ErrorPriority = ErrorPriority::UNCONSUMED_AFTER_REPEATS;
	}

	/// [`ErrorPriority::UNCONSUMED_IN_DELIMITER`]
	#[derive(Clone)]
	pub enum UNCONSUMED_IN_DELIMITER {}
	impl ConstErrorPriority for UNCONSUMED_IN_DELIMITER {
		const PRIORITY: ErrorPriority = ErrorPriority::UNCONSUMED_IN_DELIMITER;
	}

	/// [`ErrorPriority::UNCONSUMED_INPUT`]
	#[derive(Clone)]
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

/// Input [`tokens`](`Input::tokens`) with [`end`](`Input::end`)-[`Span`].  
/// For use with [`PeekFrom`] and [`PopFrom`].
///
/// Also has some convenience methods.
#[derive(Clone)]
pub struct Input {
	#[allow(missing_docs)]
	pub tokens: VecDeque<TokenTree>,
	/// This currently is usually a "one-token-past-the-end"-[`Span`].  
	/// Top-level input should default to [`Span::call_site()`].
	///
	/// If [`Span::end`] is stabilised, then that will be a better option and should be used instead where applicable.
	///
	/// [`Span::end`]: https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.end
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

	#[allow(missing_docs)]
	pub fn is_empty(&self) -> bool {
		self.tokens.is_empty()
	}

	#[allow(missing_docs)]
	pub fn len(&self) -> usize {
		self.tokens.len()
	}

	#[allow(missing_docs)]
	pub fn front(&self) -> Option<&TokenTree> {
		self.tokens.front()
	}

	/// Returns the frontmost token's [`Span`] or else [`self.end`](`Input::end`).
	pub fn front_span(&self) -> Span {
		self.tokens.front().map(TokenTree::span).unwrap_or(self.end)
	}

	#[allow(missing_docs)]
	pub fn push_front(&mut self, t: TokenTree) {
		self.tokens.push_front(t)
	}

	#[allow(missing_docs)]
	pub fn prepend(
		&mut self,
		tokens: impl IntoIterator<Item = TokenTree, IntoIter: DoubleEndedIterator>,
	) {
		for t in tokens.into_iter().rev() {
			self.push_front(t);
		}
	}
}

/// Consumes from [`Input`] to create <code>[`Result`]&lt;Self, ()></code> and emit to [`Errors`].
pub trait PopFrom {
	/// Tries to parse `Self` from an [`Input`], optionally emitting to [`Errors`].
	///
	/// # Returns
	///
	/// ## <code>[`Ok`]::&lt;Self, _></code>
	///
	/// Parsing either succeeded or its failure was recoverable.
	///
	/// There <em style=font-style:normal;font-variant:small-caps>may</em> be new [`Errors`]!
	///
	/// ## <code>[`Err`]::&lt;_, ()></code>
	///
	/// Parsing failed unrecoverably.
	///
	/// It <em style=font-style:normal;font-variant:small-caps>may</em> still be recovered further up the call chain,
	/// but there <em style=font-style:normal;font-variant:small-caps>should</em> be new [`Errors`] at this point!
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized;

	/// Convenience function for <code>&lt;[`Option`]&lt;Self> as [`PopFrom`]>::[pop_from](`PopFrom::pop_from`)</code>.
	///
	/// This is used by [`grammar!`]-generated enum parsers.
	fn peek_pop_from(input: &mut Input, errors: &mut Errors) -> Result<Option<Self>, ()>
	where
		Self: PeekFrom + Sized,
	{
		Option::<Self>::pop_from(input, errors)
	}
}

impl<T: PopFrom> PopFrom for Box<T> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		Ok(Box::new(T::pop_from(input, errors)?))
	}
}

impl<T: IntoTokens> IntoTokens for Box<T> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		(*self).into_tokens(root, tokens)
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

/// Determines if `Self` may be be parseable from an [`Input`].  
/// This is often a cursory check!
///
/// Used for variant selection in <code>&lt;[`Option`]&lt;Self> as [`PopFrom`]>::[pop_from](`PopFrom::pop_from`)</code>.  
/// Does **not** affect <code>[`Vec`]&lt;Self></code> or <code>[`VecDeque`]&lt;Self></code> parsing, which is exhaustive.
///
/// Also enables [`PopFrom::peek_pop_from`] for `Self`, which is used in [`grammar!`]-generated enum parsers.
pub trait PeekFrom {
	/// # Returns
	///
	/// ## [`true`]
	///
	/// [`PopFrom::pop_from`] <em style=font-style:normal;font-variant:small-caps>may</em> still fail and/or push to [`Errors`].
	///
	/// ## [`false`]
	///
	/// [`PopFrom::pop_from`] <em style=font-style:normal;font-variant:small-caps>should</em> fail **and** push to [`Errors`].
	fn peek_from(input: &Input) -> bool;
}

/// **Always** succeeds.
impl<T> PeekFrom for Option<T> {
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

/// **Always** succeeds.
impl<T> PeekFrom for VecDeque<T> {
	fn peek_from(_input: &Input) -> bool {
		true
	}
}

const _: () = {
	use std::collections::VecDeque;

	use crate::{EndOfInput, Errors, PopFrom};

	impl<T: PopFrom> PopFrom for Vec<T> {
		fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
			let mut this = vec![];
			while !input.is_empty() {
				let before_len = input.len();

				match T::pop_from(input, errors) {
					Ok(item) => this.extend([item]),
					Err(()) => {
						EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors).ok();
						return Ok(this);
					}
				}

				if input.len() == before_len {
					assert!(
						EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors).is_err()
					);
					break;
				}
			}

			Ok(this)
		}
	}

	impl<T: PopFrom> PopFrom for VecDeque<T> {
		fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
			let mut this = Self::default();
			while !input.is_empty() {
				let before_len = input.len();

				match T::pop_from(input, errors) {
					Ok(item) => this.extend([item]),
					Err(()) => {
						EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors).ok();
						return Ok(this);
					}
				}

				if input.len() == before_len {
					assert!(
						EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_from(input, errors).is_err()
					);
					break;
				}
			}

			Ok(this)
		}
	}
};

/// Doesn't fail to parse but emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`] after `T`.
#[derive(Clone)]
pub struct Exhaustive<T, P: ConstErrorPriority>(pub T, PhantomData<P>);

impl<T: PopFrom, P: ConstErrorPriority> PopFrom for Exhaustive<T, P> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		let value = T::pop_from(input, errors);
		EndOfInput::<P>::pop_from(input, errors).ok();
		Ok(Self(value?, PhantomData))
	}
}

impl<T: IntoTokens, P: ConstErrorPriority> IntoTokens for Exhaustive<T, P> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

/// Fails to parse and emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`].
#[derive(Clone)]
pub struct EndOfInput<P: ConstErrorPriority>(PhantomData<P>);

/// Fails iff the [`Input`] isn't empty.
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

/// Has a single [`Span`].
pub trait SimpleSpanned {
	#[allow(missing_docs)]
	fn span(&self) -> Span;

	#[allow(missing_docs)]
	fn set_span(&mut self, span: Span);

	#[allow(missing_docs)]
	fn with_span(mut self, span: Span) -> Self
	where
		Self: Sized,
	{
		self.set_span(span);
		self
	}
}

/// ```
/// use loess::{
/// 	grammar,
/// 	rust_grammar::{Identifier, Parentheses, SquareBrackets, Visibility},
/// };
/// use proc_macro2::TokenTree;
///
/// grammar! {
/// 	///
/// 	/// Has auto-documented grammar.
/// 	#[derive(Clone)]
/// 	pub enum Alternatives: doc, PeekFrom, PopFrom, IntoTokens {
/// 		Identifier(Identifier),
/// 		Paren(Parentheses), // Can be used as generic too.
/// 		Bracket(SquareBrackets<Vec<TokenTree>>),
/// 		Vis(Option<Visibility>), // Must always be last, as peeking `Option` always succeeds.
/// 	} else "Expected Alternative.";
/// }
/// ```
#[macro_export]
macro_rules! grammar {
	{
		$(#[$($attr:tt)*])*
		$vis:vis enum $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {
			$($variant:ident($($type:ty),*$(,)?)),*$(,)?
		} else $error:expr;

		$($tt:tt)*
	} => {
		#[cfg_attr(any($($($(all(), $(@ $doc)?)?)?)*), doc = $crate::grammar!(@enum_doc [$([$($type,)*])*]))]
		$(#[$($attr)*])*
		$vis enum $name {
			$($variant($($type),*),)*
		}

		#[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				false
				$(|| $crate::grammar!(@peek_first $name input $($type,)*))*
			}
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopFrom for $name {
			fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				$crate::__::Result::Ok($(if let Some(values) = ($(<$type as $crate::PopFrom>::peek_pop_from(input, errors)?),*) {
					Self::$variant(values)
				} else)* {
					return $crate::__::Result::Err(errors.push($crate::Error::new(
						$crate::ErrorPriority::GRAMMAR,
						$error,
						[input.front_span()],
					)));
				})
			}
		}

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
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
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {
			$($field_vis:vis $field:ident: $type:ty),*$(,)?
		}

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis struct $name {
			$($field_vis $field: $type,)*
		}

		#[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				$crate::grammar!(@peek_first $name input $($type,)*)
			}
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopFrom for $name {
			fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				$crate::__::Result::Ok(Self {
					$($field: <$type as $crate::PopFrom>::pop_from(input, errors)?,)*
				})
			}
		}

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				let Self {
					$($field,)*
				} = self;
				$($crate::IntoTokens::into_tokens($field, root, tokens);)*
			}
		}

		$crate::grammar!($($tt)*);
	};
	{
		$(#[$($attr:tt)*])*
		$vis:vis struct $name:ident$(: $(
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
		),*)? (
			$($field_vis:vis $type:ty),*$(,)?
		);

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis struct $name (
			$($field_vis $type,)*
		);

		#[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				$crate::grammar!(@peek_first $name input $($type,)*)
			}
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopFrom for $name {
			fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				$crate::__::Result::Ok(Self (
					$(<$type as $crate::PopFrom>::pop_from(input, errors)?,)*
				))
			}
		}

		$crate::grammar!($($tt)*);
	};
	(@peek_first $name:ident $input:ident $type:ty, $($rest:ty,)*) => (
		<$type as $crate::PeekFrom>::peek_from($input)
	);
	(@peek_first $name:ident $input:ident) => (
		::core::compile_error!($crate::__::concat!("To implement `PeekFrom` for `", $crate::__::stringify!($name), "`, at least one field is necessary."))
	);
	(@enum_doc []) => (
		// Empty.
		""
	);
	(@enum_doc [[$($type0:ty,)*] $([$($type:ty,)*])*]) => (
		// Start.
		$crate::grammar!(@enum_doc [$([$($type,)*])*] [$("[`", $crate::__::stringify!($type0), "`] ", )*])
	);
	(@enum_doc [[$($type0:ty,)*] $([$($type:ty,)*])*] [$($output:tt)*]) => (
		// Continue.
		$crate::grammar!(@enum_doc [$([$($type,)*])*] [$($output)* "| ", $("[`", $crate::__::stringify!($type0), "`] ", )*])
	);
	(@enum_doc [] [$($output:tt)*]) => (
		// End.
		$crate::__::concat!($($output)*)
	);
	{$t:tt $($tt:tt)*} => {
		// Error
		::core::compile_error!($crate::__::concat!("Unexpected grammar input: ", $crate::__::stringify!($t $($tt)*)));
	};
	{} => {}; // Stop.
}

#[doc(hidden)]
pub mod __ {
	pub use core::{concat, iter::Extend, primitive::bool, result::Result, stringify};
	pub use proc_macro2::{TokenStream, TokenTree};
}

pub struct HandledPanic;
