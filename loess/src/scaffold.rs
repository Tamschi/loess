//! Modular helper types mainly for use within [`grammar!`](`crate::grammar!`).
//!
//! The enums in this module are vacant, as they parse their wrapped types' projections.
//!
//! See also [`PopParsedFrom`#foreign-impls] for additional, mostly lower-level building blocks.

use std::{
	any::type_name,
	collections::VecDeque,
	convert::{Infallible, identity},
	iter,
	marker::PhantomData,
	ops::ControlFlow::{self, Break, Continue},
};

use proc_macro2::{TokenStream, TokenTree};

use crate::{
	ConstErrorPriority, Error, ErrorPriority, Errors, Input, PeekFrom, PopParsedFrom,
	stateful::{
		DelimitedStepper, PeekNextFrom, RepeatCountStepper, SeparatedStepper, SimpleStepper,
		Stepper,
	},
};

mod groups;
pub use groups::{CurlyBraces, MetaGroup, Parentheses, SquareBrackets};

mod scopes;
pub use scopes::{In, Scope};

/// Doesn't fail to parse but emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`] after `T`.
pub(crate) enum Exhaustive<T, P: ConstErrorPriority> {
	_Vacant(PhantomData<(T, P)>, Infallible),
}

/// Always succeeds peeks.
pub enum Optimistic<T> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<T>, Infallible),
}

impl<T> PeekFrom for Optimistic<T> {
	fn peek_from(_input: &Input) -> bool {
		true
	}
}

impl<T: PopParsedFrom> PopParsedFrom for Optimistic<T> {
	type Parsed = T::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		T::pop_parsed_from(input, errors)
	}
}

impl<T: PopParsedFrom, P: ConstErrorPriority> PopParsedFrom for Exhaustive<T, P> {
	type Parsed = T::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		let value = T::pop_parsed_from(input, errors)?;
		EndOfInput::<P>::pop_parsed_from(input, errors);
		Continue(value)
	}
}

//TODO: Maybe replace on input with some into_unconsumed_tokens_error.
/// Fails to parse and emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`].
#[derive(Clone)]
pub(crate) struct EndOfInput<P: ConstErrorPriority>(PhantomData<P>);

/// Recovers towards end of input.
impl<P: ConstErrorPriority> PopParsedFrom for EndOfInput<P> {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>> {
		if !input.is_empty() {
			let rest = input.tokens.iter().cloned().collect::<TokenStream>();
			errors.push(Error::new(
				P::PRIORITY,
				format!("Unconsumed tokens: `{rest}`"),
				rest.into_iter().map(|t| t.span()),
			));
		}
		Continue(Some(Self(PhantomData)))
	}
}

/// Exhaustive parsing of <code>C: [`Repetition`]</code>.
/// Often implicit via <code>impl [`PopParsedFrom`]</code>.
pub enum ToEnd<C: ?Sized> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<C>, Infallible),
}

/// Greedy parsing of <code>C: [`Repetition`]</code>.
pub enum Greedy<C: ?Sized> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<C>, Infallible),
}

//TODO: Use this also for Separated and Delimited.
/// Flexible [`Stepper`]-based parsing of items used by [`Greedy`] and [`ToEnd`].
///
/// You can usually copy-paste the following:
///
/// ```rust,ignore
/// fn collect_repeats(
/// 	input: &mut Input,
/// 	errors: &mut Errors,
/// 	f: &mut dyn FnMut(
/// 		&mut Input,
/// 		&mut Errors,
/// 	) -> Result<
/// 		Option<<Self::Stepper as Stepper>::Item>,
/// 		Option<<Self::Stepper as Stepper>::Item>,
/// 	>,
/// ) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
/// 	let mut failed = false;
/// 	let collection = iter::from_fn(|| {
/// 		(!failed)
/// 			.then(|| {
/// 				f(input, errors).unwrap_or_else(|item| {
/// 					failed = true;
/// 					item
/// 				})
/// 			})
/// 			.flatten()
/// 	})
/// 	.collect();
/// 	if failed { Break(Some(collection)) } else { Continue(collection) }
/// }
/// ```
pub trait Repetition {
	type Projected;
	type Stepper: Stepper;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>>;
}

impl<C: Repetition> PopParsedFrom for ToEnd<C> {
	type Parsed = C::Projected;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		let mut stepper = C::Stepper::default();
		let mut stop = false;

		//TODO: Revise error emission wrt constraint errors!
		C::collect_repeats(input, errors, &mut |input, errors| {
			if stop || input.is_empty() {
				return Continue(None);
			}
			let len_before = input.len();
			let item = stepper.pop_next_from(input, errors)?;

			if input.len() == len_before {
				errors.push(Error::new(
					ErrorPriority::UNCONSUMED_INPUT,
					format!(
						"{} looped without consuming input. (This likely implies a faulty grammar.)",
						type_name::<Self>()
					),
					input.drain_spans(..),
				));
				stop = true;
			}

			Continue(item)
		})
	}
}

