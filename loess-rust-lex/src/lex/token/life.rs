//! [lex.token.life](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token.life): Lifetimes and loop labels

use std::ops::ControlFlow::{self, Break, Continue};

use loess::{Error, ErrorPriority, Errors, Input, PeekFrom, PopFrom, PopParsedFrom, lifetimes};
use proc_macro2::{Ident, TokenTree};

lifetimes! {
	/// [LIFETIME_TOKEN](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token.life.syntax)
	#[derive(Clone)] pub _ as pub Lifetime: IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
}

impl PeekFrom for Lifetime {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Ident(ident)) if is_lifetime(ident))
	}
}

/// See <https://doc.rust-lang.org/stable/reference/tokens.html?highlight=LIFETIME_TOKEN#r-lex.token.life.syntax> as of 2025-12-04.
impl PopParsedFrom for Lifetime {
	type Parsed = Self;
	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut Errors,
	) -> ControlFlow<Option<Self>, Option<Self>> {
		let ident = Ident::peek_pop_from(input, errors).map_break(|_| None)?;

		match ident {
			Some(ident) if is_lifetime(&ident) => Continue(Some(Self(ident))),
			ident => {
				if let Some(ident) = ident {
					errors.push(Error::new(
						ErrorPriority::GRAMMAR,
						if ident.to_string().starts_with("r#") {
							format!(
								"Expected Lifetime. (`{}` cannot be a raw identifier.)",
								&ident.to_string()[2..]
							)
						} else {
							format!("Expected Identifier. (`{ident}` is a keyword.)")
						},
						[ident.span()],
					));

					input.push_front(TokenTree::Ident(ident));
				} else {
					errors.push(Error::new(
						ErrorPriority::GRAMMAR,
						"Expected Identifier.",
						[input.front_span()],
					));
				}
				Break(None)
			}
		}
	}
}

pub fn is_lifetime(ident: &Ident) -> bool {
	!["'r#crate", "'r#self", "'r#super", "'r#Self"]
		.into_iter()
		.any(|s| ident == s)
		&& ident.to_string().starts_with('\'')
}
