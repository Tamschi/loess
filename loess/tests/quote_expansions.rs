#![deny(unused_variables)] // At least for now, this is used to detect missing expansions.

use loess::{quote_into_call_site, quote_into_mixed_site, quote_into_same_site};
use proc_macro2::{Ident, Span, TokenStream, TokenTree};

pub fn mixed_site(span: Span, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
	quote_into_mixed_site!(span, root, tokens, []);
}

pub fn same_site(span: Span, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
	quote_into_same_site!(span, root, tokens, []);
}

pub fn call_site(span: Span, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
	quote_into_call_site!(span, root, tokens, []);
}

macro_rules! test {
	(let ($span:ident, $root:ident, $tokens:ident), $test:expr, $expected:expr) => {{
		let $span = Span::call_site();
		let $root: &TokenStream = &TokenStream::new();
		let $tokens = &mut TokenStream::new();
		let result = $test;
		assert_eq!($tokens.to_string(), $expected);
		result
	}};
}

#[test]
pub fn if_else_chain() {
	fn if_else_chain(
		span: Span,
		root: &TokenStream,
		tokens: &mut impl Extend<TokenTree>,
		condition: Option<bool>,
	) {
		quote_into_mixed_site!(span, root, tokens, [
			{#if condition == Some(true),
				+
			} {#else if condition == Some(false),
				-
			} {#else,
				~
			}

			{#else,
				never
			}
		]);
	}

	test!(let (span, root, tokens), if_else_chain(span, root, tokens, Some(true)), "+");
	test!(let (span, root, tokens), if_else_chain(span, root, tokens, Some(false)), "-");
	test!(let (span, root, tokens), if_else_chain(span, root, tokens, None), "~");
}

#[test]
pub fn if_let_else_chain() {
	fn if_let_else_chain(
		span: Span,
		root: &TokenStream,
		tokens: &mut impl Extend<TokenTree>,
		condition: Option<bool>,
	) {
		quote_into_mixed_site!(span, root, tokens, [
			{#if let Some(true) = condition,
				+
			} {#else if let Some(false) = condition,
				-
			} {#else,
				~
			}

			{#else,
				never
			}
		]);
	}

	test!(let (span, root, tokens), if_let_else_chain(span, root, tokens, Some(true)), "+");
	test!(let (span, root, tokens), if_let_else_chain(span, root, tokens, Some(false)), "-");
	test!(let (span, root, tokens), if_let_else_chain(span, root, tokens, None), "~");
}

#[test]
pub fn r#return() {
	fn r#return(
		span: Span,
		root: &TokenStream,
		tokens: &mut impl Extend<TokenTree>,
		condition: bool,
	) -> bool {
		quote_into_mixed_site!(span, root, tokens, [
			{#if condition,
				{#return !condition}
			}
			not condition
		]);
		true
	}

	assert_eq!(
		test!(let (span, root, tokens), r#return(span, root, tokens, true), ""),
		false
	);
	assert_eq!(
		test!(let (span, root, tokens), r#return(span, root, tokens, false), "not condition"),
		true
	);
}

//TODO: More tests!

fn my_quote(id1: Ident, id2: Option<Ident>, root: &TokenStream) -> TokenStream {
	let mut output = TokenStream::new();

	quote_into_mixed_site!(id1.span(), root, &mut output, [
		pub struct {#paste id1};

		{#if let Some(id2) = id2,
			{#located_at id2.span(),
				pub struct {#paste id2};
			}
		} {#else,
            //TODO:
			// {#error, "`id2` is missing."}
		}
	]);

	output
}
