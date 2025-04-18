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
//! Adds additional opaque Rust grammar tokens, to consume, paste and clone for example
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
	mem,
	panic::{AssertUnwindSafe, UnwindSafe, catch_unwind},
};

use error_priorities::{UNCONSUMED_AFTER_REPEATS, UNCONSUMED_INPUT};
use proc_macro2::{Literal, Span, TokenStream, TokenTree};

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

		quote_into_same_site! (span, root, tokens, [
			//TODO: Use `{#error … }`.
			{#paste root.clone()}{#raw ::core::compile_error!}({#paste message});
		]);
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

/// Spreads `self` into its contained or representative [`TokenTree`]s.
///
/// This trait's methods accept a <code>root: &[TokenStream]</code> (which may be empty).
/// This `root` <em style=font-style:normal;font-variant:small-caps>should</em> be emitted
/// before each fully qualified path, which helps wrapped macros be consumer-dependency independent:
///
/// ```
/// // wrapper crate
///
/// #[macro_export]
/// macro_rules! my_macro {
/// 	($($tt:tt)*) => ( $crate::__::my_macro!([$crate] $($tt)*) );
/// }
///
/// #[doc(hidden)]
/// pub mod __ {
/// 	pub use core; // Expected by `Errors`.
/// 	pub use my_macro_impl::my_macro;
///
/// 	# /// Ignore this! It's here just to make this test compile.
/// 	# mod my_macro_impl { pub use crate::my_macro as my_macro; }
/// }
/// ```
///
/// ```
/// // my_macro_impl (proc macro)
///
/// use loess::{
/// 	grammar, parse_once, parse_all,
/// 	Errors, Input, IntoTokens,
/// 	rust_grammar::{SquareBrackets},
/// };
/// use proc_macro2::{Span, TokenStream, TokenTree};
///
/// // […]
///
/// fn macro_impl(input: TokenStream) -> TokenStream {
/// 	let mut input = Input {
/// 		tokens: input.into_iter().collect(),
/// 		end: Span::call_site(),
/// 	};
/// 	let mut errors = Errors::new();
///
/// 	// `root` is implicitly a `TokenStream`.
/// 	let Ok(SquareBrackets { contents: root, .. }) = parse_once(
/// 			&mut input,
/// 			&mut errors,
/// 		) else { return errors.collect_tokens(&TokenStream::new()) };
///
/// 	grammar! {
/// 		/// This represents your complete input grammar.
/// 		/// This here is a placeholder, so it's empty.
/// 		struct Grammar: PopFrom {}
/// 	}
///
/// 	// Checks for exhaustiveness.
/// 	let parsed = parse_all(&mut input, &mut errors).next();
/// 	let mut output = errors.collect_tokens(&root);
///
/// 	if let Some(Grammar {}) = parsed {
/// 		// Emit your output here.
/// 	}
///
/// 	output
/// }
/// ```
pub trait IntoTokens {
	/// Emit `self`'s tokens into `tokens` while referencing `root`.
	///
	/// `root` <em style=font-style:normal;font-variant:small-caps>should</em> be prefixed to
	/// any fully qualified paths that are emitted by `self`.
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>);

	/// Convenience methods to emit `self`'s tokens into a new `T`.
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
	/// Convenience method to match an array of <code>&[TokenTree]</code>s.
	///
	/// This is mostly for [`PeekFrom`] implementations.  
	/// Grammar consumers should call <code>Token::[peek_from](`PeekFrom::peek_from`)</code> instead.
	///
	/// Iff `self` is long enough, `f` is called with references to the frontmost tokens.
	///
	/// # Returns
	///
	/// `false` if `self` is too short, otherwise the return value of `f`
	pub fn peek<'a, const N: usize>(&'a self, f: impl FnOnce([&TokenTree; N]) -> bool) -> bool {
		//TODO: Handle none-delimiter groups. (Maybe not here?)
		if self.len() < N {
			false
		} else {
			let mut iter = self.tokens.iter();
			f(std::array::from_fn(move |_| {
				iter.next().expect("due to !(self.len() < N)")
			}))
		}
	}

	/// Convenience method to match from an array of [`TokenTree`]s.
	///
	/// This is mostly for [`PopFrom`] implementations.  
	/// Grammar consumers should call <code>Token::[pop_from](`PopFrom::pop_from`)</code> instead.
	///
	/// Iff `self` is long enough, `f` is called with the frontmost tokens.
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

