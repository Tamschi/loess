use std::fmt::Write;

use crate::{
	io::{Input, Parse},
	patterns::Pattern,
	tokens::{
		keywords::{Break, Continue, For, In, Let, Loop, While},
		punctuation::{Colon, Eq},
		LifetimeOrLabel,
	},
};

use super::{
	block_expressions::BlockExpression,
	limitations::{EXCEPT_LAZY_BOOLEAN_OPERATOR_EXPRESSION, EXCEPT_STRUCT_EXPRESSION},
	match_expressions::Scrutinee,
	Expression,
};

pub struct LoopExpression<'a> {
	pub loop_label: Option<LoopLabel>,
	pub variant: LoopExpressionVariant<'a>,
}

impl<'a> Parse<'a> for LoopExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		let loop_label = input.parse();
		let has_label = loop_label.is_some();
		Self {
			loop_label,
			variant: if let (infinite, Ok(())) = input.try_parse() {
				Self::InfiniteLoopExpression(infinite)
			} else if let (predicated, Ok(())) = input.try_parse() {
				Self::PredicateLoopExpression(predicated)
			} else if let (patted, Ok(())) = input.try_parse() {
				Self::PredicatePatternLoopExpression(patted)
			} else if let (iteratored, Ok(())) = input.try_parse() {
				Self::IteratorLoopExpression(iteratored)
			} else if let Some((block, Ok(()))) = has_label.then_some(|| input.try_parse()) {
				Self::LabelBlockExpression(block)
			} else {
				input.error_expected()
			},
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("LoopExpression")
	}
}

/// This is just a helper enum for [`LoopExpression`], so it doesn't implement [`Parse`].
#[derive(Debug, Clone)]
pub enum LoopExpressionVariant<'a> {
	InfiniteLoopExpression(InfiniteLoopExpression<'a>),
	PredicateLoopExpression(PredicateLoopExpression<'a>),
	PredicatePatternLoopExpression(PredicatePatternLoopExpression<'a>),
	IteratorLoopExpression(IteratorLoopExpression<'a>),
	LabelBlockExpression(LabelBlockExpression<'a>),
}

impl Default for LoopExpressionVariant<'_> {
	fn default() -> Self {
		Self::InfiniteLoopExpression(InfiniteLoopExpression::default())
	}
}

#[derive(Debug, Clone, Default)]
pub struct InfiniteLoopExpression<'a> {
	pub r#loop: Loop,
	pub block_expression: BlockExpression<'a>,
}

impl<'a> Parse<'a> for InfiniteLoopExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#loop: input.parse(),
			block_expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Loop, BlockExpression)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct PredicateLoopExpression<'a> {
	pub r#while: While,
	/// Except struct expression!
	pub expression: Box<Expression<'a, EXCEPT_STRUCT_EXPRESSION>>,
	pub block_expression: BlockExpression<'a>,
}

impl<'a> Parse<'a> for PredicateLoopExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#while: input.parse(),
			expression: input.parse(),
			block_expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(While, Expression, BlockExpression<EXCEPT_STRUCT_EXPRESSION>)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct PredicatePatternLoopExpression<'a> {
	pub r#while: While,
	pub r#let: Let,
	pub pattern: Pattern<'a>,
	pub eq: Eq,
	pub scrutinee: Scrutinee<'a, EXCEPT_LAZY_BOOLEAN_OPERATOR_EXPRESSION>,
	pub block_expression: BlockExpression<'a>,
}

impl<'a> Parse<'a> for PredicatePatternLoopExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#while: input.parse(),
			r#let: input.parse(),
			pattern: input.parse(),
			eq: input.parse(),
			scrutinee: input.parse(),
			block_expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			While,
			Let,
			Pattern,
			Eq,
			Scrutinee<EXCEPT_LAZY_BOOLEAN_OPERATOR_EXPRESSION>,
			BlockExpression,
		)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct IteratorLoopExpression<'a> {
	pub r#for: For,
	pub pattern: Pattern<'a>,
	pub r#in: In,
	pub expression: Box<Expression<'a, EXCEPT_STRUCT_EXPRESSION>>,
	pub block_expression: BlockExpression<'a>,
}

impl<'a> Parse<'a> for IteratorLoopExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#for: input.parse(),
			pattern: input.parse(),
			r#in: input.parse(),
			expression: input.parse(),
			block_expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			For,
			Pattern,
			In,
			Expression<EXCEPT_STRUCT_EXPRESSION>,
			BlockExpression,
		)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct LoopLabel {
	lifetime_or_label: LifetimeOrLabel,
	colon: Colon,
}

impl Parse<'_> for LoopLabel {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			lifetime_or_label: input.parse(),
			colon: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(LifetimeOrLabel, Colon)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct BreakExpression<'a> {
	pub r#break: Break,
	pub lifetime_or_label: Option<LifetimeOrLabel>,
	pub expression: Option<Box<Expression<'a>>>,
}

impl<'a> Parse<'a> for BreakExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#break: input.parse(),
			lifetime_or_label: input.parse(),
			expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Break, Option<LifetimeOrLabel>, Option<Expression>)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct LabelBlockExpression<'a> {
	pub block: BlockExpression<'a>,
}

impl<'a> Parse<'a> for LabelBlockExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			block: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		BlockExpression::describe(w)
	}
}

pub struct ContinueExpression {
	pub r#continue: Continue,
	pub lifetime_or_label: Option<LifetimeOrLabel>,
}

impl Parse<'_> for ContinueExpression {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			r#continue: input.parse(),
			lifetime_or_label: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Continue, LifetimeOrLabel)>::describe(w)
	}
}
