//! [ident](https://doc.rust-lang.org/stable/reference/identifiers.html#r-ident):
//! Identifiers (not [keywords](`crate::lex::keywords`) or [lifetimes](`crate::lex::token::life`)).

use loess::{Error, ErrorPriority, Errors, Input, PeekFrom, PopFrom as _, PopParsedFrom};
use proc_macro2::{Ident, TokenTree};

use crate::lex::keywords::{is_reserved_keyword, is_strict_keyword};

pub use crate::lex::keywords::words_impl::Identifier;

impl PeekFrom for Identifier {
	fn peek_from(input: &Input) -> bool {
		matches!(input.front(), Some(TokenTree::Ident(ident)) if is_identifier(ident))
	}
}

/// See <https://doc.rust-lang.org/reference/identifiers.html#grammar-IDENTIFIER> as of 2025-04-13.
impl PopParsedFrom for Identifier {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, Option<Self>> {
		let ident = Ident::peek_pop_from(input, errors).map_err(|_| None)?;

		match ident {
			Some(ident) if is_identifier(&ident) => Ok(Self(ident)),
			ident => {
				if let Some(ident) = ident {
					errors.push(Error::new(
						ErrorPriority::GRAMMAR,
						if ident.to_string().starts_with("r#") {
							format!(
								"Expected Identifier. (`{}` cannot be a raw identifier.)",
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
				Err(None)
			}
		}
	}
}

pub fn is_identifier(ident: &Ident) -> bool {
	!(["r#crate", "r#self", "r#super", "r#Self"]
		.into_iter()
		.any(|s| ident == s)
		|| is_strict_keyword(&ident)
		|| is_reserved_keyword(&ident)
		|| ident.to_string().starts_with('\''))
}
