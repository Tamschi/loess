//! [expr](https://doc.rust-lang.org/stable/reference/expressions.html#r-expr): Expressions

use loess::{grammar, scaffold::Greedy};

use crate::{
	attributes::OuterAttribute,
	expr::{block::BlockExpression, literal::LiteralExpression},
};

pub mod block;
pub mod literal;

grammar! {
	/// [Expression](https://doc.rust-lang.org/reference/expressions.html#grammar-Expression)
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum Expression: PeekFrom, PopFrom, IntoTokens {
		ExpressionWithoutBlock(ExpressionWithoutBlock),
		ExpressionWithBlock(ExpressionWithBlock),
	} else "Expected expression.";

	/// [ExpressionWithoutBlock](https://doc.rust-lang.org/reference/expressions.html#grammar-ExpressionWithoutBlock)
	#[derive(Clone)]
	#[non_exhaustive]
	pub struct ExpressionWithoutBlock: PeekFrom, PopFrom, IntoTokens {
		pub outer_attributes: Greedy<Vec<OuterAttribute>>,
		pub variant: ExpressionWithoutBlockVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`ExpressionWithoutBlock::variant`]
	pub enum ExpressionWithoutBlockVariant: PeekFrom, PopFrom, IntoTokens {
		LiteralExpression(LiteralExpression),
	} else "Expected expression without block variant.";

	/// [ExpressionWithBlock](https://doc.rust-lang.org/reference/expressions.html#grammar-ExpressionWithBlock)
	#[derive(Clone)]
	#[non_exhaustive]
	pub struct ExpressionWithBlock: PeekFrom, PopFrom, IntoTokens {
		pub outer_attributes: Greedy<Vec<OuterAttribute>>,
		pub variant: ExpressionWithBlockVariant,
	}

	/// [`ExpressionWithBlock::variant`]
	#[derive(Clone)]
	#[non_exhaustive]
	pub enum ExpressionWithBlockVariant: PeekFrom, PopFrom, IntoTokens {
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
		//
		BlockExpression(BlockExpression),
	} else "Expected expression without block variant.";
}

impl From<Expression> for AnyExpression {
	fn from(value: Expression) -> Self {
		match value {
			Expression::ExpressionWithoutBlock(ExpressionWithoutBlock {
				outer_attributes,
				variant,
			}) => Self {
				outer_attributes,
				variant: match variant {
					ExpressionWithoutBlockVariant::LiteralExpression(l) => {
						AnyExpressionVariant::LiteralExpression(l)
					}
				},
			},
			Expression::ExpressionWithBlock(ExpressionWithBlock {
				outer_attributes,
				variant,
			}) => Self {
				outer_attributes,
				variant: match variant {
					ExpressionWithBlockVariant::BlockExpression(b) => {
						AnyExpressionVariant::BlockExpression(b)
					}
				},
			},
		}
	}
}

impl From<AnyExpression> for Expression {
	fn from(value: AnyExpression) -> Self {
		let AnyExpression {
			outer_attributes,
			variant,
		} = value;
		match variant {
			AnyExpressionVariant::LiteralExpression(l) => {
				Self::ExpressionWithoutBlock(ExpressionWithoutBlock {
					outer_attributes,
					variant: ExpressionWithoutBlockVariant::LiteralExpression(l),
				})
			}
			AnyExpressionVariant::BlockExpression(b) => {
				Self::ExpressionWithBlock(ExpressionWithBlock {
					outer_attributes,
					variant: ExpressionWithBlockVariant::BlockExpression(b),
				})
			}
		}
	}
}
