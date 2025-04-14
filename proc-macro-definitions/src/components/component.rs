use loess::{
	grammar,
	rust_grammar::{
		Async, Const, CurlyBraces, Identifier, Parentheses, RArrow, SquareBrackets, Visibility,
	},
	IntoTokens, SimpleSpanned,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::quote_spanned;
use statements::Statement;

pub mod statements;

grammar! {
	pub struct Component: PopFrom, IntoTokens {
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

impl Component {
	pub fn transform(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
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
