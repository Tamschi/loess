use crate::{
	io::{Input, Parse},
	names::paths::{PathInExpression, QualifiedPathInExpression},
};

pub enum PathExpression<'a> {
	PathInExpression(PathInExpression<'a>),
	QualifiedPathInExpression(QualifiedPathInExpression<'a>),
}

impl<'a> Parse<'a> for PathExpression<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (pie, Ok(())) = input.try_parse() {
			Self::PathInExpression(pie)
		} else if let (qpie, Ok(())) = input.try_parse() {
			Self::QualifiedPathInExpression(qpie)
		} else {
			todo!()
		}
	}
}