/// Simple quotation macro that works well with Loess's types.
///
/// Uses `{#identifier … }`-style directives.
///
/// Applies [`Span::mixed_site()`] resolution to quoted tokens, but locates them at `$span`.
///
/// ```rust
/// use loess::{quote_into_mixed_site, rust_grammar::Identifier, SimpleSpanned};
/// use proc_macro2::TokenStream;
///
/// fn my_quote(id1: Identifier, id2: Option<Identifier>, root: &TokenStream) -> TokenStream {
/// 	let mut output = TokenStream::new();
///
/// 	quote_into_mixed_site!(id1.span(), root, &mut output, [
/// 		pub struct {#paste id1};
///
/// 		{#if let Some(id2) = id2,
/// 			{#located_at id2.span(),
/// 				pub struct {#paste id2};
/// 			}
/// 		} {#else,
/// 			{#error "`id2` is missing."}
/// 		}
/// 	]);
///
/// 	output
/// }
/// ```
///
/// //TODO: Rust keywords should only be used for Rust grammar.
/// //TODO: Use `quote_into_mixed_site!`, `quote_into_call_site!`.
/// //TODO: Use `#mixed_site`, `#call_site`, `#located_at` and `#resolved_at`.
/// ## Directives
///
/// ### `{#paste $($expr:expr),*$(,)?}`
///
/// Emits each `$expr` as/through [`IntoTokens`].
///
/// ### `{#move $span:expr, $($tt:tt)* }`
///
/// Acts as nested [`quote_into!`] call with `$span` as [`Span`] argument.
///
/// ### `{#const $group:tt}` <sub>where `$group` should be a delimited group<sub>
///
/// Uses `$group` as [stringify!] argument list, parsing and emitting its contents all at once at runtime.
/// This means those contents are emitted verbatim and, in theory, a bit faster overall.
///
/// ### `{#if $expr:expr, $($tt:tt)* }`<br>`{#if let $pattern:pat = $expr:expr, $($tt:tt)* }`<br>`{#for $pattern:pat in $span:expr, $($tt:tt)* }`<br>`{#while $expr:expr, $($tt:tt)* }`<br>`{#while let $pattern:pat = $expr:expr, $($tt:tt)* }`<br>`{#loop $($tt:tt)* }`<br>`{#break $($expr:expr)? }`
///
/// Expand into flow control statements.
///
/// ### `{#else $($tt:tt)* }`
///
/// Expands into an `else`-branch.
///
/// ### `{#root}`
///
/// Pastes a clone of the `$root` given to the initial call.
///
/// ### `{#hash $($tt:tt)* }`
///
/// Emits `{#` … `}`.
#[macro_export]
macro_rules! quote_into_mixed_site {
	($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		let root: &$crate::__::TokenStream = $root;
		let tokens = $tokens;
		let mut not_if = false;
		$( $crate::__::quote_one!(span root tokens not_if, $tt); )*
	});
}

/// Like [`quote_into_mixed_site!`], but resolved according to `$span`.
#[macro_export]
macro_rules! quote_into_same_site {
	($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		let span: $crate::__::Span = $span;
		let root: &$crate::__::TokenStream = $root;
		let tokens = $tokens;
		let mut not_if = false;
		$( $crate::__::quote_one!(span root tokens not_if, $tt); )*
	});
}

/// Like [`quote_into_mixed_site!`], but resolved according to [`Span::call_site()`].
#[macro_export]
macro_rules! quote_into_call_site {
	($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		let root: &$crate::__::TokenStream = $root;
		let tokens = $tokens;
		let mut not_if = false;
		$( $crate::__::quote_one!(span root tokens not_if, $tt); )*
	});
}

//TODO: Same-site/mixed-site/call-site variants and directives.

#[doc(hidden)]
pub mod __ {
	#![allow(missing_docs)] // Internal.

	use std::str::FromStr;

	pub use core::{
		compile_error, concat, iter::Extend, primitive::bool, result::Result, stringify,
	};

	use proc_macro2::{Delimiter, Group, Punct, Spacing};

