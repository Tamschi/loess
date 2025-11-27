use std::{any::type_name, collections::VecDeque, marker::PhantomData};

use crate::{Error, ErrorPriority, Errors, Input, PeekFrom, PopParsedFrom};

pub trait Stepper: Default {
	type Item;

	fn pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Item>, ()>;

	fn peek_pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Item>, ()>
	where
		Self: PeekNextFrom,
	{
		self.peek_next_from(input)
			.then_some(self.pop_next_from(input, errors))
			.transpose()
			.map(Option::flatten)
	}
}

pub trait PeekNextFrom {
	/// # Returns
	///
	/// ## [`true`]
	///
	/// [`StatefulPopParsedFrom::pop_parsed_from`] <em style=font-style:normal;font-variant:small-caps>may</em> still fail and/or push to [`Errors`].
	///
	/// ## [`false`]
	///
	/// [`StatefulPopParsedFrom::pop_parsed_from`] <em style=font-style:normal;font-variant:small-caps>should</em> fail **and** push to [`Errors`].
	fn peek_next_from(&self, input: &Input) -> bool;
}

pub struct SimpleStepper<T> {
	_phantom: PhantomData<T>,
}

impl<T> Default for SimpleStepper<T> {
	fn default() -> Self {
		Self {
			_phantom: PhantomData,
		}
	}
}

impl<T: PopParsedFrom> Stepper for SimpleStepper<T> {
	type Item = T::Parsed;

	fn pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Item>, ()> {
		T::pop_parsed_from(input, errors).map(Some)
	}
}

impl<T: PeekFrom> PeekNextFrom for SimpleStepper<T> {
	fn peek_next_from(&self, input: &Input) -> bool {
		T::peek_from(input)
	}
}

pub struct RepeatCountStepper<S: Stepper, const MIN: usize, const MAX: usize> {
	inner: S,
	buffer: VecDeque<S::Item>,
	counter: usize,
}

impl<S: Stepper, const MIN: usize, const MAX: usize> Default for RepeatCountStepper<S, MIN, MAX> {
	fn default() -> Self {
		Self {
			inner: S::default(),
			buffer: VecDeque::new(),
			counter: 0,
		}
	}
}

impl<S: Stepper, const MIN: usize, const MAX: usize> Stepper for RepeatCountStepper<S, MIN, MAX> {
	type Item = S::Item;

	fn pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Item>, ()> {
		const {
			assert!(MIN <= MAX);
		};

		fn pop_next_from<S: Stepper>(
			inner: &mut S,
			buffer: &mut VecDeque<S::Item>,
			counter: &mut usize,
			input: &mut Input,
			errors: &mut Errors,
			min: usize,
			max: usize,
		) -> Result<Option<S::Item>, ()> {
			if *counter == 0 {
				buffer.reserve_exact(min);

				while *counter < min {
					if let Some(item) = inner.pop_next_from(input, errors)? {
						*counter += 1;
						buffer.push_back(item)
					} else {
						todo!("Report error and return.")
					}
				}
			}

			if let Some(item) = buffer.pop_front() {
				Ok(Some(item))
			} else if *counter < max
				&& let Some(item) = inner.pop_next_from(input, errors)?
			{
				*counter += 1;
				Ok(Some(item))
			} else {
				Ok(None)
			}
		}

		pop_next_from(
			&mut self.inner,
			&mut self.buffer,
			&mut self.counter,
			input,
			errors,
			MIN,
			MAX,
		)
	}
}

impl<S: Stepper, const MIN: usize, const MAX: usize> PeekNextFrom
	for RepeatCountStepper<S, MIN, MAX>
where
	S: PeekNextFrom,
{
	fn peek_next_from(&self, input: &Input) -> bool {
		if self.counter < MIN {
			true
		} else if self.counter < MAX {
			self.inner.peek_next_from(input)
		} else {
			false
		}
	}
}

pub struct SeparatedStepper<T, S> {
	stop: bool,
	_phantom: PhantomData<(T, S)>,
}

impl<T, S> Default for SeparatedStepper<T, S> {
	fn default() -> Self {
		Self {
			stop: false,
			_phantom: PhantomData,
		}
	}
}

impl<T: PopParsedFrom, S: PopParsedFrom + PeekFrom> Stepper for SeparatedStepper<T, S> {
	type Item = (T::Parsed, Option<S::Parsed>);

	fn pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Item>, ()> {
		let len_before = input.len();
		let item = match T::pop_parsed_from(input, errors) {
			//TODO: Slide separator!
			Ok(trailing) => match S::peek_pop_parsed_from(input, errors) {
				Ok(delimiter) => (trailing, delimiter),
				Err(()) => todo!("Recovery."),
			},
			Err(()) => todo!("Recovery."),
		};
		if input.len() == len_before {
			errors.push(Error::new(
				ErrorPriority::UNCONSUMED_INPUT,
				format!(
					"{} looped without consuming input. (This likely implies a faulty grammar.)",
					type_name::<(T, Option<S>)>()
				),
				input.drain_spans(..),
			));
			self.stop = true;
		}
		Ok(Some(item))
	}
}

impl<T: PeekFrom, S> PeekNextFrom for SeparatedStepper<T, S> {
	fn peek_next_from(&self, input: &Input) -> bool {
		!self.stop && T::peek_from(input)
	}
}

pub struct DelimitedStepper<T, D> {
	stop: bool,
	_phantom: PhantomData<(T, D)>,
}

impl<T, D> Default for DelimitedStepper<T, D> {
	fn default() -> Self {
		Self {
			stop: false,
			_phantom: PhantomData,
		}
	}
}

impl<T: PopParsedFrom, D: PopParsedFrom + PeekFrom> Stepper for DelimitedStepper<T, D> {
	type Item = (T::Parsed, Option<D::Parsed>);

	fn pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> Result<Option<Self::Item>, ()> {
		let len_before = input.len();
		let item = match T::pop_parsed_from(input, errors) {
			//TODO: Slide separator!
			Ok(trailing) => match D::peek_pop_parsed_from(input, errors) {
				Ok(delimiter) => (trailing, delimiter),
				Err(()) => todo!("Recovery."),
			},
			Err(()) => todo!("Recovery."),
		};
		if input.len() == len_before {
			errors.push(Error::new(
				ErrorPriority::UNCONSUMED_INPUT,
				format!(
					"{} looped without consuming input. (This likely implies a faulty grammar.)",
					type_name::<(T, Option<D>)>()
				),
				input.drain_spans(..),
			));
			self.stop = true;
		}
		Ok(Some(item))
	}
}

impl<T: PeekFrom, D> PeekNextFrom for DelimitedStepper<T, D> {
	fn peek_next_from(&self, input: &Input) -> bool {
		!self.stop && T::peek_from(input)
	}
}
