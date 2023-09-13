use std::fmt::Write;

use crate::io::{Input, Parse};

use self::never_type::NeverType;

pub mod never_type;

pub enum Type<'a> {
	TypeNoBounds(TypeNoBounds<'a>),
	ImplTraitType(ImplTraitType<'a>),
	TraitObjectType(TraitObjectType<'a>),
}

impl<'a> Parse<'a> for Type<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (type_no_bounds, Ok(())) = input.try_parse() {
			Self::TypeNoBounds(type_no_bounds)
		} else if let (impl_trait_type, Ok(())) = input.try_parse() {
			Self::ImplTraitType(impl_trait_type)
		} else if let (trait_object_type, Ok(())) = input.try_parse() {
			Self::TraitObjectType(trait_object_type)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("Type")
	}
}

pub enum TypeNoBounds<'a> {
	ParenthesizedOrTupleType(ParenthesizedOrTupleType<'a>),
	ImplTraitTypeOneBound(ImplTraitTypeOneBound<'a>),
	TraitObjectTypeOneBound(TraitObjectTypeOneBound<'a>),
	TypePath(TypePath<'a>),
	NeverType(NeverType),
	RawPointerType(RawPointerType<'a>),
	ReferenceType(ReferenceType<'a>),
	ArrayType(ArrayType<'a>),
	SliceType(SliceType<'a>),
	InferredType(InferredType<'a>),
	QualifiedPathInType(QualifiedPathInType<'a>),
	BareFunctionType(BareFunctionType<'a>),
	MacroInvocation(MacroInvocation<'a>),
}

impl<'a> Parse<'a> for TypeNoBounds<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		if let (ParenthesizedOrTupleType, Ok(())) = input.try_parse() {
			Self::ParenthesizedOrTupleType(ParenthesizedOrTupleType)
		} else if let (ImplTraitTypeOneBound, Ok(())) = input.try_parse() {
			Self::ImplTraitTypeOneBound(ImplTraitTypeOneBound)
		} else if let (TraitObjectTypeOneBound, Ok(())) = input.try_parse() {
			Self::TraitObjectTypeOneBound(TraitObjectTypeOneBound)
		} else if let (TypePath, Ok(())) = input.try_parse() {
			Self::TypePath(TypePath)
		} else if let (NeverType, Ok(())) = input.try_parse() {
			Self::NeverType(NeverType)
		} else if let (RawPointerType, Ok(())) = input.try_parse() {
			Self::RawPointerType(RawPointerType)
		} else if let (ReferenceType, Ok(())) = input.try_parse() {
			Self::ReferenceType(ReferenceType)
		} else if let (ArrayType, Ok(())) = input.try_parse() {
			Self::ArrayType(ArrayType)
		} else if let (SliceType, Ok(())) = input.try_parse() {
			Self::SliceType(SliceType)
		} else if let (InferredType, Ok(())) = input.try_parse() {
			Self::InferredType(InferredType)
		} else if let (QualifiedPathInType, Ok(())) = input.try_parse() {
			Self::QualifiedPathInType(QualifiedPathInType)
		} else if let (BareFunctionType, Ok(())) = input.try_parse() {
			Self::BareFunctionType(BareFunctionType)
		} else if let (MacroInvocation, Ok(())) = input.try_parse() {
			Self::MacroInvocation(MacroInvocation)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn Write) {
		w.write_str("TypeNoBounds")
	}
}

impl Default for TypeNoBounds<'_> {
	fn default() -> Self {
		Self::NeverType(NeverType::default())
	}
}

pub struct ParenthesizedOrTupleType<'a> {
	parens: Parens<'a>,
}

impl<'a> Parse<'a> for ParenthesizedOrTupleType<'a> {
	fn parse(input: &mut Input<'a>) -> Self {
		Self {
			parens: input.parse(),
		}
	}

	fn describe(w: &mut dyn Write) {
		Parens::describe(w)
	}
}