impl<C: Repetition> PopParsedFrom for Greedy<C>
where
	C::Stepper: PeekNextFrom,
{
	type Parsed = C::Projected;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		let mut stepper = C::Stepper::default();
		let mut stop = false;

		//TODO: Revise error emission wrt constraint errors!
		C::collect_repeats(input, errors, &mut |input, errors| {
			if stop || input.is_empty() {
				return Continue(None);
			}
			let len_before = input.len();
			let Some(item) = stepper.peek_pop_next_from(input, errors)? else {
				stop = true;
				return Continue(None);
			};

			if input.len() == len_before {
				errors.push(Error::new(
					ErrorPriority::UNCONSUMED_INPUT,
					format!(
						"{} looped without consuming input. (This likely implies a faulty grammar.)",
						type_name::<Self>()
					),
					input.drain_spans(..),
				));
				stop = true;
			}

			Continue(Some(item))
		})
	}
}

impl<C: PeekFrom> PeekFrom for ToEnd<C> {
	fn peek_from(input: &Input) -> bool {
		C::peek_from(input)
	}
}

impl<C: PopParsedFrom> Repetition for Vec<C> {
	type Projected = Vec<C::Parsed>;
	type Stepper = SimpleStepper<C>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
		let mut failed = false;
		let collection = iter::from_fn(|| {
			(!failed)
				.then(|| {
					f(input, errors).continue_ok().unwrap_or_else(|item| {
						failed = true;
						item
					})
				})
				.flatten()
		})
		.collect();
		if failed {
			Break(Some(collection))
		} else {
			Continue(Some(collection))
		}
	}
}

/// Implicit [`ToEnd`].
impl<C: PopParsedFrom> PopParsedFrom for Vec<C> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

impl<C: PopParsedFrom> Repetition for VecDeque<C> {
	type Projected = VecDeque<C::Parsed>;
	type Stepper = SimpleStepper<C>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
		let mut failed = false;
		let collection = iter::from_fn(|| {
			(!failed)
				.then(|| {
					f(input, errors).continue_ok().unwrap_or_else(|item| {
						failed = true;
						item
					})
				})
				.flatten()
		})
		.collect();
		if failed {
			Break(Some(collection))
		} else {
			Continue(Some(collection))
		}
	}
}

/// Implicit [`ToEnd`].
impl<C: PopParsedFrom> PopParsedFrom for VecDeque<C> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

impl Repetition for TokenStream {
	type Projected = TokenStream;
	type Stepper = SimpleStepper<TokenTree>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
		let mut failed = false;
		let collection = iter::from_fn(|| {
			(!failed)
				.then(|| {
					f(input, errors).continue_ok().unwrap_or_else(|item| {
						failed = true;
						item
					})
				})
				.flatten()
		})
		.collect();
		if failed {
			Break(Some(collection))
		} else {
			Continue(Some(collection))
		}
	}
}

/// [`Repetition`] of alternating `T` and `S` where either can be last.
///
/// # Recovery
///
/// Recovers towards `S`, preserving it if a `T` led the current repeat.
///
/// # Errors
///
/// Emits an error iff a repetition does not consume any input, consuming all remaining input.
///
/// This is a symptom of faulty grammar definitions.
///
/// # Returns
///
/// [`Continue`] once all input is consumed. (Never [`Break`].)
///
/// Can be wrapped in [`Greedy`] to preserve remaining input after [`T::peek_from`](`PeekFrom::peek_from`)
/// returns [`false`] at the start of an iteration.
pub struct Separated<T, S> {
	#[allow(missing_docs)]
	pub delimited: Vec<(T, S)>,
	#[allow(missing_docs)]
	pub trailing: Option<T>,
}

impl<T, S> Repetition for Separated<T, S>
where
	T: PopParsedFrom,
	S: PopParsedFrom + PeekFrom,
{
	type Projected = Separated<T::Parsed, S::Parsed>;

	type Stepper = SeparatedStepper<T, S>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
		let mut delimited = vec![];
		let mut failed = false;
		while !input.is_empty() {
			let len_before = input.len();
			let step = f(input, errors);
			failed |= step.is_break();
			match step.continue_ok().unwrap_or_else(identity) {
				None => {
					return if failed { |e| Break(e) } else { Continue }(Some(Self::Projected {
						delimited,
						trailing: None,
					}));
				}
				Some((t, None)) => {
					return if failed { |e| Break(e) } else { Continue }(Some(Self::Projected {
						delimited,
						trailing: Some(t),
					}));
				}
				Some((t, Some(s))) => delimited.push((t, s)),
			}
			assert!(
				input.len() < len_before,
				"`Separated` repeat parsed without consuming tokens."
			);
		}
		(if failed { |e| Break(e) } else { Continue })(Some(Self::Projected {
			delimited,
			trailing: None,
		}))
	}
}

/// Implicit [`ToEnd`].
impl<T: PopParsedFrom, S: PopParsedFrom + PeekFrom> PopParsedFrom for Separated<T, S> {
	type Parsed = <ToEnd<Separated<T, S>> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		<ToEnd<Separated<T, S>> as PopParsedFrom>::pop_parsed_from(input, errors)
	}
}

