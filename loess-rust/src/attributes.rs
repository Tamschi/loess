//! [attributes](https://doc.rust-lang.org/stable/reference/attributes.html#r-attributes): Attributes

use loess::{grammar, scaffold::SquareBrackets};

use crate::lex::token::punct::{Not, Pound};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [InnerAttribute](https://doc.rust-lang.org/reference/attributes.html#grammar-InnerAttribute)
	pub struct InnerAttribute: PeekFrom, PopFrom, IntoTokens {
		pound: Pound,
		not: Not,
		/// Continue inside with [`Attr`].
		brackets: SquareBrackets,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [OuterAttribute](https://doc.rust-lang.org/reference/attributes.html#grammar-OuterAttribute)
	pub struct OuterAttribute: PeekFrom, PopFrom, IntoTokens {
		pound: Pound,
		/// Continue inside with [`Attr`].
		brackets: SquareBrackets,
	}
}
