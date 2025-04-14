use loess::{
	rust_reference::{
		Async, Const, CurlyBraces, Identifier, Parentheses, RArrow, SquareBrackets, Visibility,
	},
	Errors, Input, IntoTokens, PopFrom, SimpleSpanned,
};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote_spanned;
use statements::Statement;

pub mod statements;

pub struct Component {
	visibility: Option<Visibility>,
	r#const: Option<Const>,
	r#async: Option<Async>,
	name: Identifier,
	constructor_args: Option<Parentheses>,
	render_args: Option<SquareBrackets>,
	r_arrow: RArrow,
	substrate: Identifier,
	body: CurlyBraces<Vec<Statement>>,
}

impl PopFrom for Component {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self {
			visibility: Visibility::peek_pop_from(input, errors)?,
			r#const: Const::peek_pop_from(input, errors)?,
			r#async: Async::peek_pop_from(input, errors)?,
			name: Identifier::pop_from(input, errors)?,
			constructor_args: Parentheses::peek_pop_from(input, errors)?,
			render_args: SquareBrackets::peek_pop_from(input, errors)?,
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
			r#const,
			r#async,
			name,
			constructor_args,
			render_args,
			r_arrow,
			substrate,
			body,
		} = self;
		let visibility = visibility.collect_tokens::<TokenStream>(root);

		// The verbatim span appears to be necessary here to show warnings about unused structs.
		quote_spanned! {name.span()=>
			#visibility struct #name {}
		}
		.into_tokens(root, tokens)
	}
}
