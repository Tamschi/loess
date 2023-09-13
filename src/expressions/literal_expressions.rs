use proc_macro2::Literal;

use crate::io::{Input, Parse};

pub struct LiteralExpression {
	pub literal: Literal,
}

impl Parse<'_> for LiteralExpression {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			literal: input.parse(),
		}
	}
}
