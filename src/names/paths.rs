use crate::{
	identifiers::Identifier,
	io::{Input, Parse},
	tokens::{
		keywords::{Crate, Selftype, Selfvalue, Super},
		punctuation::{Comma, Dollar, DotDot, Gt, Lt, Minus},
	},
};

pub struct PathInExpression<'a> {
	pub dot_dot: Option<DotDot>,
	pub path_expr_segment: PathExprSegment<'a>,
	pub rest: Vec<(DotDot, PathExprSegment<'a>)>,
}

impl<'a> Parse<'a> for PathInExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			dot_dot: input.parse(),
			path_expr_segment: input.parse(),
			rest: input.parse(),
		}
	}
}

pub struct PathExprSegment<'a> {
	pub path_ident_segment: PathIdentSegment,
	pub generics: Option<(DotDot, GenericArgs<'a>)>,
}

impl<'a> Parse<'a> for PathExprSegment<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			path_ident_segment: input.parse(),
			generics: input.parse(),
		}
	}
}

pub enum PathIdentSegment {
	Identifier(Identifier),
	Super(Super),
	Selfvalue(Selfvalue),
	Selftype(Selftype),
	Crate(Crate),
	DollarCrate(Dollar, Crate), //TODO: Make sure this isn't joined.
}

impl Parse<'_> for PathIdentSegment {
	fn parse(input: &mut Input<'_>) -> Self {
		if let (identifier, Ok(())) = input.try_parse() {
			Self::Identifier(identifier)
		} else if let (super_, Ok(())) = input.try_parse() {
			Self::Super(super_)
		} else if let (selfvalue, Ok(())) = input.try_parse() {
			Self::Selfvalue(selfvalue)
		} else if let (selftype, Ok(())) = input.try_parse() {
			Self::Selftype(selftype)
		} else if let (crate_, Ok(())) = input.try_parse() {
			Self::Crate(crate_)
		} else if let ((dollar, crate_), Ok(())) = input.try_parse() {
			Self::DollarCrate(dollar, crate_)
		} else {
			todo!()
		}
	}
}

pub enum GenericArgs<'a> {
	Empty(Lt, Gt),
	Some(
		Lt,
		GenericArg<'a>,
		Vec<(Comma, GenericArg<'a>)>,
		Option<Comma>,
	),
}

impl<'a> Parse<'a> for GenericArgs<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let ((lt, gt), Ok(())) = input.try_parse() {
			Self::Empty(lt, gt)
		} else if let ((lt, generic_arg, more_generic_args, comma), Ok(())) = input.try_parse() {
			Self::Some(lt, generic_arg, more_generic_args, comma)
		} else {
			todo!()
		}
	}
}

pub enum GenericArg<'a> {
	Lifetime(Lifetime),
	Type(Type<'a>),
	GenericArgsConst(GenericArgsConst<'a>),
	GenericArgsBinding(GenericArgsBinding<'a>),
}

impl<'a> Parse<'a> for GenericArg<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (lifetime, Ok(())) = input.try_parse() {
			Self::Lifetime(lifetime)
		} else if let (r#type, Ok(())) = input.try_parse() {
			Self::Type(r#type)
		} else if let (gac, Ok(())) = input.try_parse() {
			Self::GenericArgsConst(gac)
		} else if let (gab, Ok(())) = input.try_parse() {
			Self::GenericArgsBinding(gab)
		} else {
			todo!()
		}
	}
}

pub enum GenericArgsConst<'a> {
	BlockExpression(BlockExpression<'a>),
	LiteralExpression(LiteralExpression),
	MinusLiteralExpression(Minus, LiteralExpression),
	SimplePathSegment(SimplePathSegment),
}

impl<'a> Parse<'a> for GenericArgsConst<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (be, Ok(())) = input.try_parse() {
			Self::BlockExpression(be)
		} else if let (le, Ok(())) = input.try_parse() {
			Self::LiteralExpression(le)
		} else if let ((minus, le), Ok(())) = input.try_parse() {
			Self::MinusLiteralExpression(minus, le)
		} else if let (sps, Ok(())) = input.try_parse() {
			Self::SimplePathSegment(sps)
		} else {
			todo!()
		}
	}
}

pub struct GenericArgsBinding<'a> {
	pub idendifier: Identifier,
	pub r#type: Type<'a>,
}

impl<'a> Parse<'a> for GenericArgsBinding<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			idendifier: input.parse(),
			r#type: input.parse(),
		}
	}
}
