use loess::{
	rust_reference::{CurlyBraces, Identifier, RArrow, Visibility},
	Errors, Input, IntoTokens, PopFrom, SimpleSpanned,
};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote_spanned;

pub struct Component {
	visibility: Option<Visibility>,
	name: Identifier,
	r_arrow: RArrow,
	substrate: Identifier,
	body: CurlyBraces<Vec<Identifier>>,
}

impl PopFrom for Component {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self {
			visibility: Visibility::peek_pop_from(input, errors)?,
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
			visibility,
			name,
			r_arrow,
			substrate,
			body,
		} = self;
		let visibility = visibility.collect_tokens::<TokenStream>(root);
		quote_spanned! {name.span().resolved_at(Span::mixed_site())=>
			#visibility struct #name {}
		}
		.into_tokens(root, tokens)
	}
}
