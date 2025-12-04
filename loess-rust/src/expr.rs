//! [expr](https://doc.rust-lang.org/stable/reference/expressions.html#r-expr): Expressions

use loess::{grammar, scaffold::Greedy};

use crate::{
	attributes::OuterAttribute,
	expr::{block::BlockExpression, literal::LiteralExpression},
};

pub mod block;
pub mod literal;

grammar! {
	///
	/// See <https://doc.rust-lang.org/stable/reference/expressions.html?highlight=Expression#r-expr.syntax>.
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum Expression: doc, PeekFrom, PopFrom, IntoTokens {
		ExpressionWithoutBlock(ExpressionWithoutBlock),
		ExpressionWithBlock(ExpressionWithBlock),
	} else "Expected expression.";

	/// [`OuterAttribute`]<sup>*</sup> [`ExpressionWithoutBlockVariant`]
	///
	/// See <https://doc.rust-lang.org/stable/reference/expressions.html?highlight=ExpressionWithoutBlock#r-expr.syntax>.
	#[derive(Clone)]
	#[non_exhaustive]
	pub struct ExpressionWithoutBlock: PeekFrom, PopFrom, IntoTokens {
		pub outer_attributes: Greedy<Vec<OuterAttribute>>,
		pub variant: ExpressionWithoutBlockVariant,
	}

	///
	/// See <https://doc.rust-lang.org/stable/reference/expressions.html?highlight=ExpressionWithoutBlock#r-expr.syntax>.
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum ExpressionWithoutBlockVariant: doc, PeekFrom, PopFrom, IntoTokens {
		LiteralExpression(LiteralExpression),
	} else "Expected expression without block variant.";

	/// [`OuterAttribute`]<sup>*</sup> [`ExpressionWithBlockVariant`]
	///
	/// See <https://doc.rust-lang.org/stable/reference/expressions.html?highlight=ExpressionWithBlock#r-expr.syntax>.
	#[derive(Clone)]
	#[non_exhaustive]
	pub struct ExpressionWithBlock: PeekFrom, PopFrom, IntoTokens {
		pub outer_attributes: Greedy<Vec<OuterAttribute>>,
		pub variant: ExpressionWithBlockVariant,
	}

	///
	/// See <https://doc.rust-lang.org/stable/reference/expressions.html?highlight=ExpressionWithBlock#r-expr.syntax>.
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum ExpressionWithBlockVariant: doc, PeekFrom, PopFrom, IntoTokens {
		BlockExpression(BlockExpression),
	} else "Expected expression with block variant.";

	/// Flattened [`Expression`].
	#[derive(Clone)]
	#[non_exhaustive]
	pub struct AnyExpression: PeekFrom, PopFrom, IntoTokens {
		pub outer_attributes: Greedy<Vec<OuterAttribute>>,
		pub variant: AnyExpressionVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	pub enum AnyExpressionVariant: doc, PeekFrom, PopFrom, IntoTokens {
		LiteralExpression(LiteralExpression),
	} else "Expected expression without block variant.";
}
