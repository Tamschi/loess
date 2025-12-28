use std::{any::type_name, convert::Infallible, marker::PhantomData};

use crate::{PeekFrom, PopParsedFrom};

/// A (generally thread-local) scope that can influence nested parsing.
///
/// Use [`In`] or [`InUnpeeked`] to check for such scopes.
pub trait Scope {
	/// The type wrapped by the scope, usually the only type argument.
	type Wrapped: ?Sized;

	/// Whether currently in the scope.
	fn is_in() -> bool;
}

/// Checks whether in scope `S`. Panics if parsed when not!
pub enum In<S: ?Sized + Scope> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<S>, Infallible),
}

impl<S: ?Sized + Scope> PeekFrom for In<S>
where
	S::Wrapped: PeekFrom,
{
	fn peek_from(input: &crate::Input) -> bool {
		S::is_in() && S::Wrapped::peek_from(input)
	}
}

/// Does *not* (re-)enter the wrapped scope!
///
/// # Panics
///
/// Iff [`pop_parsed_from`](`PopParsedFrom::pop_parsed_from`) is called outside the assessed scope.
impl<S: ?Sized + Scope> PopParsedFrom for In<S>
where
	S::Wrapped: PopParsedFrom,
{
	type Parsed = <S::Wrapped as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut crate::Input,
		errors: &mut crate::Errors,
	) -> Result<Self::Parsed, Option<Self::Parsed>> {
		assert!(
			S::is_in(),
			"Expected to be in scope `{}`.",
			type_name::<S>()
		);
		S::Wrapped::pop_parsed_from(input, errors)
	}
}

/// Checks whether in scope `S` (without peeking inside). Panics if parsed when not!
pub enum InUnpeeked<S: ?Sized + Scope> {
	#[expect(missing_docs)]
	_Vacant(PhantomData<S>, Infallible),
}

impl<S: ?Sized + Scope> PeekFrom for InUnpeeked<S> {
	fn peek_from(_input: &crate::Input) -> bool {
		S::is_in()
	}
}

/// Does *not* (re-)enter the wrapped scope!
///
/// # Panics
///
/// Iff [`pop_parsed_from`](`PopParsedFrom::pop_parsed_from`) is called outside the assessed scope.
impl<S: ?Sized + Scope> PopParsedFrom for InUnpeeked<S>
where
	S::Wrapped: PopParsedFrom,
{
	type Parsed = <S::Wrapped as PopParsedFrom>::Parsed;

	fn pop_parsed_from(
		input: &mut crate::Input,
		errors: &mut crate::Errors,
	) -> Result<Self::Parsed, Option<Self::Parsed>> {
		assert!(
			S::is_in(),
			"Expected to be in scope `{}`.",
			type_name::<S>()
		);
		S::Wrapped::pop_parsed_from(input, errors)
	}
}
