use std::{any::type_name, collections::VecDeque, iter, marker::PhantomData, mem};

use never_say_never::Never;
use proc_macro2::{TokenStream, TokenTree};

use crate::{
	ConstErrorPriority, Error, ErrorPriority, Errors, Input, PeekFrom, PopParsedFrom,
	error_priorities::UNCONSUMED_AFTER_REPEATS,
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

//TODO: Abstract this more so that it works for Separated and Delimited.
pub trait Repeats {
	type Projected: FromIterator<<Self::Template as PopParsedFrom>::Parsed>;
	type Template: PopParsedFrom;
}

impl<T: Repeats> PopParsedFrom for ToEnd<T> {
	type Parsed = T::Projected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stop = false;

		iter::from_fn(|| {
			if stop || input.is_empty() {
				return None;
			}
			let len_before = input.len();
			let item = match T::Template::pop_parsed_from(input, errors) {
				Ok(item) => item,
				Err(()) => {
					EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_parsed_from(input, errors).ok();
					stop = true;
					return None;
				}
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

			Some(Ok(item))
		})
		.collect()
	}
}

impl<T: Repeats> PopParsedFrom for Greedy<T>
where
	T::Template: PeekFrom,
{
	type Parsed = T::Projected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stop = false;
		iter::from_fn(|| {
			if stop || input.is_empty() {
				return None;
			}
			let len_before = input.len();
			let item = match T::Template::peek_pop_parsed_from(input, errors) {
				Ok(Some(item)) => item,
				Err(()) => {
					EndOfInput::<UNCONSUMED_AFTER_REPEATS>::pop_parsed_from(input, errors).ok();
					stop = true;
					return None;
				}
				Ok(None) => return None,
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

			Some(Ok(item))
		})
		.collect()
	}
}

impl<T: PopParsedFrom> Repeats for Vec<T> {
	type Projected = Vec<T::Parsed>;
	type Template = T;
}

impl<T: PopParsedFrom> Repeats for VecDeque<T> {
	type Projected = VecDeque<T::Parsed>;
	type Template = T;
}

impl Repeats for TokenStream {
	type Projected = TokenStream;
	type Template = TokenTree;
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

/// A series of alternating `T` and `D` where either can be last. **`D` has precedence!**
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

pub trait Repeating {
	type Aggregator: Extend<<Self::Repeat as PopParsedFrom>::Parsed>;
	type Repeat: PopParsedFrom;
	type Collected: TryFrom<Self::Aggregator>;

	fn aggregator(size_hint_lower: usize, size_hint_upper: Option<usize>) -> Self::Aggregator;

	//TODO
}

pub trait SimpleRepeating: Repeating + IntoIterator + Extend<Self::Item> {}

/// TODO: Adjust!
///
/// Determines if `Self` may be be parseable from an [`Input`].
/// **This is often a cursory check!**
///
/// Used for variant selection in <code>&lt;[`Option`]&lt;Self> as [`PopFrom`]>::[pop_from](`PopFrom::pop_from`)</code>.  
/// Does **not** affect <code>[`Vec`]&lt;Self></code> or <code>[`VecDeque`]&lt;Self></code> parsing, which is exhaustive.
///
/// Also enables [`PopFrom::peek_pop_from`] for `Self`, which is used in [`grammar!`]-generated enum parsers.
///
/// Intentionally not implemented for [`Option`], as it would always match, which is too error-prone.
pub trait PeekRepeatFrom {
	/// # Returns
	///
	/// ## [`true`]
	///
	/// [`PopFrom::pop_from`] <em style=font-style:normal;font-variant:small-caps>may</em> still fail and/or push to [`Errors`].
	///
	/// ## [`false`]
	///
	/// [`PopFrom::pop_from`] <em style=font-style:normal;font-variant:small-caps>should</em> fail **and** push to [`Errors`].
	fn peek_repeat_from(input: &Input) -> bool;
}

impl<T: Repeating, const MIN: usize, const MAX: usize> PopParsedFrom
	for RepeatConstraint<T, MIN, MAX>
{
	type Parsed = T::Collected;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		todo!()
	}
}

impl<T: PeekRepeatFrom, const MIN: usize, const MAX: usize> PeekFrom
	for RepeatConstraint<T, MIN, MAX>
{
	fn peek_from(input: &Input) -> bool {
		if MAX < MIN {
			false
		} else if MIN == 0 {
			true
		} else {
			T::peek_repeat_from(input)
		}
	}
}
