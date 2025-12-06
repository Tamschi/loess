//! [statement](https://doc.rust-lang.org/reference/statements.html#r-statement): Statements

use loess::{grammar, scaffold::Greedy};

use crate::{attributes::OuterAttribute, items::Item, lex::token::punct::Semi, r#macro::invocation::MacroInvocationSemi, statement::r#let::LetStatement};

pub mod r#let;

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Statement](https://doc.rust-lang.org/reference/statements.html?highlight=statement#r-statement.syntax)
	pub enum Statement: PeekFrom, PopFrom, IntoTokens {
		Semi(Semi),
		Item(Item),
		LetStatement(LetStatement),
		OuterAttributesMacroInvocationSemi(Greedy<Vec<OuterAttribute>>, MacroInvocationSemi),
	} else "Expected statement.";
}
