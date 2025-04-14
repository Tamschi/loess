use std::collections::VecDeque;

use loess::{
	rust_reference::{CurlyBraces, Identifier, RArrow},
	Defaulted, Error, Errors, Input, IntoTokens, PopFrom, SimpleSpanned,
};
use proc_macro2::{Ident, Span, TokenTree};
use quote::{quote_spanned, ToTokens};

#[derive(Debug)]
pub struct Component {
	name: Identifier,
	r_arrow: RArrow,
	substrate: Identifier,
	body: CurlyBraces<Vec<Identifier>>,
}

impl PopFrom for Component {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self {
			name: Identifier::pop_from(input, errors)?,
			r_arrow: RArrow::pop_from(input, errors)?,
			substrate: Identifier::pop_from(input, errors)?,
			body: CurlyBraces::pop_from(input, errors)?,
		})
	}
}

impl IntoTokens for Component {
	fn into_tokens(self, root: &proc_macro2::TokenStream, tokens: &mut impl Extend<TokenTree>) {
		let Self {
			name,
			r_arrow,
			substrate,
			body,
		} = self;
		quote_spanned! {name.span().resolved_at(Span::mixed_site())=>
			struct #name {}
		}
		.into_tokens(root, tokens)
	}
}
