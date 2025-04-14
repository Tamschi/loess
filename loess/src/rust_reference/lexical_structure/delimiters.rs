use std::collections::VecDeque;

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree, extra::DelimSpan};
use quote::ToTokens;

use crate::{
	Error, ErrorPriority, Errors, Exhaustive, Input, PopFrom,
	error_priorities::UNCONSUMED_IN_DELIMITER,
};

#[derive(Debug)]
pub struct CurlyBraces<T> {
	pub span: DelimSpan,
	pub contents: T,
}

impl<T> CurlyBraces<T> {
	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> CurlyBraces<U> {
		let Self { span, contents } = self;
		CurlyBraces {
			span,
			contents: f(contents),
		}
	}

	pub fn try_map<U, E>(
		self,
		f: impl FnOnce(T) -> Result<U, E>,
	) -> Result<CurlyBraces<U>, CurlyBraces<E>> {
		let Self { span, contents } = self;
		CurlyBraces {
			span,
			contents: f(contents),
		}
		.transpose()
	}
}

impl<T, E> CurlyBraces<Result<T, E>> {
	pub fn transpose(self) -> Result<CurlyBraces<T>, CurlyBraces<E>> {
		let Self { span, contents } = self;
		match contents {
			Ok(contents) => Ok(CurlyBraces { span, contents }),
			Err(contents) => Err(CurlyBraces { span, contents }),
		}
	}
}

impl<T: PopFrom> PopFrom for CurlyBraces<T> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		dbg!(());
		let (span, mut contents) = input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Group(braces)] if braces.delimiter() == Delimiter::Brace => Ok((
					braces.delim_span(),
					Input {
						tokens: braces.stream().into_iter().collect::<VecDeque<_>>(),
						end: braces.span_close(),
					},
				)),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected `{…}`.", spans))
			})?;

		dbg!("ok");
		Ok(Self {
			span,
			contents: Exhaustive::<T, UNCONSUMED_IN_DELIMITER>::pop_from(&mut contents, errors)?.0,
		})
	}
}

impl<T: ToTokens> ToTokens for CurlyBraces<T> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self { span, contents } = self;
		let mut group = Group::new(Delimiter::Brace, contents.to_token_stream());
		group.set_span(span.join());
		group.to_tokens(tokens)
	}

	fn into_token_stream(self) -> TokenStream
	where
		Self: Sized,
	{
		let Self { span, contents } = self;
		let mut group = Group::new(Delimiter::Brace, contents.into_token_stream());
		group.set_span(span.join());
		group.into_token_stream()
	}
}
