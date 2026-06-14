//! [statement](https://doc.rust-lang.org/reference/statements.html#r-statement): Statements

use loess::{Error, ErrorPriority, Input, PeekFrom, PopParsedFrom, grammar, scaffold::Greedy};

use crate::{
	attributes::OuterAttribute, items::Item, lex::token::punct::Semi,
	r#macro::invocation::MacroInvocationSemi, statement::r#let::LetStatement,
};

pub mod r#let;

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Statement](https://doc.rust-lang.org/reference/statements.html?highlight=statement#r-statement.syntax)
	pub enum Statement: IntoTokens {
		Semi(Semi),
		Item(Item),
		LetStatement(LetStatement),
		OuterAttributesMacroInvocationSemi(Greedy<Vec<OuterAttribute>>, MacroInvocationSemi),
	} else _;
}

impl PeekFrom for Statement {
	fn peek_from(input: &Input) -> bool {
		todo!("Statement::peek_from")
	}
}

impl PopParsedFrom for Statement {
	type Parsed = Self;

	fn pop_parsed_from(
		input: &mut Input,
		errors: &mut loess::Errors,
	) -> Result<Self::Parsed, Option<Self::Parsed>> {
		Ok(
			if let Some(semi) =
				Semi::peek_pop_parsed_from(input, errors).map_err(|o| o.map(Self::Semi))?
			{
				Self::Semi(semi)
			} else if let Some(item) =
				Item::peek_pop_parsed_from(input, errors).map_err(|o| o.map(Self::Item))?
			{
				Self::Item(item)
			} else if let Some(let_statement) = LetStatement::peek_pop_parsed_from(input, errors)
				.map_err(|o| o.map(Self::LetStatement))?
			{
				Self::LetStatement(let_statement)
			} else {
				let attrs = Greedy::<Vec<OuterAttribute>>::pop_parsed_from(input, errors)
					.map_err(|_| None)?;
				match MacroInvocationSemi::peek_pop_parsed_from(input, errors) {
					Ok(Some(mis)) => Self::OuterAttributesMacroInvocationSemi(attrs, mis),
					Err(o) => {
						return Err(
							o.map(|mis| Self::OuterAttributesMacroInvocationSemi(attrs, mis))
						);
					}
					Ok(None) => {
						errors.push(Error::new(
							ErrorPriority::GRAMMAR,
							"Expected Statement.",
							[input.front_span()],
						));
						return Err(None);
					}
				}
			},
		)
	}
}
