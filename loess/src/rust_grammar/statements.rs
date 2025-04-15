use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
	Stmt,
	parse::{Parse, ParseStream, Parser},
};

use crate::{Error, ErrorPriority, Errors, Input, IntoTokens, PopFrom, grammar};

grammar! {
	/// Opaque implementation. Contributions welcome!
	#[derive(Clone)]
	pub struct Statement {
		syn: Stmt,
	}
}

impl PopFrom for Statement {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(input: ParseStream) -> syn::Result<(Statement, TokenStream)> {
			Ok((
				Statement {
					syn: Stmt::parse(input)?,
				},
				TokenStream::parse(input)?,
			))
		}

		let error_span = input
			.front_span()
			.join(input.end)
			.unwrap_or(input.front_span());

		let tokens = input.tokens.drain(..).collect::<TokenStream>().into();
		let (this, rest) = parse.parse2(tokens).map_err(|error| {
			errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Expected Statement.",
				[error_span],
			));
		})?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}

impl IntoTokens for Statement {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.syn.into_token_stream().into_tokens(root, tokens);
	}
}
