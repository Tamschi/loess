use std::fmt::Write;

use either::Either;

use crate::{
	attributes::OuterAttribute,
	io::{Input, Parse},
	names::visibility_and_privacy::Visibility,
};

pub mod modules;

pub struct Item<'a> {
	outer_attributes: Vec<OuterAttribute<'a>>,
	variant: Either<VisItem<'a>, MacroItem<'a>>,
}

impl<'a> Parse<'a> for Item<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			outer_attributes: input.parse(),
			variant: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Vec<OuterAttribute>, Either<VisItem, MacroItem>)>::describe(w)
	}
}

pub struct VisItem<'a> {
	visibility: Option<Visibility<'a>>,
	variant: VisItemVariant<'a>,
}

impl<'a> Parse<'a> for VisItem<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			visibility: input.parse(),
			variant: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Option<Visibility>, VisItemVariant)>::describe(w)
	}
}

pub enum MacroItem<'a> {
	MacroInvocationSemi(MacroInvocationSemi<'a>),
	MacroRulesDefinition(MacroRulesDefinition<'a>),
}

impl<'a> Parse<'a> for MacroItem<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (mis, Ok(())) = input.try_parse() {
			Self::MacroInvocationSemi(mis)
		} else if let (mrd, Ok(())) = input.try_parse() {
			Self::MacroRulesDefinition(mis)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		Either::<MacroInvocationSemi, MacroRulesDefinition>::describe(w)
	}
}
