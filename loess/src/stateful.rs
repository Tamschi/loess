use std::{
	any::type_name,
	collections::VecDeque,
	marker::PhantomData,
	ops::ControlFlow::{self, Break, Continue},
};

use crate::{Error, ErrorPriority, Errors, Input, PeekFrom, PopParsedFrom};

pub trait Stepper: Default {
	type Item;

	fn pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Item>, Option<Self::Item>>;

	fn peek_pop_next_from(
		&mut self,
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self::Item>, Option<Self::Item>>
	where
		Self: PeekNextFrom,
	{
		if self.peek_next_from(input) {
			self.pop_next_from(input, errors)
		} else {
			Continue(None)
		}
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
	) -> ControlFlow<Option<Self::Item>, Option<Self::Item>> {
		T::pop_parsed_from(input, errors)
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
	) -> ControlFlow<Option<Self::Item>, Option<Self::Item>> {
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
		) -> ControlFlow<Option<S::Item>, Option<S::Item>> {
			if *counter == 0 {
				buffer.reserve_exact(min);

				while *counter < min
					&& let Some(item) = inner.pop_next_from(input, errors)?
				{
					*counter += 1;
					buffer.push_back(item)
				}
			}

			if let Some(item) = buffer.pop_front() {
				Continue(Some(item))
			} else if *counter < max {
				let item = inner.pop_next_from(input, errors)?;
				*counter += 1;
				Continue(item)
			} else {
				todo!("RepeatCountStepper::pop_next_from: Report error.")
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
	) -> ControlFlow<Option<Self::Item>, Option<Self::Item>> {
		if self.stop {
			Continue(None)
		} else {
			//TODO: Recovery.
			match T::pop_parsed_from(input, errors).map_break(|_| None)? {
				Some(t) => Continue(Some((
					t,
					S::peek_pop_parsed_from(input, errors).map_break(|_| None)?,
				))),
				None => Continue(None),
			}
		}
	}
}

impl<T: PeekFrom, S> PeekNextFrom for SeparatedStepper<T, S> {
	fn peek_next_from(&self, input: &Input) -> bool {
		!self.stop && T::peek_from(input)
	}
}

impl<T, S> SeparatedStepper<T, S>
where
	T: PopParsedFrom,
	S: PopParsedFrom + PeekFrom,
{
	fn recover(input: &mut Input, errors: &mut Errors) -> ControlFlow<(), S::Parsed>
	where
		S: PopParsedFrom + PeekFrom,
	{
		todo!("SeparatedStepper::recover")
		// while !input.is_empty() {
		// 	let len_before = input.len();
		// 	match S::peek_pop_parsed_from(input, errors) {
		// 		Ok(Some(s)) => {
		// 			return Continue(s);
		// 		}
		// 		Ok(None) => {
		// 			assert_eq!(
		// 				input.len(),
		// 				len_before,
		// 				"`S::peek_pop_parsed_from` should not consume tokens if it returns `Ok(None)`."
		// 			);
		// 			drop(input.tokens.pop_front().expect(""));
		// 		}
		// 		Err(_) => {
		// 			if input.len() == len_before {
		// 				drop(input.tokens.pop_front().expect("unreachable"));
		// 			}
		// 		}
		// 	}
		// 	assert!(
		// 		input.len() < len_before,
		// 		"Input didn't shrink during `Separated::collect_repeats` recovery."
		// 	);
		// }
		// Break(())
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
	) -> ControlFlow<Option<Self::Item>, Option<Self::Item>> {
		todo!("DelimitedStepper::pop_next_from")
		// let len_before = input.len();
		// let item = match T::pop_parsed_from(input, errors) {
		// 	//TODO: Slide separator!
		// 	Ok(trailing) => match D::peek_pop_parsed_from(input, errors) {
		// 		Ok(delimiter) => (trailing, delimiter),
		// 		Err(delimiter) => todo!("Recovery."),
		// 	},
		// 	Err(trailing) => todo!("Recovery."),
		// };
		// if input.len() == len_before {
		// 	errors.push(Error::new(
		// 		ErrorPriority::UNCONSUMED_INPUT,
		// 		format!(
		// 			"{} looped without consuming input. (This likely implies a faulty grammar.)",
		// 			type_name::<(T, Option<D>)>()
		// 		),
		// 		input.drain_spans(..),
		// 	));
		// 	self.stop = true;
		// }
		// Ok(item)
	}
}

impl<T: PeekFrom, D> PeekNextFrom for DelimitedStepper<T, D> {
	fn peek_next_from(&self, input: &Input) -> bool {
		!self.stop && T::peek_from(input)
	}
}