/// [`Repetition`] of alternating `T` and **precedent** `D` where either can be last.
///
/// # Recovery
///
/// Recovers towards `D`, preserving it if a `T` led the current repeat.
///
/// # Errors
///
/// Emits an error iff a repetition does not consume any input, consuming all remaining input.
///
/// This is a symptom of faulty grammar definitions.
///
/// # Returns
///
/// [`Continue`] once all input is consumed. (Never [`Break`].)
///
/// Can be wrapped in [`Greedy`] to preserve remaining input after [`T::peek_from`](`PeekFrom::peek_from`)
/// returns [`false`] at the start of an iteration.
pub struct Delimited<T, D> {
	#[allow(missing_docs)]
	pub delimited: Vec<(T, D)>,
	#[allow(missing_docs)]
	pub trailing: Option<T>,
}

impl<T, D> Repetition for Delimited<T, D>
where
	T: PopParsedFrom,
	D: PopParsedFrom + PeekFrom,
{
	type Projected = Delimited<T::Parsed, D::Parsed>;

	type Stepper = DelimitedStepper<T, D>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
		todo!("Delimited::collect_repeats")
	}
}

/// Implicit [`ToEnd`].
impl<T: PopParsedFrom, D: PopParsedFrom + PeekFrom> PopParsedFrom for Delimited<T, D> {
	type Parsed = <ToEnd<Delimited<T, D>> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		<ToEnd<Delimited<T, D>> as PopParsedFrom>::pop_parsed_from(input, errors)
	}
}

/// Wraps around other <code>C: [`Repetition`]</code> to constrain item count.
pub enum RepeatCount<C, const MIN: usize, const MAX: usize> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<C>, Infallible),
}

impl<C: Repetition, const MIN: usize, const MAX: usize> Repetition for RepeatCount<C, MIN, MAX> {
	type Projected = C::Projected;

	type Stepper = RepeatCountStepper<C::Stepper, MIN, MAX>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> ControlFlow<
			Option<<Self::Stepper as Stepper>::Item>,
			Option<<Self::Stepper as Stepper>::Item>,
		>,
	) -> ControlFlow<Option<Self::Projected>, Option<Self::Projected>> {
		//TODO: Constrain here too/only?
		C::collect_repeats(input, errors, f)
	}
}

/// Implicit [`ToEnd`].
impl<C: Repetition, const MIN: usize, const MAX: usize> PopParsedFrom for RepeatCount<C, MIN, MAX> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

/// Waiting on feature [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560) for expansion.
impl<C: Repetition, const MAX: usize> PeekFrom for RepeatCount<C, 1, MAX>
where
	C::Stepper: PeekNextFrom,
{
	fn peek_from(input: &Input) -> bool {
		C::Stepper::default().peek_next_from(input)
	}
}

// /// Iff `T` fails to parse immediately, slides along the input by peeking until `T` peeks successfully and then retries once.
// ///
// /// Transparent to errors only during the initial attempt.
// pub enum Slide<T> {
// 	#[expect(missing_docs)]
// 	_Vacant(PhantomData<T>, Infallible),
// }

// impl<T: PopParsedFrom + PeekFrom> PopParsedFrom for Slide<T> {
// 	type Parsed = T::Parsed;

// 	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
// 		match T::pop_parsed_from(input, errors) {
// 			Continue(parsed) => Continue(parsed),
// 			Break(()) => {
// 				while !input.is_empty() && !T::peek_from(input) {
// 					input.tokens.pop_front();
// 				}
// 				T::pop_parsed_from(input, &mut Errors::new()) // Silent.
// 			}
// 		}
// 	}
// }

/// <code>[Break] => [Continue]</code>
pub enum Try<T: ?Sized> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<T>, Infallible),
}

impl<T: ?Sized + PeekFrom> PeekFrom for Try<T> {
	fn peek_from(input: &Input) -> bool {
		T::peek_from(input)
	}
}

impl<T: ?Sized + PopParsedFrom> PopParsedFrom for Try<T> {
	type Parsed = T::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		match T::pop_parsed_from(input, errors) {
			Continue(o) | Break(o) => Continue(o),
		}
	}
}

/// On [`Break`], scans for and then discards `D` without recovering.
///
/// Stops early if `D` fails. (`D`'s errors are surfaced.)
///
/// //TODO: ReplaceWith `RealignAfter`
pub enum OnErrSkipPast<T: ?Sized, D: ?Sized> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<T>, PhantomData<D>, Infallible),
}

impl<T: ?Sized + PeekFrom, D: ?Sized> PeekFrom for OnErrSkipPast<T, D> {
	fn peek_from(input: &Input) -> bool {
		T::peek_from(input)
	}
}

impl<T: PopParsedFrom, D: PopParsedFrom + PeekFrom> PopParsedFrom for OnErrSkipPast<T, D> {
	type Parsed = T::Parsed;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		T::pop_parsed_from(input, errors).map_break(|placeholder| {
			while !input.is_empty() {
				match D::peek_pop_parsed_from(input, errors) {
					Continue(None) => drop(input.tokens.pop_front()),
					Continue(Some(_)) => {}
					Break(_) => return placeholder,
				}
			}
			placeholder
		})
	}
}
