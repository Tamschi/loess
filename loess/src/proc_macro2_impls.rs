use crate::{ErrorPriority, Errors};

use super::{Error, PopFrom, PopOrReplaceExt};
use proc_macro2::{Ident, TokenTree};
use std::collections::VecDeque;

impl PopFrom for Ident {
	fn pop_from(input: &mut VecDeque<TokenTree>, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		input
			.pop_or_replace(|t| match t {
				[TokenTree::Ident(ident)] => Ok(ident),
				t => Err(t),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::TOKEN, "Expected Ident.", spans))
			})
	}
}
