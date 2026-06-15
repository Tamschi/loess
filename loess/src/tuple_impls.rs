use std::ops::ControlFlow::{self, Break, Continue};

use crate::{IntoTokens, PeekFrom, PopParsedFrom};

impl<T: ?Sized> PeekFrom for (T,)
where
	T: PeekFrom,
{
	fn peek_from(input: &crate::Input) -> bool {
		T::peek_from(input)
	}
}

impl<T: ?Sized> PopParsedFrom for (T,)
where
	T: PopParsedFrom,
{
	type Parsed = (T::Parsed,);

	fn pop_parsed_from(
		input: &mut crate::Input,
		errors: &mut crate::Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		T::pop_parsed_from(input, errors)
			.map_continue(|o| o.map(|t| (t,)))
			.map_break(|o| o.map(|t| (t,)))
	}
}

impl<T> IntoTokens for (T,)
where
	T: IntoTokens,
{
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.0.into_tokens(root, tokens)
	}
}

impl<T1, T2: ?Sized> PeekFrom for (T1, T2)
where
	T1: PeekFrom,
{
	fn peek_from(input: &crate::Input) -> bool {
		T1::peek_from(input)
	}
}

impl<T1, T2: ?Sized> PopParsedFrom for (T1, T2)
where
	T1: PopParsedFrom,
	T2: PopParsedFrom,
{
	type Parsed = (T1::Parsed, T2::Parsed);

	fn pop_parsed_from(
		input: &mut crate::Input,
		errors: &mut crate::Errors,
	) -> ControlFlow<Option<Self::Parsed>, Option<Self::Parsed>> {
		let t1 = T1::pop_parsed_from(input, errors).map_break(|_| None)?;
		match T2::pop_parsed_from(input, errors) {
			Continue(t2) => Continue(t1.zip(t2)),
			Break(t2) => Break(t1.zip(t2)),
		}
	}
}

impl<T1, T2> IntoTokens for (T1, T2)
where
	T1: IntoTokens,
	T2: IntoTokens,
{
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.0.into_tokens(root, tokens);
		self.1.into_tokens(root, tokens);
	}
}

//TODO
