use std::marker::PhantomData;

use crate::{Errors, Input, PeekFrom, PopParsedFrom};

pub trait Stepper<'a> {
	type Item;

	fn attach(input: &'a mut Input, errors: &'a mut Errors) -> Self;

	fn pop_next(&mut self) -> Option<Self::Item>;

	fn peek_pop_next(&mut self) -> Option<Self::Item>
	where
		Self: PeekNext,
	{
		self.peek_next().then_some(self.pop_next()).flatten()
	}

	fn input(&self) -> &Input;
	fn input_mut(&mut self) -> &mut Input {
		self.split_mut().0
	}
	fn errors(&self) -> &Errors;
	fn errors_mut(&mut self) -> &mut Errors {
		self.split_mut().1
	}
	fn split_mut(&mut self) -> (&mut Input, &mut Errors);
}

pub trait PeekNext {
	/// # Returns
	///
	/// ## [`true`]
	///
	/// [`StatefulPopParsedFrom::pop_parsed_from`] <em style=font-style:normal;font-variant:small-caps>may</em> still fail and/or push to [`Errors`].
	///
	/// ## [`false`]
	///
	/// [`StatefulPopParsedFrom::pop_parsed_from`] <em style=font-style:normal;font-variant:small-caps>should</em> fail **and** push to [`Errors`].
	fn peek_next(&self) -> bool;
}

pub struct SimpleStepper<'a, T> {
	input: &'a mut Input,
	errors: &'a mut Errors,
	_phantom: PhantomData<T>,
}

impl<'a, T: PopParsedFrom> Stepper<'a> for SimpleStepper<'a, T> {
	type Item = T::Parsed;

	fn attach(input: &'a mut Input, errors: &'a mut Errors) -> Self {
		Self {
			input,
			errors,
			_phantom: PhantomData,
		}
	}

	fn pop_next(&mut self) -> Option<Self::Item> {
		T::pop_parsed_from(self.input, self.errors).map_or_else(|()| None, Some)
	}

	fn input(&self) -> &Input {
		self.input
	}

	fn errors(&self) -> &Errors {
		self.errors
	}

	fn split_mut(&mut self) -> (&mut Input, &mut Errors) {
		(self.input, self.errors)
	}
}

impl<'a, T: PeekFrom> PeekNext for SimpleStepper<'a, T> {
	fn peek_next(&self) -> bool {
		T::peek_from(self.input)
	}
}

pub struct RepeatConstraint<S, const MIN: usize, const MAX: usize> {
	inner: S,
	counter: usize,
}

impl<'a, S: Stepper<'a>, const MIN: usize, const MAX: usize> Stepper<'a>
	for RepeatConstraint<S, MIN, MAX>
{
	type Item = S::Item;

	fn attach(input: &'a mut Input, errors: &'a mut Errors) -> Self {
		Self {
			inner: S::attach(input, errors),
			counter: 0,
		}
	}

	fn pop_next(&mut self) -> Option<Self::Item> {
		self.inner.pop_next()
	}

	fn input(&self) -> &Input {
		self.inner.input()
	}

	fn errors(&self) -> &Errors {
		self.inner.errors()
	}

	fn split_mut(&mut self) -> (&mut Input, &mut Errors) {
		self.inner.split_mut()
	}
}

impl<'a, S: Stepper<'a>, const MIN: usize, const MAX: usize> PeekNext
	for RepeatConstraint<S, MIN, MAX>
where
	S: PeekNext,
{
	fn peek_next(&self) -> bool {
		(self.counter < MIN) || self.inner.peek_next()
	}
}

impl<S, const MIN: usize, const MAX: usize> Drop for RepeatConstraint<S, MIN, MAX> {
	fn drop(&mut self) {
		todo!()
	}
}
