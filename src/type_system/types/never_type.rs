use std::fmt::Write;

use crate::{
	io::{Input, Parse},
	tokens::punctuation::Not,
};

pub struct NeverType {
	not: Not,
}

impl Parse<'_> for NeverType {
	fn parse(input: &mut Input<'_>) -> Self {
		Self { not: input.parse() }
	}

	fn describe(w: &mut dyn Write) {
		Not::describe(w)
	}
}

impl Default for NeverType {
	fn default() -> Self {
		Self {
			not: Not::default(),
		}
	}
}
