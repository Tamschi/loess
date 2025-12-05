//! [items](https://doc.rust-lang.org/reference/items.html#r-items): Items

use loess::{grammar, scaffold::Greedy};

use crate::{attributes::OuterAttribute, vis::Visibility};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Item](https://doc.rust-lang.org/reference/items.html?highlight=Item#r-items.syntax)
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
	/// [VisItem](https://doc.rust-lang.org/reference/items.html?highlight=VisItem#r-items.syntax)
	pub struct VisItem: PeekFrom, PopFrom, IntoTokens {
		visibility: Option<Visibility>,
		variant: VisItemVariant,
	}

	#[derive(Clone)]
	#[non_exhaustive]
	/// [`Item::variant`]
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
	/// [MacroItem](https://doc.rust-lang.org/reference/items.html?highlight=MacroItem#r-items.syntax)
	pub enum MacroItem: PeekFrom, PopFrom, IntoTokens {
		MacroInvocationSemi(MacroInvocationSemi),
		MacroRulesDefinition(MacroRulesDefinition),
	} else "Expected VisItem or MacroItem.";
}
