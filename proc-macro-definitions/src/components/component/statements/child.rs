use loess::{
	rust_reference::{CurlyBraces, Identifier, Semi},
	Error, ErrorPriority, Errors, Input, PeekFrom, PopFrom, SimpleSpanned,
};
use proc_macro2::TokenStream;

use super::Statement;

pub struct Child {
	pub identifier: ChildIdentifier,
	pub children: ChildChildren,
}

impl PeekFrom for Child {
	fn peek_from(input: &Input) -> bool {
		ChildIdentifier::peek_from(input)
	}
}

impl PopFrom for Child {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(Self {
			identifier: ChildIdentifier::pop_from(input, errors)?,
			children: ChildChildren::pop_from(input, errors)?,
		})
	}
}

pub enum ChildIdentifier {
	Local(Identifier),
	Substrate(Identifier),
	Qualified(TokenStream),
}

impl PeekFrom for ChildIdentifier {
	fn peek_from(input: &Input) -> bool {
		//TODO: Or ColonColon.
		Identifier::peek_from(input)
	}
}

impl PopFrom for ChildIdentifier {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(
			if let Some(identifier) = Identifier::peek_pop_from(input, errors)? {
				let c = identifier
					.0
					.to_string()
					.chars()
					.next()
					.expect("No zero-length identifiers, hopefully!");
				if c.is_uppercase() {
					Self::Local(identifier)
				} else if c.is_lowercase() {
					Self::Substrate(identifier)
				} else {
					return Err(errors.push(Error::new(
						ErrorPriority::GRAMMAR,
						"Expected identifier to be either upper- or lowercase.",
						[identifier.span()],
					)));
				}
			} else {
				return Err(errors.push(Error::new(
					ErrorPriority::GRAMMAR,
					"Expected child type identifier or path statement.",
					[input.front_span()],
				)));
			},
		)
	}
}

pub enum ChildChildren {
	Void(Semi),
	Braces(CurlyBraces<Vec<Statement>>), //TODO: Named slots.
}

impl PopFrom for ChildChildren {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		Ok(if let Some(semi) = Semi::peek_pop_from(input, errors)? {
			Self::Void(semi)
		} else if let Some(braces) = CurlyBraces::peek_pop_from(input, errors)? {
			Self::Braces(braces)
		} else {
			return Err(errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Expected `;` or `{`.",
				[input.front_span()],
			)));
		})
	}
}
