use loess::{
	grammar,
	scaffold::{CurlyBraces, Greedy, RepeatCount},
};

use crate::{attributes::InnerAttribute, expr::ExpressionWithoutBlock};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [BlockExpression](https://doc.rust-lang.org/stable/reference/expressions/block-expr.html?highlight=BlockExpression#r-expr.block.syntax)
	pub struct BlockExpression: PeekFrom, PopFrom, IntoTokens {
		/// Continue inside with [`BlockExpressionContent`] or [`BlockExpressionContentFlattened`].
		pub braces: CurlyBraces,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// Content of [`BlockExpression::braces`]
	pub struct BlockExpressionContent: PeekFrom, PopFrom, IntoTokens {
		inner_attributes: Greedy<Vec<InnerAttribute>>,
		statements: Option<Statements>,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [BlockExpression](https://doc.rust-lang.org/stable/reference/expressions/block-expr.html?highlight=Statements#r-expr.block.syntax)
	pub enum Statements: PeekFrom, PopFrom, IntoTokens {
		Statements(RepeatCount<Vec<Statement>, 1, { usize::MAX }>),
		StatementsExpressionWithoutBlock(RepeatCount<Vec<Statement>, 1, { usize::MAX }>, ExpressionWithoutBlock),
		ExpressionWithoutBlock(ExpressionWithoutBlock),
	} else "Expected statements with optional trailing expression without block.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [BlockExpression](https://doc.rust-lang.org/stable/reference/expressions/block-expr.html?highlight=Statements#r-expr.block.syntax)
	pub struct BlockExpressionContentFlattened: PeekFrom, PopFrom, IntoTokens {
		inner_attributes: Greedy<Vec<InnerAttribute>>,
		statements: Greedy<Vec<Statement>>,
		expression: Option<ExpressionWithoutBlock>,
	}
}
