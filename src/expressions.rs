use crate::io::{Input, Parse};

use self::{
	block_expressions::{AsyncBlockExpression, BlockExpression, UnsafeBlockExpression},
	literal_expressions::LiteralExpression,
	operator_expressions::OperatorExpression,
	path_expressions::PathExpression,
};

pub mod block_expressions;
pub mod literal_expressions;
pub mod operator_expressions;
pub mod path_expressions;

pub enum Expression<'a> {
	ExpressionWithoutBlock(ExpressionWithoutBlock<'a>),
	ExpressionWithBlock(ExpressionWithBlock<'a>),
}

impl<'a> Parse<'a> for Expression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (ewb, Ok(())) = input.try_parse() {
			Self::ExpressionWithoutBlock(ewb)
		} else if let (ewb, Ok(())) = input.try_parse() {
			Self::ExpressionWithBlock(ewb)
		} else {
			todo!()
		}
	}
}

pub struct ExpressionWithoutBlock<'a> {
	pub outer_attributes: Vec<OuterAttribute<'a>>,
	pub variant: ExpressionWithoutBlockContent<'a>,
}

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			outer_attributes: input.parse(),
			variant: input.parse(),
		}
	}
}

pub enum ExpressionWithoutBlockContent<'a> {
	LiteralExpression(LiteralExpression),
	PathExpression(PathExpression<'a>),
	OperatorExpression(OperatorExpression<'a>),
	GroupedExpression(GroupedExpression<'a>),
	ArrayExpression(ArrayExpression<'a>),
	AwaitExpression(AwaitExpression<'a>),
	IndexExpression(IndexExpression<'a>),
	TupleExpression(TupleExpression<'a>),
	TupleIndexingExpression(TupleIndexingExpression<'a>),
	StructExpression(StructExpression<'a>),
	CallExpression(CallExpression<'a>),
	MethodCallExpression(MethodCallExpression<'a>),
	FieldExpression(FieldExpression<'a>),
	ClosureExpression(ClosureExpression<'a>),
	AsyncBlockExpression(AsyncBlockExpression<'a>),
	ContinueExpression(ContinueExpression<'a>),
	BreakExpression(BreakExpression<'a>),
	RangeExpression(RangeExpression<'a>),
	ReturnExpression(ReturnExpression<'a>),
	UnderscoreExpression(UnderscoreExpression<'a>),
	MacroInvocation(MacroInvocation<'a>),
}

impl<'a> Parse<'a> for ExpressionWithoutBlockContent<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		//TODO: snake_case
		if let (LiteralExpression, Ok(())) = input.try_parse() {
			Self::LiteralExpression(LiteralExpression)
		} else if let (PathExpression, Ok(())) = input.try_parse() {
			Self::PathExpression(PathExpression)
		} else if let (OperatorExpression, Ok(())) = input.try_parse() {
			Self::OperatorExpression(OperatorExpression)
		} else if let (GroupedExpression, Ok(())) = input.try_parse() {
			Self::GroupedExpression(GroupedExpression)
		} else if let (ArrayExpression, Ok(())) = input.try_parse() {
			Self::ArrayExpression(ArrayExpression)
		} else if let (AwaitExpression, Ok(())) = input.try_parse() {
			Self::AwaitExpression(AwaitExpression)
		} else if let (IndexExpression, Ok(())) = input.try_parse() {
			Self::IndexExpression(IndexExpression)
		} else if let (TupleExpression, Ok(())) = input.try_parse() {
			Self::TupleExpression(TupleExpression)
		} else if let (TupleIndexingExpression, Ok(())) = input.try_parse() {
			Self::TupleIndexingExpression(TupleIndexingExpression)
		} else if let (StructExpression, Ok(())) = input.try_parse() {
			Self::StructExpression(StructExpression)
		} else if let (CallExpression, Ok(())) = input.try_parse() {
			Self::CallExpression(CallExpression)
		} else if let (MethodCallExpression, Ok(())) = input.try_parse() {
			Self::MethodCallExpression(MethodCallExpression)
		} else if let (FieldExpression, Ok(())) = input.try_parse() {
			Self::FieldExpression(FieldExpression)
		} else if let (ClosureExpression, Ok(())) = input.try_parse() {
			Self::ClosureExpression(ClosureExpression)
		} else if let (AsyncBlockExpression, Ok(())) = input.try_parse() {
			Self::AsyncBlockExpression(AsyncBlockExpression)
		} else if let (ContinueExpression, Ok(())) = input.try_parse() {
			Self::ContinueExpression(ContinueExpression)
		} else if let (BreakExpression, Ok(())) = input.try_parse() {
			Self::BreakExpression(BreakExpression)
		} else if let (RangeExpression, Ok(())) = input.try_parse() {
			Self::RangeExpression(RangeExpression)
		} else if let (ReturnExpression, Ok(())) = input.try_parse() {
			Self::ReturnExpression(ReturnExpression)
		} else if let (UnderscoreExpression, Ok(())) = input.try_parse() {
			Self::UnderscoreExpression(UnderscoreExpression)
		} else if let (MacroInvocation, Ok(())) = input.try_parse() {
			Self::MacroInvocation(MacroInvocation)
		} else {
			todo!()
		}
	}
}

pub struct ExpressionWithBlock<'a> {
	pub outer_attributes: Vec<OuterAttribute<'a>>,
	pub variant: ExpressionWithBlockContent<'a>,
}

impl<'a> Parse<'a> for ExpressionWithBlock<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			outer_attributes: input.parse(),
			variant: input.parse(),
		}
	}
}

pub enum ExpressionWithBlockContent<'a> {
	BlockExpression(BlockExpression<'a>),
	UnsafeBlockExpression(UnsafeBlockExpression<'a>),
	LoopExpression(LoopExpression<'a>),
	IfExpression(IfExpression<'a>),
	IfLetExpression(IfLetExpression<'a>),
	MatchExpression(MatchExpression<'a>),
}

impl<'a> Parse<'a> for ExpressionWithBlockContent<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		//TODO: snake_case
		if let (BlockExpression, Ok(())) = input.try_parse() {
			Self::BlockExpression(BlockExpression)
		} else if let (UnsafeBlockExpression, Ok(())) = input.try_parse() {
			Self::UnsafeBlockExpression(UnsafeBlockExpression)
		} else if let (LoopExpression, Ok(())) = input.try_parse() {
			Self::LoopExpression(LoopExpression)
		} else if let (IfExpression, Ok(())) = input.try_parse() {
			Self::IfExpression(IfExpression)
		} else if let (IfLetExpression, Ok(())) = input.try_parse() {
			Self::IfLetExpression(IfLetExpression)
		} else if let (MatchExpression, Ok(())) = input.try_parse() {
			Self::MatchExpression(MatchExpression)
		} else {
			todo!()
		}
	}
}
