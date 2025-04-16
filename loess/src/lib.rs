//! <details><summary>README / Example (click to expand)</summary>
//!
#![doc = include_str!("../README.md")]
//!
//! </details>
//!
//! In most cases you'll want to:
//!
//! 1. generate grammar implementations with [`grammar!`] (You can also easily implement parts manually.),
//! 2. step through the input with [`parse_once`], [`parse_once_with`] and/or [`parse_once_with_infallible`] and
//! 3. consume the last of the input with [`parse_all`], [`parse_all_with`] or [`parse_all_with_infallible`].
//!
//! You can call either [`Iterator::collect`] (for repeats) or [`Iterator::next`] (for one value) on the last step.
//!
//! # Features
//!
//! None are default, as DSL macros might not need Rust's grammar at all.
//!
//! ## `"rust_grammar"`
//!
//! Enables [`rust_grammar`].
//!
//! ## `"opaque_rust_grammar"` <sub>enables `"rust_grammar"`, depends on `syn`</sub>
//!
//! Adds additional opaque Rust grammar DTOs, to consume, paste and clone for example
//! Statements and Patterns.
//!
//! These preliminary implementations are [Syn](https://docs.rs/syn)-based and can't be inspected.

#![warn(clippy::pedantic, missing_docs)]

use std::{
	any::Any,
	collections::VecDeque,
	fmt::Debug,
	iter,
	marker::PhantomData,
	panic::{AssertUnwindSafe, UnwindSafe, catch_unwind},
};

use error_priorities::{UNCONSUMED_AFTER_REPEATS, UNCONSUMED_INPUT};
use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::quote_spanned;

mod proc_macro2_impls;

#[cfg(any(doc, feature = "rust_grammar"))]
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
/// **Expects `root` to re-export [`core`] if not empty.**
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

/// A collection of [`Error`]s submitted during e.g. parsing with [`PopFrom`].
///
/// Only the set of [`Error`]s with the highest [`ErrorPriority`] is pasted as [`compile_error!`]s through [`IntoTokens`].
#[derive(Debug, Clone)]
pub struct Errors {
	errors: Vec<Error>,
}

/// An opaque [`Error`] priority.
///
/// To reduce noise from cascading errors within the generated parser,
/// only the [`Error`]s with the respective highest priority are pasted by [`Errors`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ErrorPriority(f64);

/// [`ErrorPriority`] as generic type argument.
pub trait ConstErrorPriority {
	#[allow(missing_docs)]
	const PRIORITY: ErrorPriority;
}

impl ErrorPriority {
	const fn new(value: f64) -> Self {
		assert!(!value.is_nan());
		Self(value)
	}

	/// Constructs a priority ever so slightly higher than `self`.
	///
	/// [`Error`]s with that priority *hide* [`Error`]s with `self` when [`Errors`] is pasted.
	pub const fn next_higher(&self) -> Self {
		Self(self.0.next_up())
	}

	/// Constructs a priority ever so slightly higher than `self`.
	///
	/// [`Error`]s with that priority *are hidden by* [`Error`]s with `self` when [`Errors`] is pasted.
	pub const fn next_lower(&self) -> Self {
		Self(self.0.next_down())
	}

	#[allow(missing_docs)]
	pub const PANIC: Self = Self::new(0.);
	#[allow(missing_docs)]
	pub const TOKEN: Self = Self::new(0.);
	#[allow(missing_docs)]
	pub const GRAMMAR: Self = Self::new(0.);
	#[allow(missing_docs)]
	pub const UNCONSUMED_AFTER_REPEATS: Self = Self::new(-1.);
	#[allow(missing_docs)]
	pub const UNCONSUMED_IN_DELIMITER: Self = Self::new(-2.);
	#[allow(missing_docs)]
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
	#[allow(missing_docs)]
	pub fn new() -> Self {
		Self { errors: vec![] }
	}

