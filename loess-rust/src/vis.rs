//! [vis](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html#r-vis): Visibility and Privacy

use loess::{grammar, scaffold::Parentheses};

use crate::lex::keywords::Pub;

grammar! {
	/// [*Visibility*](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html?highlight=Visibility#r-vis.syntax)
	#[derive(Clone)]
	#[non_exhaustive]
	pub struct Visibility: PeekFrom, PopFrom, IntoTokens {
		pub r#pub: Pub,
		/// Continue inside with [`TODO`].
		pub parentheses: Option<Parentheses>,
	}
}
