use std::fmt::Write;

use either::Either;

use crate::{
	expressions::Expression,
	io::{Input, Parse},
	tokens::{
		punctuation::{Eq, Not, Pound},
		Brackets,
	},
};

pub struct InnerAttribute<'a> {
	pub pound: Pound,
	pub not: Not,
	pub brackets: Brackets<'a, Attr<'a>>,
}

impl<'a> Parse<'a> for InnerAttribute<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			pound: input.parse(),
			not: input.parse(),
			brackets: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("`#![ … ]`")
	}
}

pub struct OuterAttribute<'a> {
	pub pound: Pound,
	pub brackets: Brackets<'a, Attr<'a>>,
}

impl<'a> Parse<'a> for OuterAttribute<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			pound: input.parse(),
			brackets: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("`#[ … ]`")
	}
}

pub struct Attr<'a> {
	pub simple_path: SimplePath<'a>,
	pub attr_input: Option<AttrInput<'a>>,
}

impl<'a> Parse<'a> for Attr<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			simple_path: input.parse(),
			attr_input: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(SimplePath, Option<AttrInput>)>::describe(w)
	}
}

pub enum AttrInput<'a> {
	DelimTokenTree(DelimTokenTree<'a>),
	EqExpression { eq: Eq, expression: Expression<'a> },
}

impl<'a> Parse<'a> for AttrInput<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (delim_token_tree, Ok(())) = input.try_parse() {
			Self::DelimTokenTree(delim_token_tree)
		} else if let ((eq, expression), Ok(())) = input.try_parse() {
			Self::EqExpression { eq, expression }
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		Either::<DelimTokenTree, (Eq, Expression)>::describe(w)
	}
}

impl Default for AttrInput<'_> {
	fn default() -> Self {
		Self::DelimTokenTree(DelimTokenTree::default())
	}
}