	#[allow(missing_docs)]
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

/// Emits [`compile_error!`]s, but oly those with the highest [`ErrorPriority`].  
/// **Expects `root` to re-export [`core`] if not empty.**
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
	/// Convenience method to match against an array of [`TokenTree`]s.
	///
	/// This is mostly for [`PopFrom`] implementations.  
	/// Grammar consumers should call <code>Token::[pop_from](`PopFrom::pop_from`)</code> instead.
	///
	/// # Returns
	///
	/// If `self` is too short or `f` returns [`Err`],
	/// respective [`Span`]s to create an [`Error`] with.
	///
	/// Iff `self` is too short, <code>self.[end](`Input::end`)</code> is included.
	pub fn pop_or_replace<'a, T, const N: usize>(
		&'a mut self,
		f: impl FnOnce([TokenTree; N]) -> Result<T, [TokenTree; N]>,
	) -> Result<T, impl 'a + IntoIterator<Item = Span>> {
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
				Err(tts) => {
					let spans = tts.iter().map(|t| t.span()).collect();
					self.prepend(tts);
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
///
/// Intentionally not implemented for [`Option`], as it would always match, which is too error-prone.
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

/// Succeeds if input is empty, otherwise peeks `T`.
impl<T: PeekFrom> PeekFrom for Vec<T> {
	fn peek_from(input: &Input) -> bool {
		input.is_empty() || T::peek_from(input)
	}
}

/// Succeeds if input is empty, otherwise peeks `T`.
impl<T: PeekFrom> PeekFrom for VecDeque<T> {
	fn peek_from(input: &Input) -> bool {
		input.is_empty() || T::peek_from(input)
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

/// Parser- and printer-generator macro.
///
/// ```
/// use loess::{
/// 	grammar,
/// 	rust_grammar::{ // With the `"rust_grammar"` feature.
/// 		Identifier, Let, Parentheses, SquareBrackets, Visibility,
/// 	},
/// };
/// use proc_macro2::{Ident, TokenTree, Punct};
///
/// grammar! {
/// 	///
/// 	/// Has auto-documented grammar.
/// 	#[derive(Clone)]
/// 	pub enum Alternatives: doc, PeekFrom, PopFrom, IntoTokens {
/// 		Identifier(Identifier),
/// 		Paren(Parentheses),
/// 		Bracket(SquareBrackets<Vec<TokenTree>>),
/// 		Vis(Visibility),
/// 	} else "Expected Alternative.";
///
/// 	#[derive(Clone)]
/// 	/// `visibility` can't be first, as `Option` isn't `PeekFrom`.
/// 	/// However, `Visibility` itself is `PeekFrom` (checking for `pub`).
/// 	///
/// 	/// Fields are parsed and emitted in order.
/// 	pub struct StructuredSequence: PeekFrom, PopFrom, IntoTokens {
/// 		pub r#let: Let,
/// 		pub visibility: Option<Visibility>,
/// 		pub paren_ident: Parentheses<Ident>,
/// 		pub vec_punct: Vec<Punct>,
/// 	}
///
/// 	#[derive(Clone)]
/// 	/// Generated implementations for tuple structs are currently the most limited.
/// 	pub struct TupleSequence: PeekFrom, PopFrom (
/// 		pub Let,
/// 		pub Option<StructuredSequence>,
/// 		pub Parentheses<Ident>,
/// 		pub Vec<Punct>,
/// 	);
/// }
/// ```
///
/// [`grammar!`] is fully hygienic and uses `$crate`, so can rename dependencies freely.
#[macro_export]
macro_rules! grammar {
	{
		$(#[$($attr:tt)*])*
		$vis:vis enum $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {$(
			$(#[$($variant_attr:tt)*])*
			$variant:ident($($type:ty),*$(,)?)
		),*$(,)?} else $error:expr;

		$($tt:tt)*
	} => {
		#[cfg_attr(any($($($(all(), $(@ $doc)?)?)?)*), doc = $crate::grammar!(@enum_doc [$([$($type,)*])*]))]
		$(#[$($attr)*])*
		$vis enum $name {$(
			$(#[$($variant_attr)*])*
			$variant($($type),*),
		)*}

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
		),*)? {$(
			$(#[$($field_attr:tt)*])*
			$field_vis:vis $field:ident: $type:ty
		),*$(,)?}

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis struct $name {$(
			$(#[$($field_attr)*])*
			$field_vis $field: $type,
		)*}

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
		),*)? ($(
			$(#[$($field_attr:tt)*])*
			$field_vis:vis $type:ty
		),*$(,)?);

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis struct $name ($(
			$(#[$($field_attr)*])*
			$field_vis $type,
		)*);

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

/// A substitute panic that isn't reported as [`Error`]. **(Read for panic handling info!)**
///
/// Loess intercepts [`String`] and <code>&'static [str]</code> panics in group tokens to
/// report their message via [`Error`] on the locally frontmost [`Input`] token instead.
///
/// In order to avoid duplicate reporting, [`HandledPanic`] is substituted when unwinding
/// is resumed. This type can be detected and ignored by further panic handlers on the
/// call stack.
///
/// To catch the top-level unwind and report panics from outside any groups, you can use
/// one of [`parse_all`], [`parse_all_with`] and [`parse_all_with_infallible`] or
/// [`parse_once`], [`parse_once_with`] and [`parse_once_with_infallible`],
/// either in order of decreasing convenience.
pub struct HandledPanic;

/// Low-level non-repeating unwind-catcher that reports panics to the given [`Errors`],
/// located at <code>input.[front_span()](`Input::front_span`)</code> at the time.
///
/// Does **not** check for unconsumed [`Input`]! To parse the last part of the input, use
/// <code>[parse_all_with_infallible](input, errors, f).[next()](`Iterator::next`)</code> instead.
pub fn parse_once_with_infallible<'a, T>(
	input: &'a mut Input,
	errors: &'a mut Errors,
	f: impl 'a + UnwindSafe + FnOnce(&mut Input, &mut Errors) -> T,
) -> Result<T, ()> {
	parse_once_with_infallible_impl(input, errors, f)
}

/// Because [`AssertUnwind`] apparently doesn't forward higher-order [`FnOnce`] implementations.
fn parse_once_with_infallible_impl<'a, T>(
	input: &mut Input,
	errors: &mut Errors,
	f: impl 'a + FnOnce(&mut Input, &mut Errors) -> T,
) -> Result<T, ()> {
	fn handle_panic(input: &mut Input, errors: &mut Errors, panic: Box<dyn Any + Send>) {
		errors.push(Error::new(
			ErrorPriority::PANIC,
			&format!(
				"proc macro panicked: {:?}",
				if panic.as_ref().is::<HandledPanic>() {
					return;
				} else if let Some(message) = panic.as_ref().downcast_ref::<String>() {
					message.clone()
				} else if let Some(message) = panic.as_ref().downcast_ref::<&'static str>() {
					message.to_string()
				} else {
					return errors.push(Error::new(
						ErrorPriority::PANIC,
						"proc macro panicked",
						[input.front_span()],
					));
				}
			),
			[input.front_span()],
		))
	}

	catch_unwind(AssertUnwindSafe(|| f(input, errors))).map_err(|panic| {
		handle_panic(input, errors, panic);
	})
}

/// Non-repeating unwind-catcher that reports panics to the given [`Errors`],
/// located at <code>input.[front_span()](`Input::front_span`)</code> at the time.
///
/// Does **not** check for unconsumed [`Input`]! To parse the last part of the input, use
/// <code>[parse_all_with](input, errors, f).[next()](`Iterator::next`)</code> instead.
pub fn parse_once_with<'a, T>(
	input: &'a mut Input,
	errors: &'a mut Errors,
	f: impl 'a + UnwindSafe + FnOnce(&mut Input, &mut Errors) -> Result<T, ()>,
) -> Result<T, ()> {
	parse_once_with_impl(input, errors, f)
}

/// Because [`AssertUnwind`] apparently doesn't forward higher-order [`FnOnce`] implementations.
fn parse_once_with_impl<'a, T>(
	input: &mut Input,
	errors: &mut Errors,
	f: impl 'a + FnOnce(&mut Input, &mut Errors) -> Result<T, ()>,
) -> Result<T, ()> {
	match parse_once_with_infallible_impl(input, errors, f) {
		Ok(ok) => ok,
		Err(()) => Err(()),
	}
}

/// Convenient non-repeating unwind-catcher that reports panics to the given [`Errors`],
/// located at <code>input.[front_span()](`Input::front_span`)</code> at the time.
///
/// Does **not** check for unconsumed [`Input`]! To parse the last part of the input, use
/// <code>[parse_all](input, errors).[next()](`Iterator::next`)</code> instead.
pub fn parse_once<'a, T: PopFrom>(input: &'a mut Input, errors: &'a mut Errors) -> Result<T, ()> {
	parse_once_with_impl(input, errors, T::pop_from)
}

/// Conveniently parses remaining [`Input`] through `f` without catching [`Err`],
/// catching and submitting panics to the given [`Errors`]:
///
/// ```
/// use loess::{parse_all_with_infallible, Errors, Input, IntoTokens, PopFrom};
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	let tts = parse_all_with_infallible(
/// 			&mut input,
/// 			&mut errors,
/// 			|input, errors| TokenTree::pop_from(input, errors).expect("infallible"),
/// 		).collect::<Vec<_>>(); // Checks for exhaustiveness.
///
/// 	let root = TokenStream::new(); // See `IntoTokens`.
/// 	let mut output = TokenStream::new();
/// 	errors.into_tokens(&root, &mut output);
///
/// 	// Emit your output here:
/// 	tts.into_tokens(&root, &mut output);
///
/// 	output
/// }
/// ```
///
/// You can call [`.next()`](`Iterator::next`) instead of [`.collect()`](`Iterator::collect`)
/// to parse a single value exhaustively into an [`Option`]:
///
/// ```
/// use loess::{parse_all_with_infallible, Errors, Input, IntoTokens, PopFrom};
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	let tt = parse_all_with_infallible(
/// 			&mut input,
/// 			&mut errors,
/// 			|input, errors| TokenTree::pop_from(input, errors).expect("infallible"),
/// 		).next(); // Checks for exhaustiveness.
///
/// 	let root = TokenStream::new(); // See `IntoTokens`.
/// 	let mut output = TokenStream::new();
///
/// 	// Make sure to emit `errors` unconditionally,
/// 	// ideally before other output.
/// 	errors.into_tokens(&root, &mut output);
///
/// 	if let Some(tt) = tt {
/// 		// Emit your output here:
/// 		tt.into_tokens(&root, &mut output);
/// 	};
///
/// 	output
/// }
/// ```
pub fn parse_all_with_infallible<'a, T>(
	input: &'a mut Input,
	errors: &'a mut Errors,
	f: impl 'a + UnwindSafe + FnMut(&mut Input, &mut Errors) -> T,
) -> impl 'a + Iterator<Item = T> {
	parse_all_with_infallible_impl(input, errors, f)
}

/// Because [`AssertUnwind`] apparently doesn't forward higher-order [`FnOnce`] implementations.
fn parse_all_with_infallible_impl<'a, T>(
	input: &'a mut Input,
	errors: &'a mut Errors,
	f: impl 'a + UnwindSafe + FnMut(&mut Input, &mut Errors) -> T,
) -> impl 'a + Iterator<Item = T> {
	struct Iter<'a, F> {
		input: &'a mut Input,
		errors: &'a mut Errors,
		f: F,
	}

	impl<'a, T, F: 'a + UnwindSafe + FnMut(&mut Input, &mut Errors) -> T> Iterator for Iter<'a, F> {
		type Item = T;

		fn next(&mut self) -> Option<Self::Item> {
			if self.input.is_empty() {
				None
			} else {
				match parse_once_with_infallible_impl(self.input, self.errors, &mut self.f) {
					Ok(ok) => Some(ok),
					Err(()) => None,
				}
			}
		}
	}

	impl<'a, F> Drop for Iter<'a, F> {
		/// [`Iter`] borrows the [`Errors`] exclusively, so this will be called before that's turned into output.
		fn drop(&mut self) {
			EndOfInput::<UNCONSUMED_INPUT>::pop_from(self.input, self.errors).ok();
		}
	}

	Iter { input, errors, f }
}

