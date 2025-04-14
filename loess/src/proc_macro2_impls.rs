use crate::{ErrorPriority, Errors, Input, PeekFrom, SimpleSpanned};

use super::{Error, PopFrom};
use proc_macro2::{Ident, Span, TokenTree};

impl PopFrom for Ident {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t| match t {
				[TokenTree::Ident(ident)] => Ok(ident),
				t => Err(t),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Ident.", spans))
			})
	}
}

impl PeekFrom for Ident {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Ident(_)))
	}
}

impl SimpleSpanned for Ident {
	fn span(&self) -> Span {
		self.span()
	}
}
