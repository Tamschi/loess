//! [lex.token.punct](https://doc.rust-lang.org/stable/reference/tokens.html#r-lex.token.punct): Punctuation
//!
//! Punctuation is implemented as structs with named [`Punct`] fields.  
//! Where collisions would happen, they have a 0-based suffix.

use loess::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PopParsedFrom, SimpleSpanned, grammar,
	punctuation, scaffold::In,
};
use proc_macro2::{Punct, Spacing, Span, TokenStream, TokenTree};

use crate::lex::token::{
	IncludingDelimiters,
	delim::{CurlyBraces, Parentheses, SquareBrackets},
};

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

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [Token](https://doc.rust-lang.org/reference/tokens.html#grammar-PUNCTUATION)
	pub enum Punctuation: PeekFrom, PopFrom, IntoTokens {
		Eq(Eq),
		Lt(Lt),
		Le(Le),
		EqEq(EqEq),
		Ne(Ne),
		Ge(Ge),
		Gt(Gt),
		AndAnd(AndAnd),
		OrOr(OrOr),
		Not(Not),
		Tilde(Tilde),
		Plus(Plus),
		Minus(Minus),
		Star(Star),
		Slash(Slash),
		Percent(Percent),
		Caret(Caret),
		And(And),
		Or(Or),
		Shl(Shl),
		Shr(Shr),
		PlusEq(PlusEq),
		MinusEq(MinusEq),
		StarEq(StarEq),
		SlashEq(SlashEq),
		PercentEq(PercentEq),
		CaretEq(CaretEq),
		AndEq(AndEq),
		OrEq(OrEq),
		ShlEq(ShlEq),
		ShrEq(ShrEq),
		At(At),
		Dot(Dot),
		DotDot(DotDot),
		DotDotDot(DotDotDot),
		DotDotEq(DotDotEq),
		Comma(Comma),
		Semi(Semi),
		Colon(Colon),
		PathSep(PathSep),
		RArrow(RArrow),
		LArrow(LArrow),
		FatArrow(FatArrow),
		Pound(Pound),
		Dollar(Dollar),
		Question(Question),
		Underscore(Underscore),
		CurlyBraces(In<IncludingDelimiters<CurlyBraces>>),
		SquareBrackets(In<IncludingDelimiters<SquareBrackets>>),
		Parentheses(In<IncludingDelimiters<Parentheses>>),
	} else "Expected PUNCTUATION.";
}

// `!`
impl Default for Not {
	fn default() -> Self {
		Self {
			not: Punct::new('!', Spacing::Alone).with_span(Span::mixed_site()),
		}
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

// `_`
impl Default for Underscore {
	fn default() -> Self {
		Self {
			underscore: Punct::new('_', Spacing::Alone).with_span(Span::mixed_site()),
		}
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

// `..`
impl Default for DotDot {
	fn default() -> Self {
		Self {
			dot0: Punct::new('.', Spacing::Joint).with_span(Span::mixed_site()),
			dot1: Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()),
		}
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

// `;`
impl Default for Semi {
	fn default() -> Self {
		Self {
			semi: Punct::new(';', Spacing::Alone).with_span(Span::mixed_site()),
		}
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

// `::`
impl Default for PathSep {
	fn default() -> Self {
		Self {
			colon0: Punct::new(':', Spacing::Joint).with_span(Span::mixed_site()),
			colon1: Punct::new(':', Spacing::Alone).with_span(Span::mixed_site()),
		}
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

// `#`
impl Default for Pound {
	fn default() -> Self {
		Self {
			pound: Punct::new('#', Spacing::Alone).with_span(Span::mixed_site()),
		}
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
