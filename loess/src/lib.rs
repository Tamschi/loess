//! <details><summary style=cursor:pointer><u>README / Examples (click to expand)</u></summary>
//!
#![doc = include_str!("../../README.md")]
//!
//! </details>
//!
//! Loess is a parser library and parser generator for proc macros.
//!
//! For a simple but representative example of using Loess, see the [*inline-json5*](https://crates.io/crates/inline-json5) crate.
//!
//! In most cases you'll want to:
//!
//! 0. generate custom grammar implementations with [`grammar!`] (You can also easily implement parts manually.),
//! 1. create (mutable) instances of [`Input`] and [`Errors`],
//! 2. step through the input with [`parse_once`], [`parse_once_with`] and/or [`parse_once_with_infallible`],
//! 3. consume the last of the input with [`parse_all`], [`parse_all_with`] or [`parse_all_with_infallible`],
//! 4. perform any fallible transforms you need, possibly pushing more [`Error`]s into your [`Errors`],
//! 5. if `errors` is your [`Errors`], have `let mut output: proc_macro2::TokenStream = errors.collect_tokens();`
//!    convert it into the start of your output,
//! 6. emit your regular output with [`quote_into_mixed_site!`] (recommended), [`quote_into_with_exact_span!`] or
//!    [`quote_into_call_site!`], which accept interpolation and control flow directives.
//!
//! You can call either [`Iterator::collect`] (for repeats) or [`Iterator::next`] (for one value) on step 3.
//! Either way, the parsing iterator will check for unconsumed tokens remaining in the [`Input`] when dropped
//! and report to the [`Errors`] accordingly.
//!
//! You can combine step 2 into step 3 with a [`grammar!`]-generated top-level grammar, but for proc macros embedded
//! in a runtime library, in most cases I recommend getting `$crate` from a wrapper `macro_rules!`-macro first.
//! (See full example above.)
//!
//! Some parsing errors are recoverable, but still translate to [`compile_error!`] calls being generated in step 5.
//! Your macro should seamlessly continue to operate in such cases, which helps prevent noise from cascading errors
//! due to e.g. missing items, making it much easier for your macro's users to find problems with the input.
//!
//! You can download a *.code-snippets* file for Loess's macros and quote macro directives here:
//! <https://github.com/Tamschi/loess/blob/develop/.vscode/Loess.code-snippets>
//!
//! All macros published by this crate are fully hygienic and use `$crate`, so you can rename dependencies freely.

#![warn(clippy::pedantic, missing_docs)]

use std::{
	self,
	any::Any,
	collections::{VecDeque, vec_deque},
	fmt::Debug,
	iter::{self},
	marker::PhantomData,
	mem,
	ops::RangeBounds,
	panic::{AssertUnwindSafe, UnwindSafe, catch_unwind},
	vec,
};

use error_priorities::{UNCONSUMED_AFTER_REPEATS, UNCONSUMED_INPUT};
use never_say_never::Never;
use proc_macro2::{Ident, Literal, Punct, Span, TokenStream, TokenTree};

mod proc_macro2_impls;

//TODO: EagerPlusOne -> Eager<Repeat>
//TODO: Repeat -> Lower and upper bound on repetition.
//TODO: Repeat<Separated>
//TODO: Eager<Repeat<Separated>>
pub mod scaffold;

pub mod stateful;

mod macros;
pub use macros::__;

