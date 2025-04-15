use proc_macro2::{Literal, TokenTree};

use crate::{Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, grammar};

grammar! {
	pub struct StringLiteral(pub Literal);
	pub struct RawStringLiteral(pub Literal);

	pub enum AnyStringLiteral: PeekFrom, PopFrom, IntoTokens {
		Plain(StringLiteral),
		Raw(RawStringLiteral),
	} else "Expected &str literal.";
}

impl PeekFrom for StringLiteral {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Literal(literal)) if literal.to_string().starts_with('"'))
	}
}

impl PopFrom for StringLiteral {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|t| match t {
				[TokenTree::Literal(literal)] if literal.to_string().starts_with('"') => {
					Ok(Self(literal))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `\"`.", spans))
			})
	}
}

impl IntoTokens for StringLiteral {
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.0.into_tokens(root, tokens);
	}
}

impl PeekFrom for RawStringLiteral {
	fn peek_from(input: &Input) -> bool {
		//TODO: This might not be entirely accurate.
		matches!(input.front(), Some(TokenTree::Literal(literal)) if literal.to_string().starts_with('r'))
	}
}

impl PopFrom for RawStringLiteral {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|t| match t {
				[TokenTree::Literal(literal)] if literal.to_string().starts_with('r') => {
					Ok(Self(literal))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(
					ErrorPriority::GRAMMAR,
					"Expected raw string literal.",
					spans,
				))
			})
	}
}

impl IntoTokens for RawStringLiteral {
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.0.into_tokens(root, tokens);
	}
}
