use std::fmt::Write;

use either::Either;

use crate::{
	io::{Input, Parse},
	tokens::{
		keywords::{As, Mut},
		punctuation::{
			And, AndAnd, AndEq, Caret, CaretEq, Eq, EqEq, Ge, Gt, Le, Lt, Minus, MinusEq, Ne, Not,
			Or, OrEq, OrOr, Percent, PercentEq, Plus, PlusEq, Question, Shl, ShlEq, Shr, ShrEq,
			Slash, SlashEq, Star, StarEq,
		},
	}, types::TypeNoBounds,
};

use super::Expression;

pub enum OperatorExpression<'a> {
	BorrowExpression(BorrowExpression<'a>),
	DereferenceExpression(DereferenceExpression<'a>),
	ErrorPropagationExpression(ErrorPropagationExpression<'a>),
	NegationExpression(NegationExpression<'a>),
	ArithmeticOrLogicalExpression(ArithmeticOrLogicalExpression<'a>),
	ComparisonExpression(ComparisonExpression<'a>),
	LazyBooleanExpression(LazyBooleanExpression<'a>),
	TypeCastExpression(TypeCastExpression<'a>),
	AssignmentExpression(AssignmentExpression<'a>),
	CompoundAssignmentExpression(CompoundAssignmentExpression<'a>),
}

impl<'a> Parse<'a> for OperatorExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		//TODO: snake_case
		if let (BorrowExpression, Ok(())) = input.try_parse() {
			Self::BorrowExpression(BorrowExpression)
		} else if let (DereferenceExpression, Ok(())) = input.try_parse() {
			Self::DereferenceExpression(DereferenceExpression)
		} else if let (ErrorPropagationExpression, Ok(())) = input.try_parse() {
			Self::ErrorPropagationExpression(ErrorPropagationExpression)
		} else if let (NegationExpression, Ok(())) = input.try_parse() {
			Self::NegationExpression(NegationExpression)
		} else if let (ArithmeticOrLogicalExpression, Ok(())) = input.try_parse() {
			Self::ArithmeticOrLogicalExpression(ArithmeticOrLogicalExpression)
		} else if let (ComparisonExpression, Ok(())) = input.try_parse() {
			Self::ComparisonExpression(ComparisonExpression)
		} else if let (LazyBooleanExpression, Ok(())) = input.try_parse() {
			Self::LazyBooleanExpression(LazyBooleanExpression)
		} else if let (TypeCastExpression, Ok(())) = input.try_parse() {
			Self::TypeCastExpression(TypeCastExpression)
		} else if let (AssignmentExpression, Ok(())) = input.try_parse() {
			Self::AssignmentExpression(AssignmentExpression)
		} else if let (CompoundAssignmentExpression, Ok(())) = input.try_parse() {
			Self::CompoundAssignmentExpression(CompoundAssignmentExpression)
		} else {
			todo!()
		}
	}
}

pub struct BorrowExpression<'a> {
	pub op: Either<And, AndAnd>,
	pub r#mut: Option<Mut>,
	pub expression: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for BorrowExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			op: input.parse(),
			r#mut: input.parse(),
			expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Either<And, AndAnd>, Option<Mut>, Expression)>::describe(w)
	}
}

pub struct DereferenceExpression<'a> {
	pub star: Star,
	pub expression: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for DereferenceExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			star: input.parse(),
			expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Star, Expression)>::describe(w)
	}
}

pub struct ErrorPropagationExpression<'a> {
	pub expression: Box<Expression<'a>>,
	pub question: Question,
}

impl<'a> Parse<'a> for ErrorPropagationExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			expression: input.parse(),
			question: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, Question)>::describe(w)
	}
}

pub struct NegationExpression<'a> {
	pub op: Either<Minus, Not>,
	pub expression: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for NegationExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			op: input.parse(),
			expression: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Either<Minus, Not>, Expression)>::describe(w)
	}
}

pub struct ArithmeticOrLogicalExpression<'a> {
	left: Box<Expression<'a>>,
	op: ArithmeticOrLogicalOp,
	right: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for ArithmeticOrLogicalExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			left: input.parse(),
			op: input.parse(),
			right: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, ArithmeticOrLogicalOp, Expression)>::describe(w)
	}
}

pub enum ArithmeticOrLogicalOp {
	Add(Plus),
	Sub(Minus),
	Mul(Star),
	Div(Slash),
	Rem(Percent),
	BitAnd(And),
	BitOr(Or),
	BitXor(Caret),
	Shl(Shl),
	Shr(Shr),
}

impl Parse<'_> for ArithmeticOrLogicalOp {
	fn parse(input: &mut Input<'_>) -> Self {
		if let (plus, Ok(())) = input.try_parse() {
			Self::Add(plus)
		} else if let (minus, Ok(())) = input.try_parse() {
			Self::Sub(minus)
		} else if let (star, Ok(())) = input.try_parse() {
			Self::Mul(star)
		} else if let (slash, Ok(())) = input.try_parse() {
			Self::Div(slash)
		} else if let (percent, Ok(())) = input.try_parse() {
			Self::Rem(percent)
		} else if let (and, Ok(())) = input.try_parse() {
			Self::BitAnd(and)
		} else if let (or, Ok(())) = input.try_parse() {
			Self::BitOr(or)
		} else if let (caret, Ok(())) = input.try_parse() {
			Self::BitXor(caret)
		} else if let (shl, Ok(())) = input.try_parse() {
			Self::Shl(shl)
		} else if let (shr, Ok(())) = input.try_parse() {
			Self::Shr(shr)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("([+-*/%&|^]|`<<`|`>>`)")
	}
}

