//! [lex.token](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token>)

use loess::grammar;

use self::literal::{RawStringLiteral, StringLiteral};

pub mod delim;
pub mod life;
pub mod literal;
pub mod punct;

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
		// Punctuation(Punctuation),
		// ReservedToken(ReservedToken),
	} else "Expected Token.";
}
