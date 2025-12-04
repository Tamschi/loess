//! [lex.keywords](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords)
//!
//! Keywords are implemented as tuple structs with single public [`Ident`].

use loess::{lifetimes, words};
use proc_macro2::Ident;

pub(crate) mod words_impl {
	use loess::words;

	words! {
		// Strict keywords.
		// See <https://doc.rust-lang.org/stable/reference/keywords.html#strict-keywords> as of 2025-04-13.
		#[derive(Clone)] pub as as pub As: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub box as pub Box: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub break as pub Break: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub const as pub Const: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub continue as pub Continue: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub crate as pub Crate: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub else as pub Else: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub enum as pub Enum: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub extern as pub Extern: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub false as pub False: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub fn as pub Fn: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub for as pub For: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub if as pub If: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub impl as pub Impl: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub in as pub In: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub let as pub Let: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub loop as pub Loop: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub match as pub Match: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub mod as pub Mod: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub move as pub Move: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub mut as pub Mut: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub pub as pub Pub: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub ref as pub Ref: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub return as pub Return: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub self as pub SelfLowercase: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub Self as pub SelfUppercase: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub static as pub Static: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub struct as pub Struct: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub super as pub Super: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub trait as pub Trait: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub true as pub True: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub type as pub Type: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub unsafe as pub Unsafe: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub use as pub Use: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub where as pub Where: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub while as pub While: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

		// 2018 edition
		#[derive(Clone)] pub async as pub Async: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub await as pub Await: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
		#[derive(Clone)] pub dyn as pub Dyn: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

		// Reserved keywords.
		// See <https://doc.rust-lang.org/stable/reference/keywords.html#reserved-keywords> as of 2025-04-13.
		abstract as _;
		become as _;
		box as _;
		do as _;
		final as _;
		macro as _;
		override as _;
		priv as _;
		typeof as _;
		unsized as _;
		virtual as _;
		yield as _;

		// 2018+
		try as _;

		// 2024+
		gen as _;

		//TODO: Move this elsewhere in the API?
		/// [IDENTIFIER](https://doc.rust-lang.org/stable/reference/identifiers.html#r-ident.syntax)
		#[derive(Clone)] pub _ as pub Identifier: IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
	}
}
pub use words_impl::{
	As, Async, Await, Box, Break, Const, Continue, Crate, Dyn, Else, Enum, Extern, False, Fn, For,
	If, Impl, In, Let, Loop, Match, Mod, Move, Mut, Pub, Ref, Return, SelfLowercase, SelfUppercase,
	Static, Struct, Super, Trait, True, Type, Unsafe, Use, Where, While,
};

// Weak keywords.
// See <https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak>.
lifetimes! {
	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.lifetime-static)
	#[derive(Clone)] pub ('static) as pub LifetimeStatic: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
}
words! {

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.macro_rules)
	#[derive(Clone)] pub macro_rules as pub MacroRules: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.raw)
	#[derive(Clone)] pub raw as pub Raw: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.safe)
	#[derive(Clone)] pub safe as pub Safe: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.union)
	#[derive(Clone)] pub union as pub Union: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
}

/// See <https://doc.rust-lang.org/stable/reference/keywords.html#strict-keywords> as of 2025-04-13.
pub fn is_strict_keyword(ident: &Ident) -> bool {
	[
		"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
		"for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
		"return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
		"use", "where", "while", //
		// 2018 edition
		"async", "await", "dyn",
	]
	.iter()
	.any(|s| ident == s)
}

/// See <https://doc.rust-lang.org/stable/reference/keywords.html#reserved-keywords> as of 2025-04-13.
pub fn is_reserved_keyword(ident: &Ident) -> bool {
	[
		"abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof",
		"unsized", "virtual", "yield", //
		// 2018+
		"try", //
		// 2024+
		"gen",
	]
	.iter()
	.any(|s| ident == s)
}
