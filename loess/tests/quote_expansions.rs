#![deny(unused_variables)] // At least for now, this is used to detect missing expansions.

use std::arch::x86_64::_CMP_FALSE_OQ;

use loess::{
	quote_into_call_site, quote_into_mixed_site, quote_into_same_site, raw_quote_into_call_site,
	raw_quote_into_mixed_site, raw_quote_into_same_site,
};
use proc_macro2::{Span, TokenStream, TokenTree};

macro_rules! test {
	(let ($span:ident, $root:ident, $tokens:ident), $test:expr, $expected:expr) => {{
		let $span = Span::call_site();
		let $root: &TokenStream = &TokenStream::new();
		let mut tokens = TokenStream::new();
		let $tokens = &mut tokens;
		let result = $test;
		assert_eq!(tokens.to_string(), $expected);
		result
	}};
}

//TODO: Wrap the six functions below into tests.

#[test]
pub fn mixed_site() {
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [....... .....]), "....... .....");
}

#[test]
pub fn same_site() {
	test!(let (span, root, tokens), quote_into_same_site!(span, root, tokens, [....... .....]), "....... .....");
}

#[test]
pub fn call_site() {
	test!(let (span, root, tokens), quote_into_call_site!(span, root, tokens, [....... .....]), "....... .....");
}

#[test]
pub fn mixed_site_raw() {
	test!(let (span, _root, tokens), raw_quote_into_mixed_site!(span, tokens, [....... .....]), "....... .....");
}

#[test]
pub fn same_site_raw() {
	test!(let (span, _root, tokens), raw_quote_into_same_site!(span, tokens, [....... .....]), "....... .....");
}

#[test]
pub fn call_site_raw() {
	test!(let (span, _root, tokens), raw_quote_into_call_site!(span, tokens, [....... .....]), "....... .....");
}

#[test]
pub fn long_punctuation() {
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [............... .....]), "............... .....");
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
pub fn r#match() {
	fn r#match(
		span: Span,
		root: &TokenStream,
		tokens: &mut impl Extend<TokenTree>,
		condition: Option<bool>,
	) {
		quote_into_mixed_site!(span, root, tokens, [
			{#match condition,
				Some(true) => {+}
				Some(false) => {-}
				None => {~}
			}
		]);
	}

	test!(let (span, root, tokens), r#match(span, root, tokens, Some(true)), "+");
	test!(let (span, root, tokens), r#match(span, root, tokens, Some(false)), "-");
	test!(let (span, root, tokens), r#match(span, root, tokens, None), "~");
}

#[test]
pub fn match_blocks_else() {
	fn match_blocks_else(
		span: Span,
		root: &TokenStream,
		tokens: &mut impl Extend<TokenTree>,
		condition: Option<bool>,
	) {
		quote_into_mixed_site!(span, root, tokens, [
			{#if false,}
			{#match condition,
				Some(true) => {+}
				Some(false) => {-}
				None => {~}
			}
			{#else, never}
		]);
	}

	test!(let (span, root, tokens), match_blocks_else(span, root, tokens, Some(true)), "+");
	test!(let (span, root, tokens), match_blocks_else(span, root, tokens, Some(false)), "-");
	test!(let (span, root, tokens), match_blocks_else(span, root, tokens, None), "~");
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

#[test]
fn braced() {
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [{braced tokens}]), "{ braced tokens }");
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [{{double braced tokens}}]), "{ { double braced tokens } }");
}

#[test]
fn bracketed() {
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [[bracketed tokens]]), "[bracketed tokens]");
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [[[double bracketed tokens]]]), "[[double bracketed tokens]]");
}

#[test]
fn parenthesized() {
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [(parenthesized tokens)]), "(parenthesized tokens)");
	test!(let (span, root, tokens), quote_into_mixed_site!(span, root, tokens, [((double parenthesized tokens))]), "((double parenthesized tokens))");
}
