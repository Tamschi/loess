//! [items.extern-crate](https://doc.rust-lang.org/reference/items/extern-crates.html#r-items.extern-crate): Extern crate declarations

use loess::grammar;

use crate::{
	ident::Identifier,
	lex::{
		keywords::{As, Crate, Extern, SelfLowercase},
		token::punct::{Semi, Underscore},
	},
};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [ExternCrate](https://doc.rust-lang.org/reference/items/extern-crates.html#grammar-ExternCrate)
	pub struct ExternCrate: PeekFrom, PopFrom, IntoTokens {
		r#extern: Extern,
		crate_: Crate,
		crate_ref: CrateRef,
		as_clause: Option<AsClause>,
		semi: Semi,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [CrateRef](https://doc.rust-lang.org/reference/items/extern-crates.html#grammar-CrateRef)
	pub enum CrateRef: PeekFrom, PopFrom, IntoTokens {
		Identifier(Identifier),
		SelfLowercase(SelfLowercase),
	} else "Expected CrateRef.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [AsClause](https://doc.rust-lang.org/reference/items/extern-crates.html#grammar-AsClause)
	pub struct AsClause: PeekFrom, PopFrom, IntoTokens {
		r#as: As,
		variant: AsClauseVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`AsClause::variant`]
	pub enum AsClauseVariant: PeekFrom, PopFrom, IntoTokens {
		Identifier(Identifier),
		Underscore(Underscore),
	} else "Expected AsClauseVariant.";
}
