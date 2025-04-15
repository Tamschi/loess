use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
	Expr,
	parse::{Parse, ParseStream, Parser},
};

use crate::{Error, ErrorPriority, Errors, Input, IntoTokens, PopFrom, grammar};

grammar! {
	pub struct Expression: IntoTokens {
		syn: Expr,
	}

	pub struct ExpressionExceptStructExpression: IntoTokens {
		syn: Expr,
	}
}

impl PopFrom for Expression {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(input: ParseStream) -> syn::Result<(Expression, TokenStream)> {
			Ok((
				Expression {
					syn: Expr::parse(input)?,
				},
				TokenStream::parse(input)?,
			))
		}

		let error_span = input
			.front_span()
			.join(input.end)
			.unwrap_or(input.front_span());

		let tokens = input.tokens.drain(..).collect::<TokenStream>().into();
		let (this, rest) = parse.parse2(tokens).map_err(|error| {
			errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Expected Expression except StructExpression.",
				[error_span],
			));
		})?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}

impl PopFrom for ExpressionExceptStructExpression {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(
			input: ParseStream,
		) -> syn::Result<(ExpressionExceptStructExpression, TokenStream)> {
			Ok((
				ExpressionExceptStructExpression {
					syn: Expr::parse_without_eager_brace(input)?,
				},
				TokenStream::parse(input)?,
			))
		}

		let error_span = input
			.front_span()
			.join(input.end)
			.unwrap_or(input.front_span());

		let tokens = input.tokens.drain(..).collect::<TokenStream>().into();
		let (this, rest) = parse.parse2(tokens).map_err(|error| {
			errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Expected Expression except StructExpression.",
				[error_span],
			));
		})?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}

impl IntoTokens for Expr {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.into_token_stream().into_tokens(root, tokens);
	}
}
