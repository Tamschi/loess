//! **Inaccurate** but lightweight [Metavariable](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.meta) match parsers.
//!
//! Generally, these will only work properly iff the input respects the follow set limitations explained in [macro.decl.follow-set](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.follow-set).

use loess::{
	Errors, Input, PopParsedFrom, grammar,
	scaffold::{CurlyBraces, MetaGroup, Parentheses},
	words,
};
use loess_rust_lex::lex::token::punct::Lt;
use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree, extra::DelimSpan};

words! {
	#[derive(Clone)] pub(self) pub as Pub: PeekFrom, PopFrom, IntoTokens;
}

grammar! {
	// pub struct Block: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Expr: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Expr2021: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Ident: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Item: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Lifetime: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Literal: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Meta: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Pat: PeekFrom, PopFrom, IntoTokens { }

	// pub struct PatParam: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Path: PeekFrom, PopFrom, IntoTokens { }

	// pub struct Stmt: PeekFrom, PopFrom, IntoTokens { }

	/// This groups operators!
	// pub struct TT: PeekFrom, PopFrom, IntoTokens { }

	pub struct Ty: IntoTokens {
		inner: Group,
	}

	//TODO: Check if not examining the parentheses content matches rustc!

	pub struct Vis:  PopFrom, IntoTokens {
		inner: Option<(Pub, Option<Parentheses>)>,
	}
}

impl PopParsedFrom for Ty {
	type Parsed = Self;

	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, Option<Self>> {
		todo!("Ty::pop_parsed_from")
	}
}
