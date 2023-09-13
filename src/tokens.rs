use std::{fmt::Write, marker::PhantomData};

use proc_macro2::{extra::DelimSpan, Delimiter, Ident, Punct, Spacing, Span};

use crate::{
	identifiers::Identifier,
	io::{random_access::TokenTree, Input, Parse},
};

pub mod keywords;
pub mod punctuation;

pub struct LifetimeOrLabel {
	apostrophe: SPunct<'\'', true>,
	identifier: Identifier,
}

impl Parse<'_> for LifetimeOrLabel {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			apostrophe: input.parse(),
			identifier: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("LIFETIME_OR_LABEL")
	}
}

impl Default for LifetimeOrLabel {
	fn default() -> Self {
		Self {
			apostrophe: SPunct::default(),
			identifier: Identifier::default(),
		}
	}
}

pub struct SPunct<const CH: char, const JOINT: bool> {
	punct: Punct,
}

impl<const CH: char, const JOINT: bool> SPunct<CH, JOINT> {
	fn new(span: Span) -> Self {
		let mut punct = Punct::new(
			CH,
			if JOINT {
				Spacing::Joint
			} else {
				Spacing::Alone
			},
		);
		punct.set_span(span);
		Self { punct }
	}
}

impl<const CH: char, const JOINT: bool> Default for SPunct<CH, JOINT> {
	fn default() -> Self {
		Self::new(Span::mixed_site())
	}
}

impl<const CH: char, const JOINT: bool> From<SPunct<CH, JOINT>> for Punct {
	fn from(val: SPunct<CH, JOINT>) -> Self {
		val.punct
	}
}

impl<const CH: char, const JOINT: bool> TryFrom<Punct> for SPunct<CH, JOINT> {
	type Error = Punct;

	fn try_from(value: Punct) -> Result<Self, Self::Error> {
		if value.as_char() != CH
			|| match value.spacing() {
				Spacing::Alone => JOINT,
				Spacing::Joint => !JOINT,
			} {
			Err(value)
		} else {
			Ok(Self { punct: value })
		}
	}
}

impl<const CH: char, const JOINT: bool> Parse<'_> for SPunct<CH, JOINT> {
	fn parse(input: &mut Input<'_>) -> Self {
		match input.cursor.first() {
			None => todo!(),
			Some(TokenTree::Punct(punct)) => match punct.clone().try_into() {
				Ok(s_punct) => {
					input.consume(1);
					s_punct
				}
				Err(punct) => todo!(),
			},
			Some(_) => todo!(),
		}
	}

	fn describe(w: &mut dyn Write) {
		unimplemented!("To parse custom punctuation, please wrap all `Punct`s in `Punctuation`.")
	}
}

pub struct Punctuation<SPunctsTuple> {
	pub s_puncts: SPunctsTuple,
}

impl<const CH1: char> Punctuation<(SPunct<CH1, false>,)> {
	pub fn new(span: Span) -> Self {
		Self {
			s_puncts: (SPunct::new(span),),
		}
	}
}

impl<const CH1: char, const CH2: char> Punctuation<(SPunct<CH1, true>, SPunct<CH2, false>)> {
	pub fn new(span: Span) -> Self {
		Self {
			s_puncts: (SPunct::new(span), SPunct::new(span)),
		}
	}
}
impl<const CH1: char, const CH2: char, const CH3: char>
	Punctuation<(SPunct<CH1, true>, SPunct<CH2, true>, SPunct<CH3, false>)>
{
	pub fn new(span: Span) -> Self {
		Self {
			s_puncts: (SPunct::new(span), SPunct::new(span), SPunct::new(span)),
		}
	}
}

impl<const CH1: char, const CH2: char, const CH3: char, const CH4: char>
	Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, false>,
	)>
{
	pub fn new(span: Span) -> Self {
		Self {
			s_puncts: (
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
			),
		}
	}
}

impl<const CH1: char, const CH2: char, const CH3: char, const CH4: char, const CH5: char>
	Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, true>,
		SPunct<CH5, false>,
	)>
{
	pub fn new(span: Span) -> Self {
		Self {
			s_puncts: (
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
			),
		}
	}
}

impl<
		const CH1: char,
		const CH2: char,
		const CH3: char,
		const CH4: char,
		const CH5: char,
		const CH6: char,
	>
	Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, true>,
		SPunct<CH5, true>,
		SPunct<CH6, false>,
	)>
{
	pub fn new(span: Span) -> Self {
		Self {
			s_puncts: (
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
				SPunct::new(span),
			),
		}
	}
}

