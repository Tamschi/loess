#![allow(deprecated)]

use super::{Keyword, KeywordString};

macro_rules! keywords {
	($($type_name:ident $enum_name:ident $keyword:literal),*$(,)?) => {$(
		#[deprecated = "Please don't use this directly."]
		pub enum $enum_name {}
		impl KeywordString for $enum_name {
			const KW: &'static str = $keyword;
		}
		pub type $type_name = Keyword<$enum_name>;
	)*};
}

keywords! {
	// strict:
	As AS "as",
	Break BREAK "break",
	Const CONST "const",
	Continue CONTINUE "continue",
	Crate CRATE "crate",
	Else ELSE "else",
	Enum ENUM "enum",
	Extern EXTERN "extern",
	False FALSE "false",
	Fn FN "fn",
	For FOR "for",
	If IF "if",
	Impl IMPL "impl",
	In IN "in",
	Let LET "let",
	Loop LOOP "loop",
	Match MATCH "match",
	Mod MOD "mod",
	Move MOVE "move",
	Mut MUT "mut",
	Pub PUB "pub",
	Ref REF "ref",
	Return RETURN "return",
	Selfvalue SELFVALUE "self",
	Selftype SELFTYPE "Self",
	Static STATIC "static",
	Struct STRUCT "struct",
	Super SUPER "super",
	Trait TRAIT "trait",
	True TRUE "true",
	Type TYPE "type",
	Unsafe UNSAFE "unsafe",
	Use USE "use",
	Where WHERE "where",
	While WHILE "while",

	// strict (2018+):
	Async ASYNC "async",
	Await AWAIT "await",
	Dyn DYN "dyn",

	// reserved:
	Abstract ABSTRACT "abstract",
	Become BECOME "become",
	Box BOX "box",
	Do DO "do",
	Final FINAL "final",
	Macro MACRO "macro",
	Override OVERRIDE "override",
	Priv PRIV "priv",
	Typeof TYPEOF "typeof",
	Unsized UNSIZED "unsized",
	Virtual VIRTUAL "virtual",
	Yield YIELD "yield",

	// reserved (2018+):
	Try TRY "try",
}
