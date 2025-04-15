use std::boxed;

use loess::{
	grammar,
	rust_grammar::{
		AnyStringLiteral, As, Box, Colon, CurlyBraces, Dot, DotDot, Expression,
		ExpressionExceptStructExpression, For, Identifier, In, Parentheses, Pattern, SelfLowercase,
		Semi, SquareBrackets, Statement as RustStatement, Struct, Visibility,
	},
	Error, ErrorPriority, Errors, Input, PeekFrom, PopFrom,
};
use proc_macro2::TokenStream;

grammar! {
	pub enum Statement: PopFrom, IntoTokens {
		ParenBrace(Parentheses<CurlyBraces<Vec<RustStatement>>>),
		BracketBrace(SquareBrackets<CurlyBraces<Vec<RustStatement>>>),
		For(ForLoop),
		ParenFor(ParenForLoop),
		Block(CurlyBraces<Vec<Statement>>),
		Box(BoxStatement),
		Semi(Semi),
		Str(FormattedStr),
		Transclusion(Transclusion),
		Child(child::Child),
	} else "Expected Asteracea statement.";
}

mod child;

grammar! {
	pub struct ForLoop: PeekFrom, PopFrom, IntoTokens {
		// pub outer_attributes: Greedy<OuterAttribute>,
		pub r#for: For,
		pub pattern: Pattern,
		pub r#in: In,
		pub expression: ExpressionExceptStructExpression,
		pub block: CurlyBraces<Vec<Statement>>,
	}

	pub struct ParenForLoop: PeekFrom, PopFrom, IntoTokens {
		// pub outer_attributes: Greedy<OuterAttribute>,
		pub paren_for: Parentheses<For>,
		pub pattern: Pattern,
		pub r#in: In,
		pub expression: ExpressionExceptStructExpression,
		pub block: CurlyBraces<Vec<Statement>>,
	}

	pub struct BoxStatement: PeekFrom, PopFrom, IntoTokens {
		pub r#box: Box,
		pub storage: Option<Storage>,
		pub statement: boxed::Box<Statement>,
	}

	pub struct Storage: PeekFrom, IntoTokens {
		pub r#as: As,
		pub visibility: Option<Visibility>,
		pub self_: SelfLowercase,
		pub dot: Dot,
		pub identifier: Identifier,
		pub storage_type: Option<StorageType>,
	}

	pub struct StorageType: PeekFrom, PopFrom, IntoTokens {
		pub colon: Colon,
		pub r#struct: Option<Struct>,
		pub identifier: Identifier,
	}

	/// Note: `format!` is not emitted if literal doesn't contain curly braces
	///        and `format_args` is [`None`].
	pub struct FormattedStr: PeekFrom, PopFrom, IntoTokens {
		pub literal: AnyStringLiteral,
		pub format_args: Option<Parentheses>,
		pub semi: Semi,
	}

	pub struct Transclusion: PeekFrom, PopFrom, IntoTokens {
		pub dot_dot: DotDot,
		pub expression: Expression,
		pub semi: Semi,
	}
}

impl PopFrom for Storage {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		let storage = Self {
			r#as: As::pop_from(input, errors)?,
			visibility: Option::<Visibility>::pop_from(input, errors)?,
			self_: SelfLowercase::pop_from(input, errors)?,
			dot: Dot::pop_from(input, errors)?,
			identifier: Identifier::pop_from(input, errors)?,
			storage_type: Option::<StorageType>::pop_from(input, errors)?,
		};

		if !CurlyBraces::<TokenStream>::peek_from(input) && !Semi::peek_from(input) {
			errors.push(Error::new(
				ErrorPriority::GRAMMAR,
				"Storage must be followed by `{` or `;`.",
				[input.front_span()],
			))
		}

		Ok(storage)
	}
}
