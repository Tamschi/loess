//! [lex.token](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token>)
//!
//! Keywords and identifiers are implemented as tuple structs with single public [`Ident`].
//!
//! Punctuation is implemented as structs with named [`Punct`] fields.  
//! Where collisions would happen, they have a 0-based suffix.

use loess::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, PopFrom, PopParsedFrom,
	SimpleSpanned, punctuation, words,
};
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

// See <https://doc.rust-lang.org/stable/reference/tokens.html#punctuation> as of 2025-12-03.
punctuation! {
	#[derive(Clone)]
	(|) not before [| =] as pub Or: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt {
		pub or,
	}

	#[derive(Clone)]
	(.) not before [.] as pub Dot: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt {
		pub dot,
	}

	#[derive(Clone)]
	(..) not before [. =] as pub DotDot: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt {
		pub dot0,
		pub dot1,
	}

	#[derive(Clone)]
	(->) as pub RArrow: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt {
		pub minus,
		pub gt,
	}

	#[derive(Clone)]
	(,) as pub Comma: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt {
		pub comma,
	}

	#[derive(Clone)]
	(;) as pub Semi: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt {
		pub semi,
	}

	#[derive(Clone)]
	(:) not before [:] as pub Colon: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt {
		pub colon,
	}
}

// `|`
impl Default for Or {
	fn default() -> Self {
		Self {
			or: Punct::new('|', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Or {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(or)] if or.as_char() == '|' && or.spacing() == Spacing::Alone => {
					Ok(Self { or })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `|`.", spans))
			})
	}
}

impl IntoTokens for Or {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.or.into_tokens(root, tokens)
	}
}

// `.`
impl Default for Dot {
	fn default() -> Self {
		Self {
			dot: Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Dot {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(dot)]
					if dot.as_char() == '.' && dot.spacing() == Spacing::Alone =>
				{
					Ok(Self { dot })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `.`.", spans))
			})
	}
}

impl IntoTokens for Dot {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.dot.into_tokens(root, tokens)
	}
}

// `..`
impl Default for DotDot {
	fn default() -> Self {
		Self {
			dot0: Punct::new('.', Spacing::Joint).with_span(Span::mixed_site()),
			dot1: Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for DotDot {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, rest| match tts {
				[TokenTree::Punct(dot0), TokenTree::Punct(dot1)]
					if dot0.as_char() == '.'
						&& dot0.spacing() == Spacing::Joint
						&& dot1.as_char() == '.'
						&& (dot1.spacing() == Spacing::Alone || !matches!(rest.front(), Some(TokenTree::Punct(next_punct)) if matches!(next_punct.as_char(), '.' | '=')))
					=> { Ok(Self{dot0, dot1}) }
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `..`.", spans))
			})
	}
}

impl IntoTokens for DotDot {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		let Self { dot0, dot1 } = self;
		tokens.extend([dot0.into(), dot1.into()])
	}
}

// `->`
impl Default for RArrow {
	fn default() -> Self {
		Self {
			minus: Punct::new('-', Spacing::Joint).with_span(Span::mixed_site()),
			gt: Punct::new('>', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for RArrow {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(minus), TokenTree::Punct(gt)]
					if minus.as_char() == '-'
						&& minus.spacing() == Spacing::Joint
						&& gt.as_char() == '>' =>
				{
					Ok(Self { minus, gt })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `->`.", spans))
			})
	}
}

impl IntoTokens for RArrow {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		let Self { minus, gt } = self;
		tokens.extend([minus.into(), gt.into()])
	}
}

// `,`
impl Default for Comma {
	fn default() -> Self {
		Self {
			comma: Punct::new(',', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Comma {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(comma)] if comma.as_char() == ',' => Ok(Self { comma }),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `,`.", spans))
			})
	}
}

impl IntoTokens for Comma {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.comma.into_tokens(root, tokens)
	}
}

// `;`
impl Default for Semi {
	fn default() -> Self {
		Self {
			semi: Punct::new(';', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Semi {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(semi)] if semi.as_char() == ';' => Ok(Self { semi }),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `;`.", spans))
			})
	}
}

impl IntoTokens for Semi {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.semi.into_tokens(root, tokens)
	}
}

// `:`
impl Default for Colon {
	fn default() -> Self {
		Self {
			colon: Punct::new(':', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Colon {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(colon)]
					if colon.as_char() == ':' && colon.spacing() == Spacing::Alone =>
				{
					Ok(Self { colon })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `:`.", spans))
			})
	}
}

impl IntoTokens for Colon {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.colon.into_tokens(root, tokens)
	}
}

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

	/// [IDENTIFIER](https://doc.rust-lang.org/stable/reference/identifiers.html#r-ident.syntax)
	#[derive(Clone)] pub _ as pub Identifier: IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
}

impl PeekFrom for Identifier {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Ident(ident))
				if !(["r#crate", "r#self", "r#super", "r#Self"]
					.into_iter()
					.any(|s| ident == s)
					|| is_strict_keyword(&ident)
					|| is_reserved_keyword(&ident)),
		)
	}
}

// Weak keywords.
// See <https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak>.
words! {
	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.lifetime-static)
	#[derive(Clone)] pub ('static) as pub LifetimeStatic: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.macro_rules)
	#[derive(Clone)] pub macro_rules as pub MacroRules: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.raw)
	#[derive(Clone)] pub raw as pub Raw: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.safe)
	#[derive(Clone)] pub safe as pub Safe: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;

	/// [(weak)](https://doc.rust-lang.org/stable/reference/keywords.html#r-lex.keywords.weak.union)
	#[derive(Clone)] pub union as pub Union: doc, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt;
}

/// See <https://doc.rust-lang.org/stable/reference/identifiers.html?highlight=IDENTIFIER#identifiers> as of 2025-04-13.
impl PopParsedFrom for Identifier {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		let ident = Ident::peek_pop_from(input, errors)?;

		match ident {
			Some(ident)
				if !(["r#crate", "r#self", "r#super", "r#Self"]
					.into_iter()
					.any(|s| ident == s)
					|| is_strict_keyword(&ident)
					|| is_reserved_keyword(&ident)) =>
			{
				Ok(Self(ident))
			}
			ident => Err(if let Some(ident) = ident {
				errors.push(Error::new(
					ErrorPriority::GRAMMAR,
					if ident.to_string().starts_with("r#") {
						format!(
							"Expected Identifier. (`{}` cannot be a raw identifier.)",
							&ident.to_string()[2..]
						)
					} else {
						format!("Expected Identifier. (`{ident}` is a keyword.)")
					},
					[ident.span()],
				));

				input.push_front(TokenTree::Ident(ident));
			} else {
				errors.push(Error::new(
					ErrorPriority::GRAMMAR,
					"Expected Identifier.",
					[input.front_span()],
				));
			}),
		}
	}
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
