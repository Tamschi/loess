//! Modular helper types mainly for use within [`grammar!`](`crate::grammar!`).
//!
//! The enums in this module are vacant, as they parse their wrapped types' projections.
//!
//! See also [`PopParsedFrom`#foreign-impls] for additional, mostly lower-level building blocks.

use std::{any::type_name, collections::VecDeque, convert::Infallible, iter, marker::PhantomData};

use proc_macro2::{TokenStream, TokenTree};

use crate::{
	ConstErrorPriority, Error, ErrorPriority, Errors, Input, PeekFrom, PopParsedFrom,
	error_priorities::UNCONSUMED_AFTER_REPEATS,
	stateful::{
		DelimitedStepper, PeekNextFrom, RepeatCountStepper, SeparatedStepper, SimpleStepper,
		Stepper,
	},
};

mod groups;
pub use groups::{CurlyBraces, MetaGroup, Parentheses, SquareBrackets};

/// Doesn't fail to parse but emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`] after `T`.
pub(crate) enum Exhaustive<T, P: ConstErrorPriority> {
	_Vacant(PhantomData<(T, P)>, Infallible),
}

impl<T: PopParsedFrom, P: ConstErrorPriority> PopParsedFrom for Exhaustive<T, P> {
	type Parsed = T::Parsed;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let value = T::pop_parsed_from(input, errors);
		EndOfInput::<P>::pop_parsed_from(input, errors).ok();
		Ok(value?)
	}
}

//TODO: Maybe replace on input with some into_unconsumed_tokens_error.
/// Fails to parse and emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`].
#[derive(Clone)]
pub(crate) struct EndOfInput<P: ConstErrorPriority>(PhantomData<P>);

/// Fails iff the [`Input`] isn't empty.
impl<P: ConstErrorPriority> PopParsedFrom for EndOfInput<P> {
	type Parsed = Self;
	type Remnant = ();

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
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

/// Exhaustive parsing of <code>C: [`Repeats`]</code>.
/// Often implicit via <code>impl [`PopParsedFrom`]</code>.
pub enum ToEnd<C: ?Sized> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<C>, Infallible),
}

/// Greedy parsing of <code>C: [`Repeats`]</code>.
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
/// 	) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
/// ) -> Result<Self::Projected, ()> {
/// 	iter::from_fn(move || f(input, errors).transpose()).collect()
/// }
/// ```
pub trait Repeats {
	type Projected;
	type Stepper: Stepper;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()>;
}

impl<C: Repeats> PopParsedFrom for ToEnd<C> {
	type Parsed = C::Projected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stepper = C::Stepper::default();
		let mut stop = false;

		//TODO: Revise error emission wrt constraint errors!
		C::collect_repeats(input, errors, &mut |input, errors| {
			if stop || input.is_empty() {
				return Ok(None);
			}
			let len_before = input.len();
			let Some(item) = stepper.pop_next_from(input, errors)? else {
				EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_parsed_from(input, errors).ok();
				stop = true;
				return Ok(None);
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

			Ok(Some(item))
		})
	}
}

impl<C: Repeats> PopParsedFrom for Greedy<C>
where
	C::Stepper: PeekNextFrom,
{
	type Parsed = C::Projected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stepper = C::Stepper::default();
		let mut stop = false;

		//TODO: Revise error emission wrt constraint errors!
		C::collect_repeats(input, errors, &mut |input, errors| {
			if stop || input.is_empty() {
				return Ok(None);
			}
			let len_before = input.len();
			let Some(item) = stepper.peek_pop_next_from(input, errors)? else {
				stop = true;
				return Ok(None);
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

			Ok(Some(item))
		})
	}
}

impl<C: PeekFrom> PeekFrom for Greedy<C> {
	fn peek_from(input: &Input) -> bool {
		C::peek_from(input)
	}
}

impl<C: PeekFrom> PeekFrom for ToEnd<C> {
	fn peek_from(input: &Input) -> bool {
		C::peek_from(input)
	}
}

impl<C: PopParsedFrom> Repeats for Vec<C> {
	type Projected = Vec<C::Parsed>;
	type Stepper = SimpleStepper<C>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()> {
		iter::from_fn(move || f(input, errors).transpose()).collect()
	}
}

