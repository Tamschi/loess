use std::fmt::Write;

use either::Either;
use vec1::Vec1;

use crate::{
	expressions::{block_expressions::BlockExpression, literal_expressions::LiteralExpression},
	identifiers::Identifier,
	io::{Input, Parse},
	tokens::{
		delimiters::Parentheses,
		keywords::{As, Crate, Selftype, Selfvalue, Super},
		punctuation::{Comma, Dollar, DotDot, Gt, Lt, Minus, RArrow},
	},
	type_system::{trait_and_lifetime_bounds::Lifetime, types::Type},
};

#[derive(Debug, Clone, Default)]
pub struct SimplePath {
	pub dot_dot: Option<DotDot>,
	pub simple_path_segment: SimplePathSegment,
	pub rest: Vec<(DotDot, SimplePathSegment)>,
}

impl Parse<'_> for SimplePath {
	fn parse(input: &mut Input<'_>) -> Self {
		Self {
			dot_dot: input.parse(),
			simple_path_segment: input.parse(),
			rest: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			Option<DotDot>,
			SimplePathSegment,
			Vec<(DotDot, SimplePathSegment)>,
		)>::describe(w)
	}
}

pub enum SimplePathSegment {
	Identifier(Identifier),
	Super(Super),
	Selfvalue(Selfvalue),
	Crate(Crate),
	DollarCrate(Dollar, Crate),
}

impl Parse<'_> for SimplePathSegment {
	fn parse(input: &mut Input<'_>) -> Self {
		todo!()
	}

	fn describe(w: &mut dyn Write) {
		w.write_char('(')?;
		Identifier::describe(w)?;
		w.write_str("|`super`|`self`|`crate`|`$crate)`")
	}
}

impl Default for SimplePathSegment {
	fn default() -> Self {
		Self::Identifier(Identifier::default())
	}
}

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

#[derive(Default)]
pub struct QualifiedPathInExpression<'a> {
	qualified_path_type: QualifiedPathType<'a>,
	rest: Vec1<(DotDot, PathExprSegment<'a>)>,
}

impl<'a> Parse<'a> for QualifiedPathInExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			qualified_path_type: input.parse(),
			rest: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(QualifiedPathType, Vec1<(DotDot, PathExprSegment)>)>::describe(w)
	}
}

#[derive(Default)]
pub struct QualifiedPathType<'a> {
	lt: Lt,
	r#type: Type<'a>,
	as_type_path: Option<(As, TypePath<'a>)>,
	gt: Gt,
}

impl<'a> Parse<'a> for QualifiedPathType<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			lt: input.parse(),
			r#type: input.parse(),
			as_type_path: input.parse(),
			gt: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Lt, Type<'a>, Option<(As, TypePath<'a>)>, Gt)>::describe(w)
	}
}

#[derive(Default)]
pub struct QualifiedPathInType<'a> {
	qualified_path_type: QualifiedPathType<'a>,
	rest: Vec1<(DotDot, TypePathSegment<'a>)>,
}

impl<'a> Parse<'a> for QualifiedPathInType<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			qualified_path_type: input.parse(),
			rest: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(QualifiedPathType, Vec1<(DotDot, TypePathSegment)>)>::describe(w)
	}
}

#[derive(Default)]
pub struct TypePath<'a> {
	dot_dot: Option<DotDot>,
	type_path_segment: TypePathSegment<'a>,
	rest: Vec<(DotDot, TypePathSegment<'a>)>,
}

impl<'a> Parse<'a> for TypePath<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			dot_dot: input.parse(),
			type_path_segment: input.parse(),
			rest: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			Option<DotDot>,
			TypePathSegment,
			Vec<(DotDot, TypePathSegment)>,
		)>::describe(w)
	}
}

#[derive(Default)]
pub struct TypePathSegment<'a> {
	path_ident_segment: PathIdentSegment<'a>,
	generic_args_or_type_path_fn: Option<(Option<DotDot>, Either<GenericArgs<'a>, TypePathFn<'a>>)>,
}

impl<'a> Parse<'a> for TypePathSegment<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			path_ident_segment: input.parse(),
			generic_args_or_type_path_fn: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			PathIdentSegment<'a>,
			Option<(Option<DotDot>, Either<GenericArgs<'a>, TypePathFn<'a>>)>,
		)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct TypePathFn<'a> {
	parentheses: Parentheses<'a, Option<TypePathFnInputs<'a>>>,
	return_type: Option<(RArrow, Type<'a>)>,
}

impl<'a> Parse<'a> for TypePathFn<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			parentheses: input.parse(),
			return_type: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(
			Parentheses<'a, Option<TypePathFnInputs<'a>>>,
			Option<(RArrow, Type<'a>)>,
		)>::describe(w)
	}
}

#[derive(Debug, Clone, Default)]
pub struct TypePathFnInputs<'a> {
	first_type: Box<Type<'a>>,
	rest: Vec<(Comma, Type<'a>)>,
	comma: Option<Comma>,
}

impl<'a> Parse<'a> for TypePathFnInputs<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			first_type: input.parse(),
			rest: input.parse(),
			comma: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Type, Vec<(Comma, Type)>, Option<Comma>)>::describe(w)
	}
}
