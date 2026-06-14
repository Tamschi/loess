//! [items.mod](https://doc.rust-lang.org/reference/items/modules.html#r-items.mod): Modules

use loess::{
	Input, PeekFrom, grammar,
	scaffold::{CurlyBraces, Greedy},
};
use proc_macro2::TokenTree;

use crate::{
	attributes::InnerAttribute,
	ident::Identifier,
	items::Item,
	lex::{
		keywords::{Mod, Unsafe},
		token::punct::Semi,
	},
};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Module](https://doc.rust-lang.org/reference/items/modules.html#grammar-Module)
	pub enum Module: PeekFrom via ModuleFlattened, PopFrom via ModuleFlattened, IntoTokens {
		UnsafeModIdentifierSemi(Option<Unsafe>, Mod, Identifier, Semi),
		/// Continue inside via [`ModuleContent`].
		UnsafeModIdentifierCurlyBraces(Option<Unsafe>, Mod, Identifier, CurlyBraces),
	} else "Expected Module.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// Flattened [`Module`].
	pub struct ModuleFlattened: PopFrom, IntoTokens {
		r#unsafe: Option<Unsafe>,
		r#mod: Mod,
		identifier: Identifier,
		variant: ModuleFlattenedVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`ModuleFlattened::variant`]
	pub enum ModuleFlattenedVariant: PeekFrom, PopFrom, IntoTokens {
		Semi(Semi),
		/// Continue inside via [`ModuleContent`].
		CurlyBraces(CurlyBraces),
	} else "Expected ModuleFlattenedVariant.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// Inside [`Module::UnsafeModIdentifierCurlyBraces`] or [`ModuleFlattenedVariant::CurlyBraces`].
	pub struct ModuleContent: PopFrom, IntoTokens {
		inner_attributes: Greedy<Vec<InnerAttribute>>,
		items: Vec<Item>,
	}
}

/// `mod` or `unsafe mod`
impl PeekFrom for ModuleFlattened {
	fn peek_from(input: &Input) -> bool {
		input.peek(|[tt], mut rest| match tt {
			TokenTree::Ident(ident) => {
				ident == "mod"
					|| ident == "unsafe"
						&& matches!(rest.next(), Some(TokenTree::Ident(ident)) if ident == "mod")
			}
			TokenTree::Group(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => false,
		})
	}
}

impl From<Module> for ModuleFlattened {
	fn from(value: Module) -> Self {
		use Module::*;
		use ModuleFlattenedVariant::*;
		match value {
			UnsafeModIdentifierSemi(r#unsafe, r#mod, identifier, semi) => Self {
				r#unsafe,
				r#mod,
				identifier,
				variant: Semi(semi),
			},
			UnsafeModIdentifierCurlyBraces(r#unsafe, r#mod, identifier, curly_braces) => Self {
				r#unsafe,
				r#mod,
				identifier,
				variant: CurlyBraces(curly_braces),
			},
		}
	}
}

impl From<ModuleFlattened> for Module {
	fn from(value: ModuleFlattened) -> Self {
		use Module::*;
		use ModuleFlattenedVariant::*;
		let ModuleFlattened {
			r#unsafe,
			r#mod,
			identifier,
			variant,
		} = value;
		match variant {
			Semi(semi) => UnsafeModIdentifierSemi(r#unsafe, r#mod, identifier, semi),
			CurlyBraces(curly_braces) => {
				UnsafeModIdentifierCurlyBraces(r#unsafe, r#mod, identifier, curly_braces)
			}
		}
	}
}
