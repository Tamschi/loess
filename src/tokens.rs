use std::{fmt::Write, marker::PhantomData};

use proc_macro2::{Ident, Punct, Spacing, Span, TokenTree};

use crate::io::{Input, Parse};

pub mod keywords;
pub mod punctuation;

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
