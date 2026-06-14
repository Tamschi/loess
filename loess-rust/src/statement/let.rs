//! [statement.let](https://doc.rust-lang.org/reference/statements.html#r-statement.let): `let`` statements

use loess::{PeekFrom, grammar, scaffold::Greedy};

use crate::{
	attributes::OuterAttribute,
	expr::{Expression, block::BlockExpression},
	lex::{
		keywords::{Else, Let},
		token::punct::{Eq, Semi},
	},
};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [LetStatement](https://doc.rust-lang.org/reference/statements.html#grammar-LetStatement)
	pub struct LetStatement: PopFrom, IntoTokens {
		outer_attributes: Greedy<Vec<OuterAttribute>>,
		r#let: Let,
		// pattern_no_top_alt: PatternNoTopAlt,
		// colon_type: Option<(Colon, Type)>,
		variant: LetStatementVariant,
		semi: Semi,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`LetStatement::variant`]
	pub enum LetStatementVariant: PeekFrom, PopFrom, IntoTokens {
		EqExpression(Eq, Expression),
		///TODO: Special parsing rule.
		EqExpressionElseBlockExpression(Eq, Expression, Else, BlockExpression),
	} else "Expected LetStatementVariant";
}

impl PeekFrom for LetStatement {
	fn peek_from(input: &loess::Input) -> bool {
		todo!("LetStatement::peek_from")
	}
}