/// Conveniently parses remaining [`Input`] through `f`,
/// catching and submitting panics to the given [`Errors`]:
///
/// ```
/// use loess::{parse_all_with, Errors, Input, IntoTokens, PopFrom};
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	let tts = parse_all_with(&mut input, &mut errors, TokenTree::pop_from)
/// 		.collect::<Vec<_>>(); // Checks for exhaustiveness.
///
/// 	let root = TokenStream::new(); // See `IntoTokens`.
/// 	let mut output = TokenStream::new();
/// 	errors.into_tokens(&root, &mut output);
///
/// 	// Emit your output here:
/// 	tts.into_tokens(&root, &mut output);
///
/// 	output
/// }
/// ```
///
/// You can call [`.next()`](`Iterator::next`) instead of [`.collect()`](`Iterator::collect`)
/// to parse a single value exhaustively into an [`Option`]:
///
/// ```
/// use loess::{parse_all_with, Errors, Input, IntoTokens, PopFrom};
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	let tt = parse_all_with(&mut input, &mut errors, TokenTree::pop_from)
/// 		.next(); // Checks for exhaustiveness.
///
/// 	let root = TokenStream::new(); // See `IntoTokens`.
/// 	let mut output = TokenStream::new();
///
/// 	// Make sure to emit `errors` unconditionally,
/// 	// ideally before other output.
/// 	errors.into_tokens(&root, &mut output);
///
/// 	if let Some(tt) = tt {
/// 		// Emit your output here:
/// 		tt.into_tokens(&root, &mut output);
/// 	};
///
/// 	output
/// }
/// ```
pub fn parse_all_with<'a, T: 'a>(
	input: &'a mut Input,
	errors: &'a mut Errors,
	f: impl 'a + UnwindSafe + FnMut(&mut Input, &mut Errors) -> Result<T, ()>,
) -> impl 'a + Iterator<Item = T> {
	parse_all_with_infallible_impl(input, errors, f).map_while(|item| match item {
		Ok(ok) => Some(ok),
		Err(()) => None,
	})
}

