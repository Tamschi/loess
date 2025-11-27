use std::{any::type_name, collections::VecDeque, iter, marker::PhantomData};

use never_say_never::Never;
use proc_macro2::{TokenStream, TokenTree};

use crate::{
	ConstErrorPriority, Error, ErrorPriority, Errors, Input, PeekFrom, PopParsedFrom,
	error_priorities::UNCONSUMED_AFTER_REPEATS,
	stateful::{PeekNext, SimpleStepper, Stepper},
};

mod groups;
pub use groups::{CurlyBraces, MetaGroup, Parentheses, SquareBrackets};

/// Doesn't fail to parse but emits an [`Error`] with the given [`ConstErrorPriority`] for any unconsumed tokens in [`Input`] after `T`.
pub(crate) enum Exhaustive<T, P: ConstErrorPriority> {
	#[doc(hidden)]
	Vacant(PhantomData<(T, P)>, Never),
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

/// Exhaustive parsing of `T`.
pub enum ToEnd<T: ?Sized> {
	#[doc(hidden)]
	Vacant(PhantomData<T>, Never),
}

/// Greedy parsing of `T`.
pub enum Greedy<T: ?Sized> {
	#[doc(hidden)]
	Vacant(PhantomData<T>, Never),
}

//TODO: Use this also for Separated and Delimited.
pub trait Repeats {
	type Projected: for<'a> FromIterator<<Self::Stepper<'a> as Stepper<'a>>::Item>;
	type Stepper<'a>: Stepper<'a>;
}

impl<T: Repeats> PopParsedFrom for ToEnd<T> {
	type Parsed = T::Projected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stepper = T::Stepper::attach(input, errors);
		let mut stop = false;

		//TODO: Revise error emission wrt constraint errors!
		iter::from_fn(|| {
			if stop || stepper.input().is_empty() {
				return None;
			}
			let len_before = stepper.input().len();
			let item = match stepper.pop_next() {
				Some(item) => item,
				None => {
					let (input, errors) = stepper.split_mut();
					EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_parsed_from(input, errors).ok();
					stop = true;
					return None;
				}
			};

			if stepper.input().len() == len_before {
				let (input, errors) = stepper.split_mut();
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

			Some(Ok(item))
		})
		.collect()
	}
}

impl<T: Repeats> PopParsedFrom for Greedy<T>
where
	for<'a> T::Stepper<'a>: PeekNext,
{
	type Parsed = T::Projected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stepper = T::Stepper::attach(input, errors);
		let mut stop = false;

		//TODO: Revise error emission wrt constraint errors!
		iter::from_fn(|| {
			if stop || stepper.input().is_empty() {
				return None;
			}
			let len_before = stepper.input().len();
			let item = match stepper.peek_pop_next() {
				Some(item) => item,
				None => {
					stop = true;
					return None;
				}
			};

			if stepper.input().len() == len_before {
				let (input, errors) = stepper.split_mut();
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

			Some(Ok(item))
		})
		.collect()
	}
}

impl<T: PopParsedFrom> Repeats for Vec<T> {
	type Projected = Vec<T::Parsed>;
	type Stepper<'a> = SimpleStepper<'a, T>;
}

impl<T: PopParsedFrom> PopParsedFrom for Vec<T> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

impl<T: PopParsedFrom> Repeats for VecDeque<T> {
	type Projected = VecDeque<T::Parsed>;
	type Stepper<'a> = SimpleStepper<'a, T>;
}

impl<T: PopParsedFrom> PopParsedFrom for VecDeque<T> {
	type Parsed = <ToEnd<Self> as PopParsedFrom>::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		ToEnd::<Self>::pop_parsed_from(input, errors)
	}
}

impl Repeats for TokenStream {
	type Projected = TokenStream;
	type Stepper<'a> = SimpleStepper<'a, TokenTree>;
}

/// A series of alternating `T` and `D` where either can be last.
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
pub struct Separated<T, D> {
	#[allow(missing_docs)]
	pub delimited: Vec<(T, D)>,
	#[allow(missing_docs)]
	pub trailing: Option<T>,
}

impl<T, D> PopParsedFrom for Separated<T, D>
where
	T: PopParsedFrom,
	D: PopParsedFrom + PeekFrom,
{
	type Parsed = Separated<T::Parsed, D::Parsed>;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut delimited = vec![];

		Ok(loop {
			if input.is_empty() {
				break Self::Parsed {
					delimited,
					trailing: None,
				};
			}
			let len_before = input.len();
			if let Ok(trailing) = T::pop_parsed_from(input, errors) {
				if input.is_empty() {
					break Self::Parsed {
						delimited,
						trailing: trailing.into(),
					};
				}
				if let Ok(delimiter) = D::pop_parsed_from(input, errors) {
					delimited.push((trailing, delimiter))
				} else {
					todo!("Recovery.")
				}
			} else {
				todo!("Recovery.")
			}
			if input.len() == len_before {
				errors.push(Error::new(
					ErrorPriority::UNCONSUMED_INPUT,
					format!(
						"{} looped without consuming input. (This likely implies a faulty grammar.)",
						type_name::<Self>()
					),
					input.drain_spans(..),
				));
				continue;
			}
		})
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

impl<T, D> PopParsedFrom for Delimited<T, D>
where
	T: PopParsedFrom,
	D: PopParsedFrom + PeekFrom,
{
	type Parsed = Separated<T::Parsed, D::Parsed>;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		todo!()
	}
}

impl<T, D> PopParsedFrom for Greedy<Separated<T, D>>
where
	T: PopParsedFrom + PeekFrom,
	D: PopParsedFrom + PeekFrom,
{
	type Parsed = Separated<T::Parsed, D::Parsed>;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut delimited = vec![];

		Ok(loop {
			if input.is_empty() {
				break Self::Parsed {
					delimited,
					trailing: None,
				};
			}
			let len_before = input.len();
			match T::peek_pop_parsed_from(input, errors) {
				Ok(Some(trailing)) => match D::peek_pop_parsed_from(input, errors) {
					Ok(Some(delimiter)) => delimited.push((trailing, delimiter)),
					Ok(None) => {
						break Self::Parsed {
							delimited,
							trailing: trailing.into(),
						};
					}
					Err(()) => todo!("Recovery."),
				},
				Ok(None) => {
					break Self::Parsed {
						delimited,
						trailing: None,
					};
				}
				Err(()) => todo!("Recovery."),
			}
			if input.len() == len_before {
				errors.push(Error::new(
					ErrorPriority::UNCONSUMED_INPUT,
					format!(
						"{} looped without consuming input. (This likely implies a faulty grammar.)",
						type_name::<Self>()
					),
					input.drain_spans(..),
				));
				continue;
			}
		})
	}
}

pub enum RepeatConstraint<T, const MIN: usize, const MAX: usize = MIN> {
	Vacant(PhantomData<T>, Never),
}
