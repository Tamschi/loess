//! [lex.token.punct](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token.punct): Punctuation
//!
//! Punctuation is implemented as structs with named [`Punct`] fields.  
//! Where collisions would happen, they have a 0-based suffix.

use loess::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PopParsedFrom, SimpleSpanned, punctuation,
};
use proc_macro2::{Punct, Spacing, Span, TokenStream, TokenTree};

// See <https://doc.rust-lang.org/stable/reference/tokens.html#punctuation> as of 2025-12-03.
punctuation! {
	#[derive(Clone)] (+) not before [=] as pub Plus: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub plus }
	#[derive(Clone)] (-) not before [= >] as pub Minus: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub minus }
	#[derive(Clone)] (*) not before [=] as pub Star: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub star }
	#[derive(Clone)] (/) not before [=] as pub Slash: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub slash }
	#[derive(Clone)] (%) not before [=] as pub Percent: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub percent }
	#[derive(Clone)] (^) not before [=] as pub Caret: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub caret }
	#[derive(Clone)] (!) not before [=] as pub Not: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub not }
	#[derive(Clone)] (&) not before [& =] as pub And: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub and }
	#[derive(Clone)] (|) not before [| =] as pub Or: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub or }
	#[derive(Clone)] (&&) as pub AndAnd: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub and0, pub and1 }
	#[derive(Clone)] (||) as pub OrOr: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub or0, pub or1 }
	#[derive(Clone)] (<<) not before [=] as pub Shl: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub lt0, pub lt1 }
	#[derive(Clone)] (>>) not before [=] as pub Shr: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub gt0, pub gt1 }
	#[derive(Clone)] (+=) as pub PlusEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub plus, pub eq }
	#[derive(Clone)] (-=) as pub MinusEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub minus, pub eq }
	#[derive(Clone)] (*=) as pub StarEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub star, pub eq }
	#[derive(Clone)] (/=) as pub SlashEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub slash, pub eq }
	#[derive(Clone)] (%=) as pub PercentEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub percent, pub eq }
	#[derive(Clone)] (^=) as pub CaretEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub caret, pub eq }
	#[derive(Clone)] (!=) as pub NotEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub not, pub eq }
	#[derive(Clone)] (&=) as pub AndEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub and, pub eq }
	#[derive(Clone)] (|=) as pub OrEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub or, pub eq }
	#[derive(Clone)] (<<=) as pub ShlEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub lt0, pub lt1, pub eq }
	#[derive(Clone)] (>>=) as pub ShrEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub gt0, pub gt1, pub eq }
	#[derive(Clone)] (=) not before [=] as pub Eq: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub eq }
	#[derive(Clone)] (==) as pub EqEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub eq0, pub eq1 }
	#[derive(Clone)] (!=) as pub Ne: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub not, pub eq }
	#[derive(Clone)] (>) not before [> =] as pub Gt: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub gt }
	#[derive(Clone)] (<) not before [< = -] as pub Lt: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub lt }
	#[derive(Clone)] (>=) as pub Ge: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub gt, pub eq }
	#[derive(Clone)] (<=) as pub Le: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub lt, pub eq }
	#[derive(Clone)] (@) as pub At: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub at }
	#[derive(Clone)] (_) as pub Underscore: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub underscore }
	#[derive(Clone)] (.) not before [.] as pub Dot: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub dot }
	#[derive(Clone)] (..) not before [. =] as pub DotDot: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub dot0, pub dot1 }
	#[derive(Clone)] (...) as pub DotDotDot: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub dot0, pub dot1, pub dot2 }
	#[derive(Clone)] (..=) as pub DotDotEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub dot0, pub dot1, pub eq }
	#[derive(Clone)] (,) as pub Comma: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub comma }
	#[derive(Clone)] (;) as pub Semi: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub semi }
	#[derive(Clone)] (:) not before [:] as pub Colon: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub colon }
	#[derive(Clone)] (::) as pub PathSep: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub colon0, pub colon1 }
	#[derive(Clone)] (->) as pub RArrow: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub minus, pub gt }
	#[derive(Clone)] (=>) as pub FatArrow: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub eq, pub gt }

	///
	/// Unused since before Rust 1.0, but still treated as single token.
	#[derive(Clone)] (<-) as pub LArrow: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub lt, pub minus }

	#[derive(Clone)] (#) as pub Pound: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub pound }
	#[derive(Clone)] ($) as pub Dollar: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub dollar }
	#[derive(Clone)] (?) as pub Question: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub question }
	#[derive(Clone)] (~) as pub Tilde: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub tilde }
}

// `!`
impl Default for Not {
	fn default() -> Self {
		Self {
			not: Punct::new('!', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Not {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(not)]
					if not.as_char() == '!' && not.spacing() == Spacing::Alone =>
				{
					Ok(Self { not })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `!`.", spans))
			})
	}
}

impl IntoTokens for Not {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.not.into_tokens(root, tokens)
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

// `_`
impl Default for Underscore {
	fn default() -> Self {
		Self {
			underscore: Punct::new('_', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Underscore {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(underscore)] if underscore.as_char() == '_' => {
					Ok(Self { underscore })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `_`.", spans))
			})
	}
}

impl IntoTokens for Underscore {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.underscore.into_tokens(root, tokens)
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

// `::`
impl Default for PathSep {
	fn default() -> Self {
		Self {
			colon0: Punct::new(':', Spacing::Joint).with_span(Span::mixed_site()),
			colon1: Punct::new(':', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for PathSep {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(colon0), TokenTree::Punct(colon1)]
					if colon0.as_char() == ':'
						&& colon0.spacing() == Spacing::Joint
						&& colon1.as_char() == ':'
						&& colon1.spacing() == Spacing::Alone =>
				{
					Ok(Self { colon0, colon1 })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `::`.", spans))
			})
	}
}

impl IntoTokens for PathSep {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.colon0.into_tokens(root, tokens);
		self.colon1.into_tokens(root, tokens);
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

// `#`
impl Default for Pound {
	fn default() -> Self {
		Self {
			pound: Punct::new('#', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Pound {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(pound)] if pound.as_char() == '#' => Ok(Self { pound }),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `#`.", spans))
			})
	}
}

impl IntoTokens for Pound {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.pound.into_tokens(root, tokens)
	}
}

// `$`
impl Default for Dollar {
	fn default() -> Self {
		Self {
			dollar: Punct::new('$', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopParsedFrom for Dollar {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(dollar)]
					if dollar.as_char() == '$' && dollar.spacing() == Spacing::Alone =>
				{
					Ok(Self { dollar })
				}
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `$`.", spans))
			})
	}
}

impl IntoTokens for Dollar {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.dollar.into_tokens(root, tokens)
	}
}