impl<const CH1: char> Parse<'_> for Punctuation<(SPunct<CH1, false>,)> {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			s_puncts: (input.parse(),),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('`')?;
		w.write_char(CH1)?;
		w.write_char('`')
	}
}
impl<const CH1: char, const CH2: char> Parse<'_>
	for Punctuation<(SPunct<CH1, true>, SPunct<CH2, false>)>
{
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			s_puncts: (input.parse(), input.parse()),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('`')?;
		w.write_char(CH1)?;
		w.write_char(CH2)?;
		w.write_char('`')
	}
}
impl<const CH1: char, const CH2: char, const CH3: char> Parse<'_>
	for Punctuation<(SPunct<CH1, true>, SPunct<CH2, true>, SPunct<CH3, false>)>
{
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			s_puncts: (input.parse(), input.parse(), input.parse()),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('`')?;
		w.write_char(CH1)?;
		w.write_char(CH2)?;
		w.write_char(CH3)?;
		w.write_char('`')
	}
}
impl<const CH1: char, const CH2: char, const CH3: char, const CH4: char> Parse<'_>
	for Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, false>,
	)>
{
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			s_puncts: (input.parse(), input.parse(), input.parse(), input.parse()),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('`')?;
		w.write_char(CH1)?;
		w.write_char(CH2)?;
		w.write_char(CH3)?;
		w.write_char(CH4)?;
		w.write_char('`')
	}
}
impl<const CH1: char, const CH2: char, const CH3: char, const CH4: char, const CH5: char> Parse<'_>
	for Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, true>,
		SPunct<CH5, false>,
	)>
{
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			s_puncts: (
				input.parse(),
				input.parse(),
				input.parse(),
				input.parse(),
				input.parse(),
			),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('`')?;
		w.write_char(CH1)?;
		w.write_char(CH2)?;
		w.write_char(CH3)?;
		w.write_char(CH4)?;
		w.write_char(CH5)?;
		w.write_char('`')
	}
}
impl<
		const CH1: char,
		const CH2: char,
		const CH3: char,
		const CH4: char,
		const CH5: char,
		const CH6: char,
	> Parse<'_>
	for Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, true>,
		SPunct<CH5, true>,
		SPunct<CH6, false>,
	)>
{
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			s_puncts: (
				input.parse(),
				input.parse(),
				input.parse(),
				input.parse(),
				input.parse(),
				input.parse(),
			),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('`')?;
		w.write_char(CH1)?;
		w.write_char(CH2)?;
		w.write_char(CH3)?;
		w.write_char(CH4)?;
		w.write_char(CH5)?;
		w.write_char(CH6)?;
		w.write_char('`')
	}
}

impl<const CH1: char> Default for Punctuation<(SPunct<CH1, false>,)> {
	fn default() -> Self {
		Self {
			s_puncts: (SPunct::default(),),
		}
	}
}
impl<const CH1: char, const CH2: char> Default
	for Punctuation<(SPunct<CH1, true>, SPunct<CH2, false>)>
{
	fn default() -> Self {
		Self {
			s_puncts: (SPunct::default(), SPunct::default()),
		}
	}
}
impl<const CH1: char, const CH2: char, const CH3: char> Default
	for Punctuation<(SPunct<CH1, true>, SPunct<CH2, true>, SPunct<CH3, false>)>
{
	fn default() -> Self {
		Self {
			s_puncts: (SPunct::default(), SPunct::default(), SPunct::default()),
		}
	}
}
impl<const CH1: char, const CH2: char, const CH3: char, const CH4: char> Default
	for Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, false>,
	)>
{
	fn default() -> Self {
		Self {
			s_puncts: (
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
			),
		}
	}
}
impl<const CH1: char, const CH2: char, const CH3: char, const CH4: char, const CH5: char> Default
	for Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, true>,
		SPunct<CH5, false>,
	)>
{
	fn default() -> Self {
		Self {
			s_puncts: (
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
			),
		}
	}
}
impl<
		const CH1: char,
		const CH2: char,
		const CH3: char,
		const CH4: char,
		const CH5: char,
		const CH6: char,
	> Default
	for Punctuation<(
		SPunct<CH1, true>,
		SPunct<CH2, true>,
		SPunct<CH3, true>,
		SPunct<CH4, true>,
		SPunct<CH5, true>,
		SPunct<CH6, false>,
	)>
{
	fn default() -> Self {
		Self {
			s_puncts: (
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
				SPunct::default(),
			),
		}
	}
}

#[deprecated = "Please don't use this directly."]
pub trait KeywordString {
	const KW: &'static str;
}

#[allow(deprecated)]
pub struct Keyword<KW: KeywordString> {
	ident: Ident,
	phantom: PhantomData<KW>,
}

trait Delimiter_ {
	const DELIMITER: Delimiter;
	const OPEN: char;
	const CLOSE: char;
}

enum PARENTHESIS {}
impl Delimiter_ for PARENTHESIS {
	const DELIMITER: Delimiter = Delimiter::Parenthesis;
	const OPEN: char = '(';
	const CLOSE: char = ')';
}

enum BRACE {}
impl Delimiter_ for BRACE {
	const DELIMITER: Delimiter = Delimiter::Brace;
	const OPEN: char = '{';
	const CLOSE: char = '}';
}

enum BRACKET {}
impl Delimiter_ for BRACKET {
	const DELIMITER: Delimiter = Delimiter::Bracket;
	const OPEN: char = '[';
	const CLOSE: char = ']';
}

pub struct Delimited<'a, Delimiter, Contents>
where
	Delimiter: Delimiter_,
{
	pub delimiter: PhantomData<Delimiter>,
	pub delim_span: DelimSpan,
	pub enclosed: &'a [TokenTree<'a>],
	pub contents: PhantomData<Contents>,
}

impl<'a, Delimiter, Contents> Parse<'a> for Delimited<'a, Delimiter, Contents>
where
	Delimiter: Delimiter_,
{
	fn parse(input: &mut Input<'a>) -> Self {
		match input.cursor.first() {
			Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::DELIMITER => Self {
				delimiter: PhantomData,
				delim_span: group.delim_span(),
				enclosed: group.contents.as_slice(),
				contents: PhantomData,
			},
			_ => input.error_expected(),
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("`{ … }`")
	}
}

pub mod delimiters {
	use super::{Delimited, BRACE, BRACKET, PARENTHESIS};

	pub type Parentheses<'a, Content> = Delimited<'a, PARENTHESIS, Content>;
	pub type Braces<'a, Content> = Delimited<'a, BRACE, Content>;
	pub type Brackets<'a, Content> = Delimited<'a, BRACKET, Content>;
}
