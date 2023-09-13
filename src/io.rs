use std::fmt::Write;

use either::Either;
use proc_macro2::TokenTree;
use this_is_fine::Fine;
use vec1::Vec1;

use crate::help::DiagnosticsList;

pub struct Input<'a> {
	pub cursor: &'a [TokenTree],
	pub diagnostics: &'a DiagnosticsList<'a>,
}

impl<'a> Input<'a> {
	pub fn error_expected<T: Parse<'a> + Default>(&mut self) -> T {
		todo!("If errors aren't suspended, create a diagnostic that evaluates with the given type's self-description.");
		T::default()
	}
}

pub trait Parse<'a>: Sized {
	fn parse(input: &mut Input<'a>) -> Self;

	fn try_parse(input: &mut Input<'a>) -> Fine<Self, DiagnosticsList<'a>> {
		todo!()
	}

	fn describe(w: &mut dyn Write);
}

impl<'a, T> Parse<'a> for Option<T>
where
	T: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		let (t, r) = input.try_parse();
		r.ok().map(|()| t)
	}
}

impl<'a, T> Parse<'a> for Box<T>
where
	T: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		Box::new(input.parse())
	}

	fn describe(w: &mut dyn Write) {
		T::describe(w)
	}
}

impl<'a, T> Parse<'a> for Vec<T>
where
	T: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		let mut vec = Vec::new();
		while let (t, Ok(())) = input.try_parse() {
			vec.push(t);
		}
		vec
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("any times ")?;
		T::describe(w)
	}
}

impl<'a, T> Parse<'a> for Vec1<T>
where
	T: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		let mut vec = Vec1::new(input.parse());
		while let (t, Ok(())) = input.try_parse() {
			vec.push(t);
		}
		vec
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("at least one time ")?;
		T::describe(w)
	}
}

impl<'a, L, R> Parse<'a> for Either<L, R>
where
	L: Parse<'a>,
	R: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		if let (l, Ok(())) = input.try_parse() {
			Self::Left(l)
		} else if let (r, Ok(())) = input.try_parse() {
			Self::Right(r)
		} else {
			// It might be good to give each Parse implementation a human() method that returns formatting information.
			todo!()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("( ")?;
		L::describe(w)?;
		w.write_str("|")?;
		R::describe(w)?;
		w.write_str(" )")
	}
}

impl<'a, T1> Parse<'a> for (T1,)
where
	T1: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		(input.parse(),)
	}

	fn describe(w: &mut dyn Write) {
		T1::describe(w)
	}
}
impl<'a, T1, T2> Parse<'a> for (T1, T2)
where
	T1: Parse<'a>,
	T2: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		(input.parse(), input.parse())
	}

	fn describe(w: &mut dyn Write) {
		T1::describe(w)?;
		w.write_str(" ")?;
		T2::describe(w)
	}
}
impl<'a, T1, T2, T3> Parse<'a> for (T1, T2, T3)
where
	T1: Parse<'a>,
	T2: Parse<'a>,
	T3: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		(input.parse(), input.parse(), input.parse())
	}

	fn describe(w: &mut dyn Write) {
		T1::describe(w)?;
		w.write_str(" ")?;
		T2::describe(w)?;
		w.write_str(" ")?;
		T3::describe(w)
	}
}
impl<'a, T1, T2, T3, T4> Parse<'a> for (T1, T2, T3, T4)
where
	T1: Parse<'a>,
	T2: Parse<'a>,
	T3: Parse<'a>,
	T4: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		(input.parse(), input.parse(), input.parse(), input.parse())
	}
	fn describe(w: &mut dyn Write) {
		T1::describe(w)?;
		w.write_str(" ")?;
		T2::describe(w)?;
		w.write_str(" ")?;
		T3::describe(w)?;
		w.write_str(" ")?;
		T4::describe(w)
	}
}
impl<'a, T1, T2, T3, T4, T5> Parse<'a> for (T1, T2, T3, T4, T5)
where
	T1: Parse<'a>,
	T2: Parse<'a>,
	T3: Parse<'a>,
	T4: Parse<'a>,
	T5: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		(
			input.parse(),
			input.parse(),
			input.parse(),
			input.parse(),
			input.parse(),
		)
	}
	fn describe(w: &mut dyn Write) {
		T1::describe(w)?;
		w.write_str(" ")?;
		T2::describe(w)?;
		w.write_str(" ")?;
		T3::describe(w)?;
		w.write_str(" ")?;
		T4::describe(w)?;
		w.write_str(" ")?;
		T5::describe(w)
	}
}
impl<'a, T1, T2, T3, T4, T5, T6> Parse<'a> for (T1, T2, T3, T4, T5, T6)
where
	T1: Parse<'a>,
	T2: Parse<'a>,
	T3: Parse<'a>,
	T4: Parse<'a>,
	T5: Parse<'a>,
	T6: Parse<'a>,
{
	fn parse(input: &mut Input<'a>) -> Self {
		(
			input.parse(),
			input.parse(),
			input.parse(),
			input.parse(),
			input.parse(),
			input.parse(),
		)
	}
	fn describe(w: &mut dyn Write) {
		T1::describe(w)?;
		w.write_str(" ")?;
		T2::describe(w)?;
		w.write_str(" ")?;
		T3::describe(w)?;
		w.write_str(" ")?;
		T4::describe(w)?;
		w.write_str(" ")?;
		T5::describe(w)?;
		w.write_str(" ")?;
		T6::describe(w)
	}
}

impl<'a> Input<'a> {
	pub fn parse<T: Parse<'a>>(&mut self) -> T {
		//TODO: Auto-descend into groups here.
		T::parse(self)
	}

	pub fn try_parse<T: Parse<'a>>(&mut self) -> Fine<T, DiagnosticsList<'a>> {
		T::try_parse(self)
	}

	pub fn consume(&mut self, token_tree_count: usize) -> &'a [TokenTree] {
		let (a, b) = self.cursor.split_at(token_tree_count);
		self.cursor = b;
		a
	}
}
