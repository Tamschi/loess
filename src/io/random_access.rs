use std::borrow::Cow;

use proc_macro2::{extra::DelimSpan, Delimiter, Ident, Literal, Punct};

/// Copy of [`proc_macro2::TokenTree`], except using [`Group`].
pub enum TokenTree<'a> {
	Group(Box<Group<'a>>),
	Ident(Ident),
	Punct(Punct),
	Literal(Literal),
}

impl From<proc_macro2::TokenTree> for TokenTree<'_> {
	fn from(value: proc_macro2::TokenTree) -> Self {
		match value {
			proc_macro2::TokenTree::Group(group) => Self::Group(group.into()),
			proc_macro2::TokenTree::Ident(ident) => Self::Ident(ident),
			proc_macro2::TokenTree::Punct(punct) => Self::Punct(punct),
			proc_macro2::TokenTree::Literal(literal) => Self::Literal(literal),
		}
	}
}

impl From<TokenTree<'_>> for proc_macro2::TokenTree {
	fn from(value: TokenTree) -> Self {
		match value {
			TokenTree::Group(group) => proc_macro2::TokenTree::Group(group.into()),
			TokenTree::Ident(ident) => proc_macro2::TokenTree::Ident(ident),
			TokenTree::Punct(punct) => proc_macro2::TokenTree::Punct(punct),
			TokenTree::Literal(literal) => proc_macro2::TokenTree::Literal(literal),
		}
	}
}

/// Copy of [`Group`], except the contents are a slice.
pub struct Group<'a> {
	pub delimiter: Delimiter,
	pub delim_span: DelimSpan,
	pub contents: Cow<'a, [TokenTree<'a>]>,
}

impl From<proc_macro2::Group> for Group<'_> {
	fn from(value: proc_macro2::Group) -> Self {
		Self {
			delimiter: value.delimiter(),
			delim_span: value.delim_span(),
			contents: value.stream().into_iter().map(Into::into).collect(),
		}
	}
}

impl From<Group<'_>> for proc_macro2::Group {
	fn from(value: Group) -> Self {
		let mut group =
			proc_macro2::Group::new(value.delimiter, value.contents.into_iter().collect());

		// There seems to be no way to set the delimiter spans separately.
		group.set_span(value.delim_span.join());

		group
	}
}