	pub use proc_macro2::{
		Delimiter::{Brace, Bracket, Parenthesis},
		Span, TokenStream, TokenTree,
	};

	pub use crate::quote_one;

	#[doc(hidden)]
	#[macro_export]
	macro_rules! quote_one {
		//TODO: Missing directives.
		//TODO: Error handling with syntax help.
		($span:tt $root:tt $tokens:tt $not_if:tt, {#paste $($expr:expr),*$(,)?}) => {
			$( $crate::IntoTokens::into_tokens($expr, $root, $tokens); )*
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#raw $($tt:tt)*}) => {
			$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#mixed_site $($tt:tt)*}) => {{
			let span = $span.resolved_at($crate::__::Span::mixed_site());
			$( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		}};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#call_site $($tt:tt)*}) => {{
			let span = $span.resolved_at($crate::__::Span::call_site());
			$( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		}};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#located_at $span2:expr, $($tt:tt)*}) => {{
			let span = $span.located_at($span2);
			$( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		}};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#resolved_at $span2:expr, $($tt:tt)*}) => {{
			let span = $span.resolved_at($span2);
			$( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		}};
		($_span:tt $root:tt $tokens:tt, {#with_exact_span $span:expr, $($tt:tt)*}) => {{
			let span: $crate::__::Span = $span;
			$( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		}};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#macro $path:path $([
			$(
				$(loess $(@ $loess:tt)?)?
				$(span $(@ $span_:tt)?)?
				$(root $(@ $root_:tt)?)?
				$(tokens $(@ $tokens_:tt)?)?
			),* $(, $(@ $comma:tt)?)?
		])?, $($tt:tt)*}) => {
			$path!(
				$([
					$(
						$($crate $(@ $loess_)?)?
						$($span $(@ $span_)?)?
						$($root $(@ $root_)?)?
						$($tokens $(@ $tokens_)?)?
					),* $(, $(@ $comma)?)?
				])?
				$($tt)*
			);
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#let $pat:pat = $expr:expr $(, else { $($else:tt)* })?$(;)?}) => {
			let $pat = $expr;
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#break $($label:lifetime)? $($expr:expr)?$(;)?}) => {
			break $($label)? $($expr)?;
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#continue $($label:lifetime)?$(;)?}) => {
			continue $($label)?;
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#return $($expr:expr)?$(;)?}) => {
			return $($expr)?;
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {# $(else $(@ $else:tt)?)? if $(let $pat:pat =)? $expr:expr, $($tt:tt)*}) => {
			// Handles both `#if` and `#else if`.
			if true $(&& $not_if $(@ $else)?)? {
				if $(let $pat =)? $expr {
					$not_if = false;
					let mut not_if = false;
					$( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
				} else {
					$not_if = true;
				}
			}
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#else, $($tt:tt)*}) => {
			if $not_if {
				$not_if = false;
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
			}
		};
		//TODO: `match`
		($span:tt $root:tt $tokens:tt $not_if:tt, {#$($label:lifetime:)? $(loop $(@ $loop:tt)?)?, $($tt:tt)*}) => {
			// Handles both blocks and unconditional loops.
			$not_if = false;
			$($label:)? $(loop $(@ $loop)?)? {
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
			}
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#$($label:lifetime:)? for $pat:pat in $expr:expr, $($tt:tt)*}) => {
			$not_if = true;
			$($label:)? for $pat in $expr {
				$not_if = false;
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root $tokens $not_if, $tt); )*
			}
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#$($label:lifetime:)? while $(let $pat:pat = )?$expr:expr, $($tt:tt)*}) => {
			$not_if = true;
			$($label:)? while $(let $pat = )?$expr {
				$not_if = false;
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root $tokens $not_if, $tt); )*
			}
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#$reserved:ident $($tt:tt)*}) => {
			$crate::__::compile_error!($crate::__::concat!("`{#", $crate::__::stringify!($reserved), "… }` is reserved within Loess's quotes. (Did you mean `{#paste ", $crate::__::stringify!($reserved), "… }` or `{#, #", $crate::__::stringify!($reserved), "… }`?)"));
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, {#$reserved:lifetime $($tt:tt)*}) => {
			$crate::__::compile_error!($crate::__::concat!("`{#", $crate::__::stringify!($reserved), "… }` is reserved within Loess's quotes. (Did you mean `{#", $crate::__::stringify!($reserved), ":, … }` or `{#", $crate::__::stringify!($reserved), ": for … in …, … }`?)"));
		};
		($span:tt $root:tt $tokens:tt, $_not_if:tt, {$($tt:tt)*}) => {
			$crate::__::grouped($span, $crate::__::Brace, $tokens, {
				let mut inner_tokens = $crate::__::TokenStream::new();
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
				inner_tokens
			});
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, [$($tt:tt)*]) => {
			$crate::__::grouped($span, $crate::__::Bracket, $tokens, {
				let mut inner_tokens = $crate::__::TokenStream::new();
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
				inner_tokens
			});
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, ($($tt:tt)*)) => {
			$crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
				let mut inner_tokens = $crate::__::TokenStream::new();
				let mut not_if = false;
				$( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
				inner_tokens
			});
		};
		($span:tt $root:tt $tokens:tt $not_if:tt, $other:tt) => (
			// Fortunately `'` can't arrive as trailing punctuation inside `$other` here (It's always either a literal or inside a lifetime),
			// so it's at least reasonable to expect `stringify!` to insert a space iff the trailing punctuation is spaced and otherwise not.
			$crate::__::tt($span, $tokens, const { $crate::__::strip_dot($crate::__::stringify!($other .)) } );
		);
		($span:tt $root:tt $tokens:tt $not_if:tt,) => {}; // End.
	}

	pub fn grouped(
		span: Span,
		delimiter: Delimiter,
		tokens: &mut impl Extend<TokenTree>,
		stream: TokenStream,
	) {
		let mut group = Group::new(delimiter, stream);
		group.set_span(span);
		tokens.extend([TokenTree::Group(group)]);
	}

	fn assign_span(
		span: Span,
		ts: impl IntoIterator<Item = TokenTree>,
	) -> impl IntoIterator<Item = TokenTree> {
		ts.into_iter().map(move |tt| match tt {
			TokenTree::Group(group) => {
				let group = Group::new(
					group.delimiter(),
					assign_span(span, group.stream()).into_iter().collect(),
				);
				TokenTree::Group(group)
			}
			mut tt @ (TokenTree::Ident(_) | TokenTree::Punct(_) | TokenTree::Literal(_)) => {
				tt.set_span(span);
				tt
			}
		})
	}

	pub fn raw(span: Span, tokens: &mut impl Extend<TokenTree>, stringified: &str) {
		tokens.extend(assign_span(
			span,
			TokenStream::from_str(stringified).expect("Failed to parse stringified tokens, somehow. (Are you using the internal API from another crate? Please don't.)"),
		))
	}

	pub const fn strip_dot(s: &str) -> &str {
		s.split_at(s.len() - 1).0
	}

	pub fn tt(span: Span, tokens: &mut impl Extend<TokenTree>, stringified: &str) {
		// Note that there can actually be multiple tokens here, since `$tt:tt` grabs lifetimes and certain operators in one go!
		let mut ts = TokenStream::from_str(stringified).expect("Failed to parse stringified tokens, somehow. (Are you using the internal API from another crate? Please don't.)").into_iter().collect::<Box<[_]>>();

		// The last token tree's spacing information is lost, but it's easy enough to restore it here:
		if let TokenTree::Punct(trailing_punct) = ts.last_mut().expect(
			"always at least one token (Please don't use the internal API from other crates.)",
		) {
			*trailing_punct = Punct::new(
				trailing_punct.as_char(),
				match stringified
					.chars()
					.last()
					.expect("always at least one char")
					.is_ascii_whitespace()
				{
					true => Spacing::Alone,
					false => Spacing::Joint,
				},
			)
		}

		tokens.extend(assign_span(span, ts));
	}
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

