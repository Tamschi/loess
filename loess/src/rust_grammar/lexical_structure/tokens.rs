use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::{Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, WithSpanExt};

/// See <https://doc.rust-lang.org/stable/reference/tokens.html?highlight=arrow#punctuation> as of 2025-04-13.
pub struct RArrow(pub Punct, pub Punct);

impl Default for RArrow {
	fn default() -> Self {
		Self(
			Punct::new('-', Spacing::Joint).with_span(Span::mixed_site()),
			Punct::new('>', Spacing::Alone).with_span(Span::mixed_site()),
		)
	}
}

impl PopFrom for RArrow {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(minus), TokenTree::Punct(gt)]
					if minus.as_char() == '-'
						&& minus.spacing() == Spacing::Joint
						&& gt.as_char() == '>' =>
				{
					Ok(Self(minus, gt))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `->`.", spans))
			})
	}
}

impl IntoTokens for RArrow {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens);
		self.1.into_tokens(root, tokens);
	}
}

pub struct DotDot(pub Punct, pub Punct);

impl Default for DotDot {
	fn default() -> Self {
		Self(
			Punct::new('.', Spacing::Joint).with_span(Span::mixed_site()),
			Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()),
		)
	}
}

impl PeekFrom for DotDot {
	fn peek_from(input: &Input) -> bool {
		matches!(
			(input.front(), input.tokens.get(1)),
			(Some(TokenTree::Punct(dot0)), Some(TokenTree::Punct(dot1))) if dot0.as_char() == '.' && dot0.spacing() == Spacing::Joint && dot1.as_char() == '.',
		)
	}
}

impl PopFrom for DotDot {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(minus), TokenTree::Punct(gt)]
					if minus.as_char() == '.'
						&& minus.spacing() == Spacing::Joint
						&& gt.as_char() == '.' =>
				{
					Ok(Self(minus, gt))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `..`.", spans))
			})
	}
}

impl IntoTokens for DotDot {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens);
		self.1.into_tokens(root, tokens);
	}
}

pub struct Semi(pub Punct);

impl Default for Semi {
	fn default() -> Self {
		Self(Punct::new(';', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

impl PeekFrom for Semi {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(semi)) if semi.as_char() == ';',
		)
	}
}

impl PopFrom for Semi {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(semi)] if semi.as_char() == ';' => Ok(Self(semi)),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `;`.", spans))
			})
	}
}

impl IntoTokens for Semi {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

pub struct Or(pub Punct);

impl Default for Or {
	fn default() -> Self {
		Self(Punct::new('|', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

impl PeekFrom for Or {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(or)) if or.as_char() == '|' && or.spacing() == Spacing::Alone,
		)
	}
}

impl PopFrom for Or {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(or)] if or.as_char() == '|' && or.spacing() == Spacing::Alone => {
					Ok(Self(or))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `|`.", spans))
			})
	}
}

impl IntoTokens for Or {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

pub struct Dot(pub Punct);

impl Default for Dot {
	fn default() -> Self {
		Self(Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

impl PeekFrom for Dot {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(dot)) if dot.as_char() == '.' && dot.spacing() == Spacing::Alone,
		)
	}
}

impl PopFrom for Dot {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(dot)]
					if dot.as_char() == '.' && dot.spacing() == Spacing::Alone =>
				{
					Ok(Self(dot))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `.`.", spans))
			})
	}
}

impl IntoTokens for Dot {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

pub struct Colon(pub Punct);

impl Default for Colon {
	fn default() -> Self {
		Self(Punct::new(':', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

impl PeekFrom for Colon {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(colon)) if colon.as_char() == ':' && colon.spacing() == Spacing::Alone,
		)
	}
}

impl PopFrom for Colon {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(colon)]
					if colon.as_char() == ':' && colon.spacing() == Spacing::Alone =>
				{
					Ok(Self(colon))
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `:`.", spans))
			})
	}
}

impl IntoTokens for Colon {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

macro_rules! ident_token {
	($name:ident = $str:literal => $error:literal) => {
		pub struct $name(pub Ident);

		impl PeekFrom for $name {
			fn peek_from(input: &Input) -> bool {
				matches!(input.front(), Some(TokenTree::Ident(ident)) if ident == $str)
			}
		}

		impl PopFrom for $name {
			fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
			where
				Self: Sized,
			{
				input
					.pop_or_replace(|t| match t {
						[TokenTree::Ident(ident)] if ident == $str => Ok(Self(ident)),
						other => Err(other),
					})
					.map_err(|spans| {
						errors.push(Error::new(ErrorPriority::TOKEN, $error, spans))
					})
			}
		}

		impl IntoTokens for $name {
			fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
				self.0.into_tokens(root, tokens)
			}
		}
	};
}

ident_token!(As = "as" => "Expected `as`.");
ident_token!(Async = "async" => "Expected `async`.");
ident_token!(Box = "box" => "Expected `box`.");
ident_token!(Const = "const" => "Expected `const`.");
ident_token!(For = "for" => "Expected `for`.");
ident_token!(In = "in" => "Expected `in`.");
ident_token!(SelfLowercase = "self" => "Expected `self`.");
ident_token!(Struct = "struct" => "Expected `struct`.");
