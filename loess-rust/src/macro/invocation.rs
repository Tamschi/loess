//! [macro.invocation](https://doc.rust-lang.org/reference/macros.html#r-macro.invocation)

use loess::{
	grammar,
	scaffold::{CurlyBraces, Parentheses, SquareBrackets},
};

use crate::{lex::token::{
	Token,
	punct::{Not, Semi},
}, paths::simple::SimplePath};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [MacroInvocation](https://doc.rust-lang.org/reference/macros.html#grammar-MacroInvocation)
	pub struct MacroInvocation: PeekFrom, PopFrom, IntoTokens {
		simple_path: SimplePath,
		not: Not,
		delim_token_tree: DelimTokenTree,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [DelimTokenTree](https://doc.rust-lang.org/reference/macros.html#grammar-DelimTokenTree)
	pub enum DelimTokenTree: PeekFrom, PopFrom, IntoTokens {
		/// Continue inside with <code>[`Vec`]&lt;[`TokenTree`]&gt;</code>.
		Parentheses(Parentheses),
		/// Continue inside with <code>[`Vec`]&lt;[`TokenTree`]&gt;</code>.
		SquareBrackets(SquareBrackets),
		/// Continue inside with <code>[`Vec`]&lt;[`TokenTree`]&gt;</code>.
		CurlyBraces(CurlyBraces),
	} else "Expected delimited token tree.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [TokenTree](https://doc.rust-lang.org/reference/macros.html#grammar-TokenTree)
	pub enum TokenTree: PeekFrom, PopFrom, IntoTokens {
		DelimTokenTree(DelimTokenTree),
		TokenExceptDelimiters(Token),
	} else "Expected delimited token tree.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [MacroInvocationSemi](https://doc.rust-lang.org/reference/macros.html#grammar-MacroInvocationSemi)
	pub enum MacroInvocationSemi: PeekFrom, PopFrom, IntoTokens {
		/// Continue inside with <code>[`Vec`]&lt;[`TokenTree`]&gt;</code>.
		WithParentheses(SimplePath, Not, Parentheses, Semi),
		/// Continue inside with <code>[`Vec`]&lt;[`TokenTree`]&gt;</code>.
		WithSquareBrackets(SimplePath, Not, SquareBrackets, Semi),
		/// Continue inside with <code>[`Vec`]&lt;[`TokenTree`]&gt;</code>.
		WithCurlyBraces(SimplePath, Not, CurlyBraces),
	} else "Expected delimited token tree.";
}
