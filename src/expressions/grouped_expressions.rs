use std::fmt::Write;

use crate::{
	io::{Input, Parse},
	tokens::delimiters::Parentheses,
};

use super::Expression;

#[derive(Debug, Clone, Default)]
pub struct GroupedExpression<'a> {
	pub parens: Parentheses<'a, Box<Expression<'a>>>,
}

impl<'a> Parse<'a> for GroupedExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			parens: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		Parentheses::<Expression>::describe(w)
	}
}
