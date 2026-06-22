use std::ops::ControlFlow::{self, Break, Continue};

use crate::{IntoTokens, PeekFrom, PopParsedFrom};

impl<T> IntoTokens for (T,)
where
	T: IntoTokens,
{
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.0.into_tokens(root, tokens)
	}
}

impl<T1, T2> IntoTokens for (T1, T2)
where
	T1: IntoTokens,
	T2: IntoTokens,
{
	fn into_tokens(
		self,
		root: &proc_macro2::TokenStream,
		tokens: &mut impl Extend<proc_macro2::TokenTree>,
	) {
		self.0.into_tokens(root, tokens);
		self.1.into_tokens(root, tokens);
	}
}

//TODO
