use std::collections::VecDeque;

use loess::{
	rust_reference::{CurlyBraces, Identifier, RArrow},
	Error, PopFrom, SimpleSpanned, UnwrapOrPlaceholder,
};
use proc_macro2::{Ident, Span, TokenTree};
use quote::{quote_spanned, ToTokens};

pub struct Component {
	name: Identifier,
	r_arrow: RArrow,
	substrate: Identifier,
	body: CurlyBraces<Vec<Ident>>,
}

impl PopFrom for Component {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Vec<Error>) -> Result<Self, ()> {
		Ok(Self {
			name: Identifier::pop_from(input, errors)?,
			r_arrow: RArrow::pop_from(input, errors).unwrap_or_default(),
			substrate: Identifier::pop_from(input, errors)?,
			body: CurlyBraces::pop_from(input, errors)?,
		})
	}
}

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let Self {
			name,
			r_arrow,
			substrate,
			body,
		} = self;
		quote_spanned! {name.span().resolved_at(Span::mixed_site())=>
		}
		.to_tokens(tokens)
	}
}