/// Implicit [`ToEnd`].
impl<C: PopParsedFrom> PopParsedFrom for Vec<C> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

impl<C: PopParsedFrom> Repeats for VecDeque<C> {
	type Projected = VecDeque<C::Parsed>;
	type Stepper = SimpleStepper<C>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()> {
		iter::from_fn(move || f(input, errors).transpose()).collect()
	}
}

/// Implicit [`ToEnd`].
impl<C: PopParsedFrom> PopParsedFrom for VecDeque<C> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

impl Repeats for TokenStream {
	type Projected = TokenStream;
	type Stepper = SimpleStepper<TokenTree>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()> {
		iter::from_fn(move || f(input, errors).transpose()).collect()
	}
}

/// A series of alternating `T` and `S` where either can be last.
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
/// [`Ok`] once all input is consumed. (Never [`Err`].)
///
/// Can be wrapped in [`Greedy`] to preserve remaining input after [`T::peek_from`](`PeekFrom::peek_from`)
/// returns [`false`] at the start of an iteration.
pub struct Separated<T, S> {
	#[allow(missing_docs)]
	pub delimited: Vec<(T, S)>,
	#[allow(missing_docs)]
	pub trailing: Option<T>,
}

impl<T, S> Repeats for Separated<T, S>
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
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()> {
		todo!()
	}
}

/// Implicit [`ToEnd`].
impl<T: PopParsedFrom, S: PopParsedFrom + PeekFrom> PopParsedFrom for Separated<T, S> {
	type Parsed = <ToEnd<Separated<T, S>> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		<ToEnd<Separated<T, S>> as PopParsedFrom>::pop_parsed_from(input, errors)
	}
}

/// A series of alternating `T` and **precedent** `D` where either can be last.
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
/// [`Ok`] once all input is consumed. (Never [`Err`].)
///
/// Can be wrapped in [`Greedy`] to preserve remaining input after [`T::peek_from`](`PeekFrom::peek_from`)
/// returns [`false`] at the start of an iteration.
pub struct Delimited<T, D> {
	#[allow(missing_docs)]
	pub delimited: Vec<(T, D)>,
	#[allow(missing_docs)]
	pub trailing: Option<T>,
}

impl<T, D> Repeats for Delimited<T, D>
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
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()> {
		todo!()
	}
}

/// Implicit [`ToEnd`].
impl<T: PopParsedFrom, D: PopParsedFrom + PeekFrom> PopParsedFrom for Delimited<T, D> {
	type Parsed = <ToEnd<Delimited<T, D>> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		<ToEnd<Delimited<T, D>> as PopParsedFrom>::pop_parsed_from(input, errors)
	}
}

/// Wraps around other <code>C: [`Repeats`]</code> to constrain item count.
pub enum RepeatCount<C, const MIN: usize, const MAX: usize> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<C>, Infallible),
}

impl<C: Repeats, const MIN: usize, const MAX: usize> Repeats for RepeatCount<C, MIN, MAX> {
	type Projected = C::Projected;

	type Stepper = RepeatCountStepper<C::Stepper, MIN, MAX>;

	fn collect_repeats(
		input: &mut Input,
		errors: &mut Errors,
		f: &mut dyn FnMut(
			&mut Input,
			&mut Errors,
		) -> Result<Option<<Self::Stepper as Stepper>::Item>, ()>,
	) -> Result<Self::Projected, ()> {
		//TODO: Constrain here too/only?
		C::collect_repeats(input, errors, f)
	}
}

/// Implicit [`ToEnd`].
impl<C: Repeats, const MIN: usize, const MAX: usize> PopParsedFrom for RepeatCount<C, MIN, MAX> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

/// Waiting on feature [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560) for expansion.
impl<C: Repeats, const MAX: usize> PeekFrom for RepeatCount<C, 1, MAX>
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
// 			Ok(parsed) => Ok(parsed),
// 			Err(()) => {
// 				while !input.is_empty() && !T::peek_from(input) {
// 					input.tokens.pop_front();
// 				}
// 				T::pop_parsed_from(input, &mut Errors::new()) // Silent.
// 			}
// 		}
// 	}
// }
