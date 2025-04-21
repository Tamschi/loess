use crate::{ErrorPriority, Errors, Input, IntoTokens, PeekFrom, SimpleSpanned};

use super::{Error, PopFrom};
use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};

impl PeekFrom for Group {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Group(_)))
	}
}

impl PeekFrom for Ident {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Ident(_)))
	}
}

impl PeekFrom for Punct {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Punct(_)))
	}
}

impl PeekFrom for Literal {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Literal(_)))
	}
}

impl PeekFrom for TokenTree {
	fn peek_from(input: &Input) -> bool {
		!input.is_empty()
	}
}

/// **Always** succeeds.
impl PeekFrom for TokenStream {
	fn peek_from(_input: &Input) -> bool {
		true
	}
}

impl PopFrom for Group {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Group(group)] => Ok(group),
				t => Err(t),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Group.", spans))
			})
	}
}

impl PopFrom for Ident {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Ident(ident)] => Ok(ident),
				t => Err(t),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Ident.", spans))
			})
	}
}

impl PopFrom for Punct {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Punct(punct)] => Ok(punct),
				t => Err(t),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Punct.", spans))
			})
	}
}

impl PopFrom for Literal {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Literal(literal)] => Ok(literal),
				t => Err(t),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Literal.", spans))
			})
	}
}

impl PopFrom for TokenTree {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input.pop_or_replace(|[t], _| Ok(t)).map_err(|spans| {
			errors.push(Error::new(ErrorPriority::TOKEN, "Expected token.", spans))
		})
	}
}

/// Exhaustive, infallible.
impl PopFrom for TokenStream {
	fn pop_from(input: &mut Input, _errors: &mut Errors) -> Result<Self, ()> {
		Ok(input.tokens.drain(..).collect())
	}
}

impl SimpleSpanned for Ident {
	fn span(&self) -> Span {
		self.span()
	}

	fn set_span(&mut self, span: Span) {
		self.set_span(span)
	}
}

impl SimpleSpanned for Punct {
	fn span(&self) -> Span {
		self.span()
	}

	fn set_span(&mut self, span: Span) {
		self.set_span(span)
	}
}

impl SimpleSpanned for Literal {
	fn span(&self) -> Span {
		self.span()
	}

	fn set_span(&mut self, span: Span) {
		self.set_span(span)
	}
}

impl IntoTokens for Group {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([TokenTree::Group(self)])
	}
}

impl IntoTokens for Ident {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([TokenTree::Ident(self)])
	}
}

impl IntoTokens for Punct {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([TokenTree::Punct(self)])
	}
}

impl IntoTokens for Literal {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([TokenTree::Literal(self)])
	}
}

impl IntoTokens for TokenTree {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend([self])
	}
}

impl IntoTokens for TokenStream {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		tokens.extend(self);
	}
}
