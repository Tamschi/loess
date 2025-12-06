//! [vis](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html#r-vis): Visibility and Privacy

use loess::{grammar, scaffold::Parentheses};

use crate::{
	lex::keywords::{Crate, In, Pub, SelfLowercase, Super},
	paths::simple::SimplePath,
};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html#grammar-Visibility)
	pub struct Visibility: PeekFrom, PopFrom, IntoTokens {
		pub r#pub: Pub,
		/// Continue inside with [`VisibilityContent`].
		pub parentheses: Option<Parentheses>,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// Inside [`Visibility::parentheses`].
	pub enum VisibilityContent: PeekFrom, PopFrom, IntoTokens {
		Crate(Crate),
		SelfLowercase(SelfLowercase),
		Super(Super),
		InSimplePath(In, SimplePath),

	} else "Expected VisibilityContent";
}
