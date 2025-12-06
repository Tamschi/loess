//! [items](https://doc.rust-lang.org/reference/items.html#r-items): Items

use loess::{grammar, scaffold::Greedy};

use crate::{attributes::OuterAttribute, items::extern_crate::ExternCrate, vis::Visibility};

#[path ="items/extern_crate.rs"]
pub mod extern_crate;
#[path ="items/mod.rs"]
pub mod r#mod;

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Item](https://doc.rust-lang.org/reference/items.html#grammar-Item)
	pub struct Item: PeekFrom, PopFrom, IntoTokens {
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
	pub struct VisItem: PeekFrom, PopFrom, IntoTokens {
		visibility: Option<Visibility>,
		variant: VisItemVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`VisItem::variant`]
	pub enum VisItemVariant: PeekFrom, PopFrom, IntoTokens {
		Module(Module),
		ExternCrate(ExternCrate),
		UseDeclaration(UseDeclaration),
		Function(Function),
		TypeAlias(TypeAlias),
		Struct(Struct),
		Enumeration(Enumeration),
		Union(Union),
		ConstantItem(ConstantItem),
		StaticItem(StaticItem),
		Trait(Trait),
		Implementation(Implementation),
		ExternBlock(ExternBlock),
	} else "Expected VisItem or MacroItem.";

	#[derive(Clone)]
	#[non_exhaustive]
	/// [MacroItem](https://doc.rust-lang.org/reference/items.html#grammar-MacroItem)
	pub enum MacroItem: PeekFrom, PopFrom, IntoTokens {
		MacroInvocationSemi(MacroInvocationSemi),
		MacroRulesDefinition(MacroRulesDefinition),
	} else "Expected VisItem or MacroItem.";
}
