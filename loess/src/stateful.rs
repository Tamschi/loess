use std::marker::PhantomData;

use crate::{Errors, Input, PeekFrom, PopParsedFrom};

pub trait PopNextFrom {
	type Item;

	fn pop_next_from(&mut self, input: &mut Input, errors: &mut Errors) -> Option<Self::Item>;

	fn peek_pop_next_from(&mut self, input: &mut Input, errors: &mut Errors) -> Option<Self::Item>
	where
		Self: PeekNextFrom,
	{
		self.peek_next_from(input)
			.then_some(self.pop_next_from(input, errors))
			.flatten()
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

pub struct SimpleStepper<T>(PhantomData<T>);

impl<T> Default for SimpleStepper<T> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<T: PopParsedFrom> PopNextFrom for SimpleStepper<T> {
	type Item = T::Parsed;

	fn pop_next_from(&mut self, input: &mut Input, errors: &mut Errors) -> Option<Self::Item> {
		T::pop_parsed_from(input, errors).map_or_else(|()| None, Some)
	}
}

impl<T: PeekFrom> PeekNextFrom for SimpleStepper<T> {
	fn peek_next_from(&self, input: &Input) -> bool {
		T::peek_from(input)
	}
}
