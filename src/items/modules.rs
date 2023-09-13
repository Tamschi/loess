use std::fmt::Write;

use either::Either;

use crate::{
	attributes::InnerAttribute,
	identifiers::Identifier,
	io::{Input, Parse},
	tokens::{
		delimiters::Braces,
		keywords::{Mod, Unsafe},
		punctuation::Semi,
	},
};

use super::Item;

pub struct Module<'a> {
	r#unsafe: Option<Unsafe>,
	r#mod: Mod,
	identifier: Identifier,
	semi_or_braces: Either<Semi, Braces<'a, (Vec<InnerAttribute<'a>>, Vec<Item<'a>>)>>,
}

impl<'a> Parse<'a> for Module<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#unsafe: input.parse(),
			r#mod: input.parse(),
			identifier: input.parse(),
			semi_or_braces: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			Option<Unsafe>,
			Mod,
			Identifier,
			Either<Semi, Braces<(Vec<InnerAttribute>, Vec<Item>)>>,
		)>::describe(w)
	}
}

impl Default for Module<'_> {
	fn default() -> Self {
		Self {
			r#unsafe: None,
			r#mod: Mod::default(),
			identifier: Identifier::default(),
			semi_or_braces: Either::Right(Braces::default()),
		}
	}
}
