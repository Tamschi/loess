//! [attributes](https://doc.rust-lang.org/stable/reference/attributes.html#r-attributes): Attributes

use loess::{grammar, scaffold::SquareBrackets};

use crate::lex::token::punct::Pound;

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// `#` `[…]` TODO
	pub struct OuterAttribute: PeekFrom, PopFrom, IntoTokens {
		pound: Pound,
		/// Continue inside with [`Attr`].
		brackets: SquareBrackets,
	}
}
