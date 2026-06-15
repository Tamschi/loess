use std::ops::ControlFlow;

use crate::{ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopParsedFrom, SimpleSpanned};

use super::Error;
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

impl PopParsedFrom for Group {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Group(group)] => Ok(group),
				t => Err(t),
			})
			.map_continue(Some)
			.map_break(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Group.", spans));
				None
			})
	}
}

impl PopParsedFrom for Ident {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Ident(ident)] => Ok(ident),
				t => Err(t),
			})
			.map_continue(Some)
			.map_break(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Ident.", spans));
				None
			})
	}
}

impl PopParsedFrom for Punct {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Punct(punct)] => Ok(punct),
				t => Err(t),
			})
			.map_continue(Some)
			.map_break(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Punct.", spans));
				None
			})
	}
}

impl PopParsedFrom for Literal {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t, _| match t {
				[TokenTree::Literal(literal)] => Ok(literal),
				t => Err(t),
			})
			.map_continue(Some)
			.map_break(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Literal.", spans));
				None
			})
	}
}

impl PopParsedFrom for TokenTree {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|[t], _| Ok(t))
			.map_continue(Some)
			.map_break(|spans| {
				errors.push(Error::new(
					ErrorPriority::TOKEN,
					"Expected TokenTree.",
					spans,
				));
				None
			})
	}
}

/// Exhaustive, infallible.
impl PopParsedFrom for TokenStream {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		_errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>> {
		ControlFlow::Continue(Some(input.tokens.drain(..).collect()))
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
