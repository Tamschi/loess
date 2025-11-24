use loess::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, PopParsedFrom,
	SimpleSpanned,
};
use proc_macro2::{Ident, Span, TokenStream, TokenTree};

/// [IDENTIFIER](https://doc.rust-lang.org/stable/reference/identifiers.html#r-ident.syntax)
#[derive(Clone)]
pub struct Identifier(pub Ident);

impl PeekFrom for Identifier {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Ident(ident))
				if !(["r#crate", "r#self", "r#super", "r#Self"]
					.into_iter()
					.any(|s| ident == s)
					|| is_strict_keyword(&ident)
					|| is_reserved_keyword(&ident)),
		)
	}
}

/// See <https://doc.rust-lang.org/stable/reference/identifiers.html?highlight=IDENTIFIER#identifiers> as of 2025-04-13.
impl PopParsedFrom for Identifier {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		let ident = Ident::peek_pop_from(input, errors)?;

		match ident {
			Some(ident)
				if !(["r#crate", "r#self", "r#super", "r#Self"]
					.into_iter()
					.any(|s| ident == s)
					|| is_strict_keyword(&ident)
					|| is_reserved_keyword(&ident)) =>
			{
				Ok(Self(ident))
			}
			ident => Err(if let Some(ident) = ident {
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
			}),
		}
	}
}

impl IntoTokens for Identifier {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

impl SimpleSpanned for Identifier {
	fn span(&self) -> Span {
		self.0.span()
	}

	fn set_span(&mut self, span: Span) {
		self.0.set_span(span)
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
