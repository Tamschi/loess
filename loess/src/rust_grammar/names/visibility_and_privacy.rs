use proc_macro2::{TokenStream, TokenTree};

use crate::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom,
	rust_grammar::{Parentheses, Pub},
};

/// [`pub`](`Pub`) [`(`](`Parentheses`) [`T`](`TokenStream`) [`)`](`Parentheses`)<sup>?</sup>
#[derive(Clone)]
pub struct Visibility<T = TokenStream> {
	#[allow(missing_docs)]
	pub r#pub: Pub,
	#[allow(missing_docs)]
	pub parentheses: Option<Parentheses<T>>,
}

impl<T> PeekFrom for Visibility<T> {
	fn peek_from(input: &Input) -> bool {
		Pub::peek_from(input)
	}
}

impl<T: PopFrom> PopFrom for Visibility<T> {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		if let Some(r#pub) = Pub::peek_pop_from(input, errors)? {
			Ok(Self {
				r#pub,
				parentheses: Parentheses::<TokenStream>::peek_from(input)
					.then(|| Parentheses::pop_from(input, errors))
					.transpose()?,
			})
		} else {
			Err(errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Expected Visibility. (Expected `pub`.)",
				[input.front_span()],
			)))
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