impl Default for ArithmeticOrLogicalOp {
	fn default() -> Self {
		Self::Add(Plus::default())
	}
}

pub struct ComparisonExpression<'a> {
	left: Box<Expression<'a>>,
	op: ComparisonOp,
	right: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for ComparisonExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			left: input.parse(),
			op: input.parse(),
			right: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, ComparisonOp, Expression)>::describe(w)
	}
}

pub enum ComparisonOp {
	Eq(EqEq),
	Ne(Ne),
	Gt(Gt),
	Lt(Lt),
	Ge(Ge),
	Le(Le),
}

impl Parse<'_> for ComparisonOp {
	fn parse(input: &mut Input<'_>) -> Self {
		if let (eq_eq, Ok(())) = input.try_parse() {
			Self::Eq(eq_eq)
		} else if let (ne, Ok(())) = input.try_parse() {
			Self::Ne(ne)
		} else if let (gt, Ok(())) = input.try_parse() {
			Self::Gt(gt)
		} else if let (lt, Ok(())) = input.try_parse() {
			Self::Lt(lt)
		} else if let (ge, Ok(())) = input.try_parse() {
			Self::Ge(ge)
		} else if let (le, Ok(())) = input.try_parse() {
			Self::Le(le)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("(`==`|`!=`|`>`|`<`|`>=`|`<=`)")
	}
}

impl Default for ComparisonOp {
	fn default() -> Self {
		Self::Eq(EqEq::default())
	}
}

pub struct LazyBooleanExpression<'a> {
	left: Box<Expression<'a>>,
	op: LazyBooleanOp,
	right: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for LazyBooleanExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			left: input.parse(),
			op: input.parse(),
			right: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, LazyBooleanOp, Expression)>::describe(w)
	}
}

pub enum LazyBooleanOp {
	OrElse(OrOr),
	AndThen(AndAnd),
}

impl Parse<'_> for LazyBooleanOp {
	fn parse(input: &mut Input<'_>) -> Self {
		if let (or_or, Ok(())) = input.try_parse() {
			Self::OrElse(or_or)
		} else if let (and_and, Ok(())) = input.try_parse() {
			Self::AndThen(and_and)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("(`&&`|`||`)")
	}
}

impl Default for LazyBooleanOp {
	fn default() -> Self {
		Self::Eq(EqEq::default())
	}
}

pub struct TypeCastExpression<'a> {
	expression: Box<Expression<'a>>,
	r#as: As,
	type_no_bounds: TypeNoBounds<'a>,
}

impl<'a> Parse<'a> for TypeCastExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			expression: input.parse(),
			r#as: input.parse(),
			type_no_bounds: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, As, TypeNoBounds)>::describe(w)
	}
}

pub struct AssignmentExpression<'a> {
	left: Box<Expression<'a>>,
	eq: Eq,
	right: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for AssignmentExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			left: input.parse(),
			eq: input.parse(),
			right: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, Eq, TypeNoBounds)>::describe(w)
	}
}

pub struct CompoundAssignmentExpression<'a> {
	left: Box<Expression<'a>>,
	op: CompoundAssignmentOp,
	right: Box<Expression<'a>>,
}

impl<'a> Parse<'a> for CompoundAssignmentExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			left: input.parse(),
			op: input.parse(),
			right: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		<(Expression, CompoundAssignmentOp, Expression)>::describe(w)
	}
}

pub enum CompoundAssignmentOp {
	AddAssign(PlusEq),
	SubAssign(MinusEq),
	MulAssign(StarEq),
	DivAssign(SlashEq),
	RemAssign(PercentEq),
	BitAndAssign(AndEq),
	BitOrAssign(OrEq),
	BitXorAssign(CaretEq),
	ShlAssign(ShlEq),
	ShrAssign(ShrEq),
}

impl Parse<'_> for CompoundAssignmentOp {
	fn parse(input: &mut Input<'_>) -> Self {
		if let (plus_eq, Ok(())) = input.try_parse() {
			Self::AddAssign(plus_eq)
		} else if let (minus_eq, Ok(())) = input.try_parse() {
			Self::SubAssign(minus_eq)
		} else if let (star_eq, Ok(())) = input.try_parse() {
			Self::MulAssign(star_eq)
		} else if let (slash_eq, Ok(())) = input.try_parse() {
			Self::DivAssign(slash_eq)
		} else if let (percent_eq, Ok(())) = input.try_parse() {
			Self::RemAssign(percent_eq)
		} else if let (and_eq, Ok(())) = input.try_parse() {
			Self::BitAndAssign(and_eq)
		} else if let (or_eq, Ok(())) = input.try_parse() {
			Self::BitOrAssign(or_eq)
		} else if let (caret_eq, Ok(())) = input.try_parse() {
			Self::BitXorAssign(caret_eq)
		} else if let (shl_eq, Ok(())) = input.try_parse() {
			Self::ShlAssign(shl_eq)
		} else if let (shr_eq, Ok(())) = input.try_parse() {
			Self::ShrAssign(shr_eq)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("([+-*/%&|^]|`<<`|`>>`)")
	}
}

impl Default for CompoundAssignmentOp {
	fn default() -> Self {
		Self::AddAssign(Plus::default())
	}
}
