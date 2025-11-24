use std::{any::type_name, iter, marker::PhantomData};

use crate::{Error, ErrorPriority, Errors, Input, PeekFrom};

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
			.then_some(Self::pop_parsed_from(input, errors))
			.transpose()
	}
}

pub(crate) enum Vacant {}

pub struct Eager<T> {
	_phantom: PhantomData<T>,
	_vacant: Vacant,
}

impl<T> PopParsedFrom for Eager<T>
where
	T: PopParsedFrom + IntoIterator<Item: PeekFrom + PopParsedFrom>,
	T::Parsed: FromIterator<<<T as IntoIterator>::Item as PopParsedFrom>::Parsed>,
{
	type Parsed = T::Parsed;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut stop = false;
		iter::from_fn(|| {
			if stop || input.is_empty() {
				return None;
			}
			let len_before = input.len();
			let item = match T::Item::peek_pop_parsed_from(input, errors) {
				Ok(Some(item)) => item,
				Err(()) => {
					stop = true;
					return Some(Err(()));
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

pub struct OnePlusEager<T> {
	_phantom: PhantomData<T>,
	_vacant: Vacant,
}

impl<T: PopParsedFrom + PeekFrom> PopParsedFrom for OnePlusEager<T> {
	type Parsed = Vec<T::Parsed>;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		let mut parsed = vec![T::pop_parsed_from(input, errors)?];
		Ok(loop {
			if input.is_empty() {
				break parsed;
			}
			let len_before = input.len();
			let Some(item) = T::peek_pop_parsed_from(input, errors)? else {
				break parsed;
			};
			parsed.extend([item]);
			if input.len() == len_before {
				return Err(errors.push(Error::new(
					ErrorPriority::UNCONSUMED_INPUT,
					format!(
						"{} looped without consuming input. (This likely implies a faulty grammar.)",
						type_name::<Self>()
					),
					input.drain_spans(..),
				)));
			}
		})
	}
}

impl<T: PeekFrom> PeekFrom for OnePlusEager<T> {
	fn peek_from(input: &Input) -> bool {
		T::peek_from(input)
	}
}

/// A series of alternating `T` and `D` where either can be last.
///
/// *Note:* Does not implement [`PopFrom`], as that would require specialisation of [`GrammarPopFrom`].
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
/// Can be wrapped in [`Eager`] to preserve remaining input after [`T::peek_from`](`PeekFrom::peek_from`)
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

impl<T, D> PopParsedFrom for Eager<Separated<T, D>>
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
