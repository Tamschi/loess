//! [items](https://doc.rust-lang.org/reference/items.html#r-items): Items

use loess::{PeekFrom, grammar, scaffold::Greedy};

use crate::{
	attributes::OuterAttribute, items::extern_crate::ExternCrate,
	r#macro::invocation::MacroInvocationSemi, vis::Visibility,
};

#[path = "items/extern_crate.rs"]
pub mod extern_crate;
#[path = "items/mod.rs"]
pub mod r#mod;

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Item](https://doc.rust-lang.org/reference/items.html#grammar-Item)
	pub struct Item: PopFrom, IntoTokens {
		outer_attributes: Greedy<Vec<OuterAttribute>>,
		variant: ItemVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`Item::variant`]
	pub enum ItemVariant: PeekFrom, PopFrom, IntoTokens {
		VisItem(VisItem),
		MacroItem(MacroItem),
	} else "Expected VisItem or MacroItem.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [VisItem](https://doc.rust-lang.org/reference/items.html#grammar-VisItem)
	pub struct VisItem: PopFrom, IntoTokens {
		visibility: Option<Visibility>,
		variant: VisItemVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`VisItem::variant`]
	pub enum VisItemVariant: PeekFrom, PopFrom, IntoTokens {
		// Module(Module),
		ExternCrate(ExternCrate),
		// UseDeclaration(UseDeclaration),
		// Function(Function),
		// TypeAlias(TypeAlias),
		// Struct(Struct),
		// Enumeration(Enumeration),
		// Union(Union),
		// ConstantItem(ConstantItem),
		// StaticItem(StaticItem),
		// Trait(Trait),
		// Implementation(Implementation),
		// ExternBlock(ExternBlock),
	} else "Expected VisItem or MacroItem.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [MacroItem](https://doc.rust-lang.org/reference/items.html#grammar-MacroItem)
	pub enum MacroItem: PeekFrom, PopFrom, IntoTokens {
		MacroInvocationSemi(MacroInvocationSemi),
		// MacroRulesDefinition(MacroRulesDefinition),
	} else "Expected VisItem or MacroItem.";
}

impl PeekFrom for Item {
	fn peek_from(input: &loess::Input) -> bool {
		todo!("Item::peek_from")
	}
}

impl PeekFrom for VisItem {
	fn peek_from(input: &loess::Input) -> bool {
		Visibility::peek_from(input) || VisItemVariant::peek_from(input)
	}
}
