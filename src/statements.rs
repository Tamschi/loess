use crate::{
	expressions::block_expressions::BlockExpression,
	io::Parse,
	items::Item,
	patterns::PatternNoTopAlt,
	tokens::{
		keywords::{Else, Let},
		punctuation::{Colon, Eq, Semi},
	},
};

pub enum Statement<'a> {
	Semi(Semi),
	Item(Box<Item<'a>>),
	LetStatement(LetStatement<'a>),
	ExpressionStatement(ExpressionStatement<'a>),
	MacroInvocationSemi(MacroInvocationSemi<'a>),
}

impl<'a> Parse<'a> for Statement<'a> {
	fn parse(input: &mut crate::io::Input<'a>) -> Self {
		if let (semi, Ok(())) = input.parse() {
			Self::Semi(semi)
		} else if let (item, Ok(())) = input.parse() {
			Self::Item(item)
		} else if let (let_statement, Ok(())) = input.parse() {
			Self::LetStatement(let_statement)
		} else if let (expression_statement, Ok(())) = input.parse() {
			Self::ExpressionStatement(expression_statement)
		} else if let (macro_invocation_semi, Ok(())) = input.parse() {
			Self::MacroInvocationSemi(macro_invocation_semi)
		} else {
			todo!()
		}
	}
}

pub struct LetStatement<'a> {
	outer_attributes: Vec<OuterAttribute<'a>>,
	r#let: Let,
	pattern_no_top_alt: PatternNoTopAlt<'a>,
	r#type: Option<(Colon, Type<'a>)>,
	assignment: Option<(Eq, Expression<'a>, Option<(Else, BlockExpression<'a>)>)>,
	semi: Semi,
}

impl<'a> Parse<'a> for LetStatement<'a> {
	fn parse(input: &mut crate::io::Input<'a>) -> Self {
		Self {
			outer_attributes: input.parse(),
			r#let: input.parse(),
			pattern_no_top_alt: input.parse(),
			r#type: input.parse(),
			assignment: input.parse(),
			semi: input.parse(),
		}
	}
}

pub enum ExpressionStatement<'a> {
	ExpressionWithoutBlock {
		expression_without_block: ExpressionWithoutBlock<'a>,
		semi: Semi,
	},
	ExpressionWithBlock {
		expression_with_block: ExpressionWithBlock<'a>,
		semi: Option<Semi>,
	},
}

impl<'a> Parse<'a> for ExpressionStatement<'a> {
	fn parse(input: &mut crate::io::Input<'a>) -> Self {
		if let ((expression_without_block, semi), Ok(())) = input.try_parse() {
			Self::ExpressionWithoutBlock {
				expression_without_block,
				semi,
			}
		} else if let ((expression_with_block, semi), Ok(())) = input.try_parse() {
			Self::ExpressionWithBlock {
				expression_with_block,
				semi,
			}
		} else {
			todo!()
		}
	}
}