/// Low-level [`FnOnce`]-unwind-catcher that reports panics to the given [`Errors`] without also catching [`Err(())`](`Err`).
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

/// [`FnOnce`]-unwind-catcher that reports panics to the given [`Errors`].
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

/// Convenient non-repeating [`PopFrom::pop_from`]-unwind-catcher that reports panics to the given [`Errors`].
///
/// Does **not** check for unconsumed [`Input`]! To parse the last part of the input, use
/// <code>[parse_all](input, errors).[next()](`Iterator::next`)</code> instead.
pub fn parse_once<'a, T: PopFrom>(input: &'a mut Input, errors: &'a mut Errors) -> Result<T, ()> {
	parse_once_with_impl(input, errors, T::pop_from)
}

/// Low-level function that parses remaining [`Input`] through [`FnMut`] without also catching [`Err(())`](`Err`),
/// catching and submitting panics to the given [`Errors`].
///
/// # Yields <sub>in order of precedence</sub>
///
/// [`None`] once after the previous call's `f` call didn't change `input`'s length (to break out of stalls),  
/// [`None`] when `input` is empty,  
/// [`None`] when `f` panics,  
/// [`Some`] otherwise.
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
/// 	let root = TokenStream::new(); // See `IntoTokens`.
///
/// 	let Some(tt) = parse_all_with_infallible(
/// 			&mut input,
/// 			&mut errors,
/// 			|input, errors| TokenTree::pop_from(input, errors).expect("infallible"),
/// 		).next() // Checks for exhaustiveness.
/// 		else { return errors.collect_tokens(&root) };
///
/// 	// Make sure to emit `errors` unconditionally,
/// 	// ideally before other output.
/// 	let mut output = errors.collect_tokens(&root);
///
/// 	// Emit your output here:
/// 	tt.into_tokens(&root, &mut output);
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
		stalled: bool,
	}

	impl<'a, T, F: 'a + UnwindSafe + FnMut(&mut Input, &mut Errors) -> T> Iterator for Iter<'a, F> {
		type Item = T;

		fn next(&mut self) -> Option<Self::Item> {
			if mem::take(&mut self.stalled) || self.input.is_empty() {
				None
			} else {
				let before_len = self.input.len();
				let next =
					match parse_once_with_infallible_impl(self.input, self.errors, &mut self.f) {
						Ok(ok) => Some(ok),
						Err(()) => None,
					};
				self.stalled = self.input.len() == before_len;
				next
			}
		}
	}

	impl<'a, F> Drop for Iter<'a, F> {
		/// [`Iter`] borrows the [`Errors`] exclusively, so this will be called before that's turned into output.
		fn drop(&mut self) {
			EndOfInput::<UNCONSUMED_INPUT>::pop_from(self.input, self.errors).ok();
		}
	}

	Iter {
		input,
		errors,
		f,
		stalled: false,
	}
}

