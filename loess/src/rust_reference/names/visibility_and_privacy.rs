use proc_macro2::{Ident, TokenStream, TokenTree};

use crate::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, SpanOrFrontOfExt,
	rust_reference::Parentheses,
};

pub struct Visibility<T = TokenStream> {
	pub r#pub: Ident,
	pub parentheses: Option<Parentheses<T>>,
}

/// Checks for `pub`.
impl<T> PeekFrom for Visibility<T> {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Ident(ident)) if ident == "pub")
	}
}

impl<T: PopFrom> PopFrom for Visibility<T> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		let ident = Ident::peek_pop_from(input, errors)?;

		match ident {
			Some(r#pub) if r#pub == "pub" => Ok(Self {
				r#pub,
				parentheses: Parentheses::<TokenStream>::peek_from(input)
					.then(|| Parentheses::pop_from(input, errors))
					.transpose()?,
			}),
			ident => Err({
				let span = ident.span_or_front_of(input);

				errors.push(Error::new(
					ErrorPriority::GRAMMAR,
					"Expected Visibility. (Expected `pub`.)",
					[span],
				));

				if let Some(ident) = ident {
					input.push_front(TokenTree::Ident(ident));
				}
			}),
		}
	}
}

impl IntoTokens for Visibility {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		let Self { r#pub, parentheses } = self;
		r#pub.into_tokens(root, tokens);
		parentheses.into_tokens(root, tokens);
	}
}
