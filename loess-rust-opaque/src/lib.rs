//! Additional grammar tokens representing the stable Rust programming language,
//! closely following [The Rust Reference](https://doc.rust-lang.org/stable/reference/).
//!
//! Corrections in that regard are not automatically considered breaking changes,
//! unless they became necessary due to a change in Rust **and** reduce what is considered valid.
//!
//! Breaking changes to the API are considered breaking as normal.
//!
//! *Note that unstable grammar **is** accidentally accepted in some cases.*  
//! ***Ceasing to accept unstable grammar is not by itself considered a breaking change for Loess.***

use loess::{Error, ErrorPriority, Errors, Input, IntoTokens, grammar_helpers::PopParsedFrom};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
	Expr, Pat, Path, Stmt,
	parse::{Parse, ParseStream, Parser},
};

fn error_reporter(errors: &mut Errors) -> impl '_ + FnOnce(syn::Error) {
	move |error| {
		errors.push(Error::new(
			ErrorPriority::GRAMMAR,
			error.to_string(),
			[error.span()],
		))
	}
}

macro_rules! wrappers {
	($(
		$(#[$($attr:tt)*])*
		$name:ident($wrapped:ty)$(: $(
			// $(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*$(,)?)?;
	)*) => {$(
		$(#[$($attr)*])*
		#[derive(Clone)]
		pub struct $name($wrapped);

		$($(
			// $(
			// 	$(@ $PeekFrom)?
			// 	impl PeekFrom for $name {
			// 		fn peek_from(input: &Input) -> bool {
			// 			fn peek(input: ParseStream) -> syn::Result<bool> {
			// 				let result = input.peek(<$wrapped>::default());
			// 				let _ = TokenStream::parse(input).expect("infallible"); // Discard cloned input.
			// 				Ok(result)
			// 			}

			// 			let input = input.tokens.iter().cloned().collect::<TokenStream>();
			// 			peek.parse2(input).expect("infallible")
			// 		}
			// 	}
			// )?

			$(
				$(@ $PopFrom)?
				impl PopParsedFrom for $name {
					type Parsed = Self;
					fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
						fn parse(input: ParseStream) -> syn::Result<($wrapped, TokenStream)> {
							Ok((<$wrapped>::parse(input)?, TokenStream::parse(input)?))
						}

						let tokens: TokenStream = input.tokens.drain(..).collect();
						let (parsed, rest) = parse.parse2(tokens).map_err(error_reporter(errors))?;
						input.prepend(rest.into_iter().collect::<Vec<_>>());
						Ok(Self(parsed))
					}
				}
			)?

			$(
				$(@ $IntoTokens)?
				impl IntoTokens for $name {
					fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
						self.0.into_token_stream().into_tokens(root, tokens);
					}
				}
			)?
		)*)?
	)*};
}

wrappers! {
	/// [*Expression*](https://doc.rust-lang.org/stable/reference/expressions.html#r-expr.syntax)
	Expression(Expr): PopFrom, IntoTokens;

	/// [*Expression*](https://doc.rust-lang.org/stable/reference/expressions.html#r-expr.syntax)
	/// <sub>except [*StructExpression*](https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#r-expr.struct.syntax)</sub>
	ExpressionExceptStructExpression(Expr): IntoTokens;

	/// [*Pattern*](https://doc.rust-lang.org/stable/reference/patterns.html#r-patterns.syntax)
	Pattern(Pat): IntoTokens;

	/// [*SimplePath*](https://doc.rust-lang.org/stable/reference/paths.html#r-paths.simple.syntax)
	SimplePath(Path): IntoTokens;

	/// [*Statement*](https://doc.rust-lang.org/stable/reference/statements.html#r-statement.syntax)
	Statement(Stmt): PopFrom, IntoTokens;
}

impl PopParsedFrom for ExpressionExceptStructExpression {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(
			input: ParseStream,
		) -> syn::Result<(ExpressionExceptStructExpression, TokenStream)> {
			Ok((
				ExpressionExceptStructExpression(Expr::parse_without_eager_brace(input)?),
				TokenStream::parse(input)?,
			))
		}

		let tokens = input.tokens.drain(..).collect::<TokenStream>().into();
		let (this, rest) = parse.parse2(tokens).map_err(error_reporter(errors))?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}

impl PopParsedFrom for Pattern {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(input: ParseStream) -> syn::Result<(Pattern, TokenStream)> {
			Ok((
				Pattern(Pat::parse_multi_with_leading_vert(input)?),
				TokenStream::parse(input)?,
			))
		}

		let tokens = input.tokens.drain(..).collect::<TokenStream>().into();
		let (this, rest) = parse.parse2(tokens).map_err(error_reporter(errors))?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}

impl PopParsedFrom for SimplePath {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		fn parse(input: ParseStream) -> syn::Result<(SimplePath, TokenStream)> {
			Ok((
				SimplePath(Path::parse_mod_style(input)?),
				TokenStream::parse(input)?,
			))
		}

		let tokens = input.tokens.drain(..).collect::<TokenStream>().into();
		let (this, rest) = parse.parse2(tokens).map_err(error_reporter(errors))?;

		input.prepend(rest.into_iter().collect::<Vec<_>>());
		Ok(this)
	}
}
