use loess::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, PopParsedFrom,
	scaffold::Parentheses,
};
use proc_macro2::{TokenStream, TokenTree};

use crate::Pub;

/// [*Visibility*](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html#r-vis.syntax):
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

impl<T: PopParsedFrom> PopParsedFrom for Visibility<T> {
	type Parsed = Visibility<T::Parsed>;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self::Parsed, ()> {
		if let Some(r#pub) = Pub::peek_pop_from(input, errors)? {
			Ok(Self::Parsed {
				r#pub,
				parentheses: Parentheses::<TokenStream>::peek_from(input)
					.then(|| Parentheses::<T>::pop_parsed_from(input, errors))
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
