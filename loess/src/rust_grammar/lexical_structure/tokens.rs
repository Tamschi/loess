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

ident_token!(Async = "async" => "Expected `async`.");
ident_token!(Const = "const" => "Expected `const`.");
ident_token!(For = "for" => "Expected `for`.");
ident_token!(In = "in" => "Expected `in`.");
