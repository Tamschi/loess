use crate::{ErrorPriority, Errors, Input, IntoTokens, PeekFrom, SimpleSpanned};

use super::{Error, PopFrom};
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::ToTokens;

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

impl IntoTokens for Ident {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([TokenTree::Ident(self)]);
	}
}

impl IntoTokens for Literal {
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.into_token_stream().into_tokens(root, tokens);
	}
}
