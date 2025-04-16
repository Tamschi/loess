use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
	Path,
	parse::{Parse, ParseStream, Parser},
};

use crate::{Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, grammar};

grammar! {
	/// Opaque implementation. Contributions welcome!
	#[derive(Clone)]
	pub struct SimplePath {
		syn: Path,
	}
}

/// For now, **always** succeeds.
impl PeekFrom for SimplePath {
	fn peek_from(_input: &Input) -> bool {
		true
	}
}

impl PopFrom for SimplePath {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(input: ParseStream) -> syn::Result<(SimplePath, TokenStream)> {
			Ok((
				SimplePath {
					syn: Path::parse_mod_style(input)?,
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
				"Expected SimplePath.",
				[error_span],
			));
		})?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}

impl IntoTokens for SimplePath {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.syn.into_token_stream().into_tokens(root, tokens);
	}
}
