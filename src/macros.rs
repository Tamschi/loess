use std::fmt::Write;

use crate::{
	io::{random_access::TokenTree, Input, Parse},
	names::paths::SimplePath,
	tokens::{
		delimiters::{Braces, Brackets, Parentheses},
		punctuation::Not,
	},
};

pub struct MacroInvocation<'a> {
	pub simple_path: SimplePath,
	pub not: Not,
	pub delim_token_tree: DelimTokenTree<'a>,
}

impl<'a> Parse<'a> for MacroInvocation<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			simple_path: input.parse(),
			not: input.parse(),
			delim_token_tree: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(SimplePath, Not, DelimTokenTree)>::describe(w)
	}
}

impl Default for MacroInvocation<'_> {
	fn default() -> Self {
		Self {
			simple_path: Default::default(),
			not: Default::default(),
			delim_token_tree: Default::default(),
		}
	}
}

pub enum DelimTokenTree<'a> {
	Parentheses(Parentheses<'a, Vec<TokenTree<'a>>>),
	Brackets(Brackets<'a, Vec<TokenTree<'a>>>),
	Braces(Braces<'a, Vec<TokenTree<'a>>>),
}

impl<'a> Parse<'a> for DelimTokenTree<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (parentheses, Ok(())) = input.try_parse() {
			Self::Parentheses(parentheses)
		} else if let (brackets, Ok(())) = input.try_parse() {
			Self::Brackets(brackets)
		} else if let (braces, Ok(())) = input.try_parse() {
			Self::Braces(braces)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('(')?;
		Parentheses::<Vec<TokenTree>>::describe(w)?;
		w.write_char('|')?;
		Brackets::<Vec<TokenTree>>::describe(w)?;
		w.write_char('|')?;
		Braces::<Vec<TokenTree>>::describe(w)?;
		w.write_char(')')
	}
}
