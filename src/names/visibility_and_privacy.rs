use std::fmt::Write;

use crate::{
	io::{Input, Parse},
	tokens::{
		delimiters::Parentheses,
		keywords::{Crate, In, Pub, Selfvalue, Super},
	},
};

use super::paths::SimplePath;

#[derive(Default)]
pub struct Visibility<'a> {
	r#pub: Pub,
	parentheses: Option<Parentheses<'a, VisibilityVariant>>,
}

impl<'a> Parse<'a> for Visibility<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#pub: input.parse(),
			parentheses: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Pub, Parentheses<VisibilityVariant>)>::describe(w)
	}
}

pub enum VisibilityVariant {
	Crate(Crate),
	Selfvalue(Selfvalue),
	Super(Super),
	InSimplePath(In, SimplePath),
}

impl Parse<'_> for VisibilityVariant {
	fn parse(input: &mut Input<'_>) -> Self {
		if let (Crate, Ok(())) = input.try_parse() {
			Self::Crate(Crate)
		} else if let (Selfvalue, Ok(())) = input.try_parse() {
			Self::Selfvalue(Selfvalue)
		} else if let (Super, Ok(())) = input.try_parse() {
			Self::Super(Super)
		} else if let ((r#in, simple_path), Ok(())) = input.try_parse() {
			Self::InSimplePath(r#in, simple_path)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('(')?;
		Crate::describe(w)?;
		w.write_char('|')?;
		Selfvalue::describe(w)?;
		w.write_char('|')?;
		Super::describe(w)?;
		w.write_char('|')?;
		<(In, SimplePath)>::decribe(w)?;
		w.write_char(')')
	}
}

impl Default for VisibilityVariant {
	fn default() -> Self {
		Self::InSimplePath(In::default(), SimplePath::default())
	}
}
