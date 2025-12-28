//! [paths.simple](https://doc.rust-lang.org/reference/paths.html#r-paths.simple): Simple Paths

use loess::{PeekFrom, grammar, scaffold::Greedy};

use crate::{
	ident::Identifier,
	lex::{
		keywords::{Crate, SelfUppercase, Super},
		token::punct::{Dollar, PathSep},
	},
};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [SimplePath](https://doc.rust-lang.org/reference/paths.html#grammar-SimplePath)
	pub struct SimplePath: PopFrom, IntoTokens {
		path_sep: Option<PathSep>,
		simple_path_segment: SimplePathSegment,
		path_sep_simple_path_segments: Greedy<Vec<(PathSep, SimplePathSegment)>>,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [SimplePathSegment](https://doc.rust-lang.org/reference/paths.html#grammar-SimplePathSegment)
	pub enum SimplePathSegment: PeekFrom, PopFrom, IntoTokens {
		Identifier(Identifier),
		Super(Super),
		SelfUppercase(SelfUppercase),
		Crate(Crate),
		DollarCrate(Dollar, Crate),
	} else "Expected SimplePathSegment.";
}

impl PeekFrom for SimplePath {
	fn peek_from(input: &loess::Input) -> bool {
		todo!()
	}
}
