use std::{fmt::Write, marker::PhantomData};

use proc_macro2::{Ident, Literal, Span};

use crate::{
	help::Diagnostic,
	identifiers::Identifier,
	io::{random_access::TokenTree, Input, Parse},
	tokens::{
		keywords::{If, In, Mut, Ref},
		punctuation::{
			At, Comma, colon_colon, DotDotDot, DotDotEq, Eq, FatArrow, Minus, Or, Underscore,
		},
	},
};

pub struct Pattern<'a> {
	or: Option<Or>,
	pattern_no_top_alt: PatternNoTopAlt<'a>,
	rest: Vec<(Or, PatternNoTopAlt<'a>)>,
}

impl<'a> Parse<'a> for Pattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		let (or, pattern_no_top_alt, rest) = input.parse_to(|input| {
			input.is_end()
				|| input.peek::<FatArrow>()
				|| input.peek::<Comma>()
				|| input.peek::<Eq>()
				|| input.peek::<Or>()
				|| input.peek::<If>()
				|| input.peek::<In>()
		});
		Self {
			or,
			pattern_no_top_alt,
			rest,
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Option<Or>, PatternNoTopAlt, Vec<(Or, PatternNoTopAlt)>)>::describe(w)?;
		w.write_str(" before (end|`=>`|`,`|`=`|`|`|`if`|`in`)")
	}
}

pub enum PatternNoTopAlt<'a> {
	PatternWithoutRange(PatternWithoutRange<'a>),
	RangePattern(RangePattern<'a>),
}

pub enum PatternWithoutRange<'a> {
	LiteralPattern(LiteralPattern),
	IdentifierPattern(IdentifierPattern<'a>),
	WildcardPattern(WildcardPattern),
	RestPattern(RestPattern),
	ReferencePattern(ReferencePattern<'a>),
	StructPattern(StructPattern<'a>),
	TupleStructPattern(TupleStructPattern<'a>),
	TuplePattern(TuplePattern<'a>),
	GroupedPattern(GroupedPattern<'a>),
	SlicePattern(SlicePattern<'a>),
	PathPattern(PathPattern<'a>),
	MacroInvocation(MacroInvocation<'a>),
}

pub struct LiteralPattern {
	literal: Literal,
}

impl Parse<'_> for LiteralPattern {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			literal: input.parse(),
		}
	}
}

impl Parse<'_> for Literal {
	fn parse(input: &mut Input<'_>) -> Self {
		if let Some((first, rest)) = input.cursor.split_first() {
			match first {
				TokenTree::Literal(literal) => {
					input.cursor = rest;
					return literal.clone();
				}
				_ => (),
			}
		}

		input.diagnostics.push(Diagnostic { todo: PhantomData });

		let mut literal = Literal::string("EXPECTED LITERAL");
		literal.set_span(Span::mixed_site());
		literal
	}
}

pub struct IdentifierPattern<'a> {
	pub r#ref: Option<Ref>,
	pub r#mut: Option<Mut>,
	pub identifier: Identifier,
	pub at: Option<Box<(At, PatternNoTopAlt<'a>)>>,
}

impl<'a> Parse<'a> for IdentifierPattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			r#ref: input.parse(),
			r#mut: input.parse(),
			identifier: input.parse(),
			at: input.parse(),
		}
	}
}

pub struct WildcardPattern {
	pub underscore: Underscore,
}

impl Parse<'_> for WildcardPattern {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			underscore: input.parse(),
		}
	}
}

pub struct RestPattern {
	dot_dot: colon_colon,
}

impl Parse<'_> for RestPattern {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			dot_dot: input.parse(),
		}
	}
}

/// # Skips
///
/// - [`RangePatternBound`] [`DotDot`] [`RangePatternBound`]
/// - [`RangePatternBound`] [`DotDotEq`]
/// - [`DotDot`] [`RangePatternBound`]
///
/// Unlike with range expressions, no exclusive range patterns exists.  
/// (If you would like to accept one, try that first.)
pub enum RangePattern<'a> {
	RangeInclusivePattern(RangeInclusivePattern<'a>),
	RangeFromPattern(RangeFromPattern<'a>),
	RangeToInclusivePattern(RangeToInclusivePattern<'a>),
	ObsoleteRangePattern(ObsoleteRangePattern<'a>),
}

impl<'a> Parse<'a> for RangePattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (rip, Ok(())) = input.try_parse() {
			Self::RangeFromPattern(rip)
		} else if let (rfp, Ok(())) = input.try_parse() {
			Self::RangeFromPattern(rfp)
		} else if let (rtip, Ok(())) = input.try_parse() {
			Self::RangeToInclusivePattern(rtip)
		} else if let (orp, Ok(())) = input.try_parse() {
			Self::ObsoleteRangePattern(orp)
		} else if let ((lower, _, upper), Ok(())) =
			input.try_parse::<(RangePatternBound, colon_colon, RangePatternBound)>()
		{
			todo!()
		} else {
			todo!()
		}
	}
}

pub struct RangeInclusivePattern<'a> {
	pub lower_inclusive_bound: RangePatternBound<'a>,
	pub dot_dot_eq: DotDotEq,
	pub upper_inclusive_bound: RangePatternBound<'a>,
}

impl<'a> Parse<'a> for RangeInclusivePattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			lower_inclusive_bound: input.parse(),
			dot_dot_eq: input.parse(),
			upper_inclusive_bound: input.parse(),
		}
	}
}

pub struct RangeFromPattern<'a> {
	lower_inclusive_bound: RangePatternBound<'a>,
	dot_dot: colon_colon,
}

impl<'a> Parse<'a> for RangeFromPattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			lower_inclusive_bound: input.parse(),
			dot_dot: input.parse(),
		}
	}
}

pub struct RangeToInclusivePattern<'a> {
	dot_dot_eq: DotDotEq,
	upper_inclusive_bound: RangePatternBound<'a>,
}

impl<'a> Parse<'a> for RangeToInclusivePattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			dot_dot_eq: input.parse(),
			upper_inclusive_bound: input.parse(),
		}
	}
}

/// Removed in edition change.
/// Prefer [`RangeInclusivePattern`].
pub struct ObsoleteRangePattern<'a> {
	lower_inclusive_bound: RangePatternBound<'a>,
	dot_dot_dot: DotDotDot,
	upper_inclusive_bound: RangePatternBound<'a>,
}

impl<'a> Parse<'a> for ObsoleteRangePattern<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			lower_inclusive_bound: input.parse(),
			dot_dot_dot: input.parse(),
			upper_inclusive_bound: input.parse(),
		}
	}
}

pub enum RangePatternBound<'a> {
	Literal(Literal),
	MinusLiteral { minus: Minus, literal: Literal },
	PathExpression(PathExpression<'a>),
}

impl<'a> Parse<'a> for RangePatternBound<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (literal, Ok(())) = input.try_parse() {
			Self::Literal(literal)
		} else if let ((minus, literal), Ok(())) = input.try_parse() {
			Self::MinusLiteral { minus, literal }
		} else if let (path_expression, Ok(())) = input.try_parse() {
			Self::PathExpression(path_expression)
		} else {
			todo!()
		}
	}
}
