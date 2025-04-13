use std::collections::VecDeque;

use proc_macro2::{Delimiter, TokenTree, extra::DelimSpan};
use quote::ToTokens;

use crate::{Error, PopFrom, PopOrReplaceExt};

pub struct CurlyBraces<T> {
	pub span: DelimSpan,
	pub contents: T,
}

impl<T: PopFrom> PopFrom for CurlyBraces<T> {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Vec<Error>) -> Result<Self, ()> {
		let (span, mut contents) = input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Group(braces)] if braces.delimiter() == Delimiter::Brace => Ok((
					braces.delim_span(),
					braces.stream().into_iter().collect::<VecDeque<_>>(),
				)),
				other => Err(other),
			})
			.map_err(|spans| errors.push(Error::new("Expected `{…}`.", spans)))?;

		Ok(Self {
			span,
			contents: T::pop_from(&mut contents, errors)?,
		})
	}
}
