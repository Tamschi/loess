use std::collections::VecDeque;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;

use crate::{
	Error, ErrorPriority, Errors, Placeholder, PopFrom, SimpleSpanned, next_placeholder_number,
};

#[derive(Debug)]
pub struct Identifier(pub Ident);

/// See <https://doc.rust-lang.org/stable/reference/identifiers.html?highlight=IDENTIFIER#identifiers> as of 2025-04-13.
impl PopFrom for Identifier {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()> {
		let ident = Ident::pop_from(input, errors)?;
		if (&["r#crate", "r#self", "r#super", "r#Self"])
			.into_iter()
			.any(|s| ident == s)
			|| is_strict_keyword(&ident)
			|| is_reserved_keyword(&ident)
		{
			let span = ident.span();
			input.push_front(TokenTree::Ident(ident));
			Err(errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Expected Identifier.",
				[span],
			)))
		} else {
			Ok(Self(ident))
		}
	}
}

impl ToTokens for Identifier {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens)
	}

	fn into_token_stream(self) -> TokenStream {
		self.0.into_token_stream()
	}
}

impl SimpleSpanned for Identifier {
	fn span(&self) -> Span {
		self.0.span()
	}
}

impl Placeholder for Identifier {
	fn placeholder() -> Self {
		Self(Ident::new(
			&format!("PLACEHOLDER_IDENTIFIER_{}", next_placeholder_number()),
			Span::mixed_site(),
		))
	}
}

/// See <https://doc.rust-lang.org/stable/reference/keywords.html#strict-keywords> as of 2025-04-13.
pub fn is_strict_keyword(ident: &Ident) -> bool {
	[
		"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
		"for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
		"return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
		"use", "where", "while", //
		// 2018 edition
		"async", "await", "dyn",
	]
	.iter()
	.any(|s| ident == s)
}

/// See <https://doc.rust-lang.org/stable/reference/keywords.html#reserved-keywords> as of 2025-04-13.
pub fn is_reserved_keyword(ident: &Ident) -> bool {
	[
		"abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof",
		"unsized", "virtual", "yield", //
		// 2018+
		"try", //
		// 2024+
		"gen",
	]
	.iter()
	.any(|s| ident == s)
}
