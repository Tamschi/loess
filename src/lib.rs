//! A group-lazy defaulting speculative Rust parser.
//!
//! The crate structure mirrors roughtly [The Rust Reference](https://doc.rust-lang.org/stable/reference/)'s.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Floess)](https://iteration-square.schichler.dev/#narrow/stream/project.2Floess)

#![doc(html_root_url = "https://docs.rs/loess/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned)]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

pub mod expressions;
pub mod help;
pub mod identifiers;
pub mod io;
pub mod names;
pub mod patterns;
pub mod statements;
pub mod tokens;
pub mod types;

#[macro_export]
macro_rules! Token {
	[+] => { $crate::tokens::punctuation::Plus };
	[-] => { $crate::tokens::punctuation::Minus };
	[*] => { $crate::tokens::punctuation::Star };
	[/] => { $crate::tokens::punctuation::Slash };
	[%] => { $crate::tokens::punctuation::Percent };
	[^] => { $crate::tokens::punctuation::Caret };
	[!] => { $crate::tokens::punctuation::Not };
	[&] => { $crate::tokens::punctuation::And };
	[|] => { $crate::tokens::punctuation::Or };
	[&&] => { $crate::tokens::punctuation::AndAnd };
	[||] => { $crate::tokens::punctuation::OrOr };
	[<<] => { $crate::tokens::punctuation::Shl };
	[>>] => { $crate::tokens::punctuation::Shr };
	[+=] => { $crate::tokens::punctuation::PlusEq };
	[-=] => { $crate::tokens::punctuation::MinusEq };
	[*=] => { $crate::tokens::punctuation::StarEq };
	[/=] => { $crate::tokens::punctuation::SlashEq };
	[%=] => { $crate::tokens::punctuation::PercentEq };
	[^=] => { $crate::tokens::punctuation::CareEq };
	[&=] => { $crate::tokens::punctuation::AndEq };
	[|=] => { $crate::tokens::punctuation::OrEq };
	[<<=] => { $crate::tokens::punctuation::ShlEq };
	[>>=] => { $crate::tokens::punctuation::ShrEq };
	[=] => { $crate::tokens::punctuation::Eq };
	[==] => { $crate::tokens::punctuation::EqEq };
	[!=] => { $crate::tokens::punctuation::Ne };
	[>] => { $crate::tokens::punctuation::Gt };
	[<] => { $crate::tokens::punctuation::Lt };
	[>=] => { $crate::tokens::punctuation::Ge };
	[<=] => { $crate::tokens::punctuation::Le };
	[@] => { $crate::tokens::punctuation::At };
	[_] =>{ $crate::tokens::punctuation::Underscore };
	[.] => { $crate::tokens::punctuation::Dot };
	[..] => { $crate::tokens::punctuation::DotDot };
	[...] => { $crate::tokens::punctuation::DotDotDot };
	[..=] => { $crate::tokens::punctuation::DotDotEq };
	[,] => { $crate::tokens::punctuation::Comma };
	[;] => { $crate::tokens::punctuation::Semi };
	[:] => { $crate::tokens::punctuation::Colon };
	[::] => { $crate::tokens::punctuation::PathSep };
	[->] => { $crate::tokens::punctuation::RArrow };
	[=>] => { $crate::tokens::punctuation::FatArrow };
	[#] => { $crate::tokens::punctuation::Pound };
	[$] => { $crate::tokens::punctuation::Dollar };
	[?] => { $crate::tokens::punctuation::Question };
	[~] => { $crate::tokens::punctuation::Tilde };
}
