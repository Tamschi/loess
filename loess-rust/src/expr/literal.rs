//! [expr.literal](https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html#r-expr.literal): Literal expressions

use loess::grammar;

use crate::lex::{
	keywords::{False, True},
	token::literal::{AnyBoolLiteral, AnyStringLiteral, RawStringLiteral, StringLiteral},
};

grammar! {
	///
	/// See <https://doc.rust-lang.org/stable/reference/expressions/literal-expr.html?highlight=LiteralExpression#r-expr.literal.syntax>.
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum LiteralExpression: doc, PeekFrom, PopFrom, IntoTokens {
		// Char
		StringLiteral(StringLiteral),
		RawStringLiteral(RawStringLiteral),
		// Byte
		// ByteString
		// RawByteString
		// CString
		// RawCString
		// Integer
		// Float
		True(True),
		False(False),
	} else "Expected literal expression.";

	///
	/// Simplified [`LiteralExpression`].
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum LiteralExpressionByType: doc, PeekFrom, PopFrom, IntoTokens {
		// AnyChar
		AnyString(AnyStringLiteral),
		// AnyByte
		// AnyByteString
		// RawByteString
		// AnyCString
		// RawCString
		// AnyInteger
		// AnyFloat
		AnyBool(AnyBoolLiteral),
	} else "Expected literal expression.";
}

impl LiteralExpression {
	pub fn group_by_type(self) -> LiteralExpressionByType {
		use LiteralExpression::*;
		use LiteralExpressionByType::*;
		match self {
			StringLiteral(s) => AnyString(AnyStringLiteral::Plain(s)),
			RawStringLiteral(r) => AnyString(AnyStringLiteral::Raw(r)),
			True(t) => AnyBool(AnyBoolLiteral::True(t)),
			False(f) => AnyBool(AnyBoolLiteral::False(f)),
		}
	}
}

impl LiteralExpressionByType {
	pub fn flatten(self) -> LiteralExpression {
		use LiteralExpression::*;
		use LiteralExpressionByType::*;
		match self {
			AnyString(s) => match s {
				AnyStringLiteral::Plain(s) => StringLiteral(s),
				AnyStringLiteral::Raw(r) => RawStringLiteral(r),
			},
			AnyBool(b) => match b {
				AnyBoolLiteral::True(t) => True(t),
				AnyBoolLiteral::False(f) => False(f),
			},
		}
	}
}
