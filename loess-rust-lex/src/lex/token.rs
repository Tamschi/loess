//! [lex.token](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token>)

use loess::{grammar, scopes};

use crate::lex::token::punct::Punctuation;

use self::literal::{RawStringLiteral, StringLiteral};

pub mod delim;
pub mod life;
pub mod literal;
pub mod punct;

scopes! {
	/// Used on [`Token`] to exclude [`CurlyBraces`](`delim::CurlyBraces`),  [`SquareBrackets`](`delim::SquareBrackets`) and [`Parentheses`](`delim::Parentheses`):
	///
	/// ```
	/// use loess_rust::lex::token::{ExceptDelimiters, Token};
	///
	/// type TokenExceptDelimiters = ExceptDelimiters<Token>;
	/// ```
	pub ExceptDelimiters - pub(self) IncludingDelimiters: bool;
}

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Token](https://doc.rust-lang.org/reference/tokens.html#grammar-Token)
	pub enum Token: doc, PeekFrom, PopFrom, IntoTokens {
		// IdentifierOrKeyword(IdentifierOrKeyword),
		// RawIdentifier(RawIdentifier),
		// CharLiteral(CharLiteral),
		StringLiteral(StringLiteral),
		RawStringLiteral(RawStringLiteral),
		// ByteLiteral(ByteLiteral),
		// ByteStringLiteral(ByteStringLiteral),
		// RawByteStringLiteral(RawByteStringLiteral),
		// CStringLiteral(CStringLiteral),
		// RawCStringLiteral(RawCStringLiteral),
		// IntegerLiteral(IntegerLiteral),
		// FloatLiteral(FloatLiteral),
		// LifetimeToken(LifetimeToken),
		Punctuation(Punctuation),
		// ReservedToken(ReservedToken),
	} else "Expected Token.";
}