/// Parses remaining [`Input`] through [`FnMut`],
/// catching and submitting panics to the given [`Errors`].
///
/// # Yields <sub>in order of precedence</sub>
///
/// [`None`] once after the previous call's `f` call didn't change `input`'s length (to break out of stalls),  
/// [`None`] when `input` is empty,  
/// [`None`] when `f` returns [`Err(())`](`Err`) or panics,  
/// [`Some`] otherwise.
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
/// 	let root = TokenStream::new(); // See `IntoTokens`.
///
/// 	let Some(tt) = parse_all_with(&mut input, &mut errors, TokenTree::pop_from)
/// 		.next() // Checks for exhaustiveness.
/// 		else { return errors.collect_tokens(&root) };
///
/// 	// Make sure to emit `errors` unconditionally,
/// 	// ideally before other output.
/// 	let mut output = errors.collect_tokens(&root);
///
/// 	// Emit your output here:
/// 	tt.into_tokens(&root, &mut output);
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
/// catching and submitting panics to the given [`Errors`].
///
/// # Yields <sub>in order of precedence</sub>
///
/// [`None`] once after the previous call's [`PopFrom::pop_from`] didn't change `input`'s length (to break out of stalls),  
/// [`None`] when `input` is empty,  
/// [`None`] when [`PopFrom::pop_from`] returns [`Err(())`](`Err`) or panics,  
/// [`Some`] otherwise.
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
/// 	let root = TokenStream::new(); // See `IntoTokens`.
///
/// 	let Some(tt) = parse_all::<TokenTree>(&mut input, &mut errors)
/// 		.next() // Checks for exhaustiveness.
/// 		else { return errors.collect_tokens(&root) };
///
/// 	// Make sure to emit `errors` unconditionally,
/// 	// ideally before other output.
/// 	let mut output = errors.collect_tokens(&root);
///
/// 	// Emit your output here:
/// 	tt.into_tokens(&root, &mut output);
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
