use std::fmt::Write;

use crate::io::{Input, Parse};

use super::{
	limitations::{ExpressionLimitation, NONE},
	Expression,
};

#[derive(Debug, Clone, Default)]
pub struct Scrutinee<'a, LIMITATION: ExpressionLimitation = NONE> {
	pub expression: Box<Expression<'a, LIMITATION::EXCEPT_STRUCT_EXPRESSION>>,
}

impl<'a, LIMITATION: ExpressionLimitation> Parse<'a> for Scrutinee<'a, LIMITATION> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		Expression::<LIMITATION::EXCEPT_STRUCT_EXPRESSION>::describe(w)
	}
}
