use loess::{
	grammar, quote_into_with_exact_span,
	rust_grammar::{
		Async, Const, CurlyBraces, Identifier, Parentheses, RArrow, SquareBrackets, Visibility,
	},
	IntoTokens, SimpleSpanned,
};
use proc_macro2::{TokenStream, TokenTree};
use statements::Statement;

pub mod statements;

grammar! {
	pub struct Component: PopFrom {
		pub visibility: Option<Visibility>,
		pub r#const: Option<Const>,
		pub r#async: Option<Async>,
		pub name: Identifier,
		pub constructor_args: Option<Parentheses>,
		pub render_args: Option<SquareBrackets>,
		pub r_arrow: RArrow,
		pub substrate: Identifier,
		pub body: CurlyBraces<Vec<Statement>>,
	}
}

impl IntoTokens for Component {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
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

		quote_into_with_exact_span!(name.span(), root, tokens, [
			{#paste visibility } struct {#paste name } {}
		]);
	}
}
