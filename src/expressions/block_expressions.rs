use vec1::Vec1;

use crate::{
	help::DiagnosticsList,
	io::{Input, Parse},
	tokens::{
		keywords::{Async, Move, Unsafe},
		Braces,
	},
};

pub struct BlockExpression<'a> {
	pub braces: Braces<'a, (Vec<InnerAttribute<'a>>, Statements<'a>)>,
}

impl<'a> Parse<'a> for BlockExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			braces: input.parse(),
		}
	}
}

pub enum Statements<'a> {
	Statements {
		statements: Vec1<Statement<'a>>,
	},
	StatementsExpressionWithoutBlock {
		statements: Vec1<Statement<'a>>,
		expression_without_block: ExpressionWithoutBlock<'a>,
	},
	ExpressionWithoutBlock {
		expression_without_block: ExpressionWithoutBlock<'a>,
	},
}

pub struct AsyncBlockExpression<'a> {
	pub r#async: Async,
	pub r#move: Option<Move>,
	pub block_expression: BlockExpression<'a>,
}

impl<'a> Parse<'a> for AsyncBlockExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#async: input.parse(),
			r#move: input.parse(),
			block_expression: input.parse(),
		}
	}
}

pub struct UnsafeBlockExpression<'a> {
	pub r#unsafe: Unsafe,
	pub block_expression: BlockExpression<'a>,
}

impl<'a> Parse<'a> for UnsafeBlockExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#unsafe: input.parse(),
			block_expression: input.parse(),
		}
	}
}