use crate::scaffold::EndOfInput;

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

	/// since 0.2.4
	pub fn message(&self) -> &str {
		&self.message
	}

	/// Returns the error's overall [`Span`],
	/// but only iff that can be constructed by joining all the [`Span`]s originally given to [`Error::new`].
	///
	/// since 0.2.4
	pub fn span(&self) -> Option<Span> {
		self.spans
			.iter()
			.cloned()
			.map(|a| Some(a))
			.reduce(|a, b| a.zip(b).map(|(a, b)| a.join(b)).flatten())
			.flatten()
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

		#[allow(unused_variables, unused_mut)] // Not suppressed because it's the same crate.
		{
			quote_into_with_exact_span! (span, root, tokens, {
				{#error { {#(message)} }};
			});
		}
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

	/// since 0.2.4
	pub fn into_of_highest_priority(self) -> impl Iterator<Item = Error> {
		let highest_priority = self.errors.iter().map(|error| error.priority).max();

		self.errors
			.into_iter()
			.filter(move |e| e.priority == highest_priority.unwrap())
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
/// 	scaffold::SquareBrackets,
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
	/// Emits `self`'s tokens into `tokens` while referencing `root`.
	///
	/// `root` <em style=font-style:normal;font-variant:small-caps>should</em> be prefixed to
	/// any fully qualified paths that are emitted by `self`.
	///
	/// This method is not fallible and doesn't borrow [`Errors`] because you *really*
	/// <em style=font-style:normal;font-variant:small-caps>should</em> emit all your
	/// parsing [`Error`]s before your regular macro output.
	///
	/// It's likely cleanest to do fallible transforms separately first, into output types
	/// with [`grammar!`]-generated [`IntoTokens`] implementations, in particular to
	/// take advantage of [`ErrorPriority`], but you can of course still sneakily emit
	/// diagnostics here if you think that doesn't misalign their order and causality,
	/// and doesn't cause error spans to overlap in your input (which technically is
	/// permitted just fine, but often leads to a bad dev UX for your macro consumers).
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

impl<T: IntoTokens + Clone> IntoTokens for &T {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		T::into_tokens(self.clone(), root, tokens)
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
	/// Iff `self` is long enough, `f` is called with references to the frontmost tokens
	/// and a [`vec_deque::Iter`] over the remaining tokens after them.
	///
	/// # Returns
	///
	/// `false` if `self` is too short, otherwise the return value of `f`.
	pub fn peek<'a, const N: usize>(
		&'a self,
		f: impl FnOnce([&TokenTree; N], vec_deque::Iter<'a, TokenTree>) -> bool,
	) -> bool {
		//TODO: Handle none-delimiter groups. (Maybe not here?)
		if self.len() < N {
			false
		} else {
			let mut iter = self.tokens.iter();
			f(
				std::array::from_fn(|_| iter.next().expect("due to !(self.len() < N)")),
				iter,
			)
		}
	}

	/// Convenience method to match from an array of [`TokenTree`]s.
	///
	/// This is mostly for [`PopFrom`] implementations.  
	/// Grammar consumers should call <code>Token::[pop_from](`PopFrom::pop_from`)</code> instead.
	///
	/// Iff `self` is long enough, `f` is called with the frontmost tokens
	/// and an <code>&mut [Input]</code> pointing to `self`.
	///
	/// # Returns
	///
	/// If `self` is too short or `f` returns [`Err`],
	/// respective [`Span`]s to create an [`Error`] with.
	///
	/// Iff `self` is too short, <code>self.[end](`Input::end`)</code> is included.
	pub fn pop_or_replace<'a, T, const N: usize>(
		&'a mut self,
		//TODO (breaking): Also pass `&mut self` into the closure to check/consume further tokens.
		f: impl FnOnce([TokenTree; N], &mut Self) -> Result<T, [TokenTree; N]>,
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
			match f(
				[(); N].map(|()| self.tokens.pop_front().expect("unreachable")),
				self,
			) {
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

	pub fn drain_spans(&mut self, range: impl RangeBounds<usize>) -> impl Iterator<Item = Span> {
		let end = self.is_empty().then_some(self.end);
		self.tokens.drain(range).map(|tt| tt.span()).chain(end)
	}
}

/// Consumes from [`Input`] to create <code>[`Result`]&lt;Self::[Parsed](`PopParsedFrom::Parsed`), ()></code> and emit to [`Errors`].
pub trait PopParsedFrom {
	type Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()>;
	fn peek_pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Parsed>, ()>
	where
		Self: PeekFrom,
	{
		Self::peek_from(input)
			.then(|| Self::pop_parsed_from(input, errors))
			.transpose()
	}
}

/// Proxies <code>[`PopParsedFrom`]&lt;Parsed = Self></code> with easier type inference.
///
/// This is effectively a sealed trait. To implement this trait, implement [`PopParsedFrom`] with `type Parsed = Self;`.
pub trait PopFrom: PopParsedFrom<Parsed = Self> {
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

impl<T: PopParsedFrom<Parsed = Self>> PopFrom for T {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Self::pop_parsed_from(input, errors)
	}

	fn peek_pop_from(input: &mut Input, errors: &mut Errors) -> Result<Option<Self>, ()>
	where
		Self: PeekFrom,
	{
		Self::peek_pop_parsed_from(input, errors)
	}
}

impl<T: PopParsedFrom> PopParsedFrom for Box<T> {
	type Parsed = Box<T::Parsed>;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()>
	where
		Self: Sized,
	{
		Ok(Box::new(T::pop_parsed_from(input, errors)?))
	}
}

impl<T: IntoTokens> IntoTokens for Box<T> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		(*self).into_tokens(root, tokens)
	}
}

impl<T: PeekFrom + PopParsedFrom> PopParsedFrom for Option<T> {
	type Parsed = Option<T::Parsed>;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()>
	where
		Self: Sized,
	{
		T::peek_from(input)
			.then(|| T::pop_parsed_from(input, errors))
			.transpose()
	}
}

/// Determines if `Self` may be be parseable from an [`Input`].
/// **This is often a cursory check!**
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
	/// [`PopParsedFrom::pop_parsed_from`] <em style=font-style:normal;font-variant:small-caps>may</em> still fail and/or push to [`Errors`].
	///
	/// ## [`false`]
	///
	/// [`PopParsedFrom::pop_parsed_from`] <em style=font-style:normal;font-variant:small-caps>should</em> fail **and** push to [`Errors`].
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
pub(crate) fn parse_once_with_infallible_impl<'a, T>(
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
pub(crate) fn parse_once_with_impl<'a, T>(
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
pub(crate) fn parse_all_with_infallible_impl<'a, T>(
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

pub trait LocatedAt {
	fn located_at(self, span: Span) -> Self;
}

impl LocatedAt for Span {
	fn located_at(self, span: Span) -> Self {
		(&self).located_at(span)
	}
}

impl LocatedAt for Ident {
	fn located_at(mut self, span: Span) -> Self {
		self.set_span(self.span().located_at(span));
		self
	}
}

impl LocatedAt for Punct {
	fn located_at(mut self, span: Span) -> Self {
		self.set_span(self.span().located_at(span));
		self
	}
}

pub trait ResolvedAt {
	fn resolved_at(self, span: Span) -> Self;
}

impl ResolvedAt for Span {
	fn resolved_at(self, span: Span) -> Self {
		(&self).resolved_at(span)
	}
}

impl ResolvedAt for Ident {
	fn resolved_at(mut self, span: Span) -> Self {
		self.set_span(self.span().resolved_at(span));
		self
	}
}

impl ResolvedAt for Punct {
	fn resolved_at(mut self, span: Span) -> Self {
		self.set_span(self.span().resolved_at(span));
		self
	}
}
