use std::collections::VecDeque;

use loess::{
	rust_reference::{CurlyBraces, Identifier, RArrow},
	Defaulted, Error, Errors, PopFrom, SimpleSpanned,
};
use proc_macro2::{Ident, Span, TokenTree};
use quote::{quote_spanned, ToTokens};

#[derive(Debug)]
pub struct Component {
	name: Identifier,
	r_arrow: RArrow,
	substrate: Identifier,
	body: CurlyBraces<Vec<Ident>>,
}

impl PopFrom for Component {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self {
			name: Identifier::pop_from(input, errors)?,
			r_arrow: RArrow::pop_from(input, errors)?,
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
			struct #name {}
		}
		.to_tokens(tokens)
	}
}