/// Conveniently parses remaining [`Input`] through [`PopFrom`],
/// catching and submitting panics to the given [`Errors`]:
///
/// ```
/// use loess::{parse_all, Errors, Input, IntoTokens};
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	let tts: Vec<TokenTree> = parse_all(&mut input, &mut errors)
/// 		.collect(); // Checks for exhaustiveness.
///
/// 	let root = TokenStream::new(); // See `IntoTokens`.
/// 	let mut output = TokenStream::new();
/// 	errors.into_tokens(&root, &mut output);
///
/// 	// Emit your output here:
/// 	tts.into_tokens(&root, &mut output);
///
/// 	output
/// }
/// ```
///
/// You can call [`.next()`](`Iterator::next`) instead of [`.collect()`](`Iterator::collect`)
/// to parse a single value exhaustively into an [`Option`]:
///
/// ```
/// use loess::{parse_all, Errors, Input, IntoTokens};
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	let tt: Option<TokenTree> = parse_all(&mut input, &mut errors)
/// 		.next(); // Checks for exhaustiveness.
///
/// 	let root = TokenStream::new(); // See `IntoTokens`.
/// 	let mut output = TokenStream::new();
///
/// 	// Make sure to emit `errors` unconditionally,
/// 	// ideally before other output.
/// 	errors.into_tokens(&root, &mut output);
///
/// 	if let Some(tt) = tt {
/// 		// Emit your output here:
/// 		tt.into_tokens(&root, &mut output);
/// 	};
///
/// 	output
/// }
/// ```
pub fn parse_all<'a, T: 'a + PopFrom>(
	input: &'a mut Input,
	errors: &'a mut Errors,
) -> impl 'a + Iterator<Item = T> {
	parse_all_with(input, errors, T::pop_from)
}
