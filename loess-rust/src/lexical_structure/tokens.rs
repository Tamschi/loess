use loess::{
	Error, ErrorPriority, Errors, Input, IntoTokens, PeekFrom, SimpleSpanned,
	grammar_helpers::PopParsedFrom,
};
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

/// `->`
#[derive(Clone)]
pub struct RArrow(pub Punct, pub Punct);

impl Default for RArrow {
	fn default() -> Self {
		Self(
			Punct::new('-', Spacing::Joint).with_span(Span::mixed_site()),
			Punct::new('>', Spacing::Alone).with_span(Span::mixed_site()),
		)
	}
}

/// `->`
impl PeekFrom for RArrow {
	fn peek_from(input: &Input) -> bool {
		input.peek(|tts, _| {
			matches!(tts, [TokenTree::Punct(minus), TokenTree::Punct(gt)]
			if minus.as_char() == '-' && minus.spacing() == Spacing::Joint && gt.as_char() == '>')
		})
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
					Ok(Self(minus, gt))
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
		let Self(minus, gt) = self;
		tokens.extend([minus.into(), gt.into()])
	}
}

/// `..`
#[derive(Clone)]
pub struct DotDot(pub Punct, pub Punct);

impl Default for DotDot {
	fn default() -> Self {
		Self(
			Punct::new('.', Spacing::Joint).with_span(Span::mixed_site()),
			Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()),
		)
	}
}

/// `..`
impl PeekFrom for DotDot {
	fn peek_from(input: &Input) -> bool {
		input.peek(|tts, mut rest| {
			matches!(tts, [TokenTree::Punct(dot0), TokenTree::Punct(dot1)]
			if dot0.as_char() == '.'
				&& dot0.spacing() == Spacing::Joint
				&& dot1.as_char() == '.'
				&& (dot1.spacing() == Spacing::Alone || !matches!(rest.next(), Some(TokenTree::Punct(next_punct)) if matches!(next_punct.as_char(), '.' | '='))))
		})
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
					=> { Ok(Self(dot0, dot1)) }
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `..`.", spans))
			})
	}
}

impl IntoTokens for DotDot {
	fn into_tokens(self, _root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		let Self(dot0, dot1) = self;
		tokens.extend([dot0.into(), dot1.into()])
	}
}

/// `;`
#[derive(Clone)]
pub struct Semi(pub Punct);

impl Default for Semi {
	fn default() -> Self {
		Self(Punct::new(';', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

/// `;`
impl PeekFrom for Semi {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(semi)) if semi.as_char() == ';',
		)
	}
}

impl PopParsedFrom for Semi {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(semi)] if semi.as_char() == ';' => Ok(Self(semi)),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `;`.", spans))
			})
	}
}

impl IntoTokens for Semi {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

/// `,`
#[derive(Clone)]
pub struct Comma(pub Punct);

impl Default for Comma {
	fn default() -> Self {
		Self(Punct::new(',', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

/// `,`
impl PeekFrom for Comma {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(comma)) if comma.as_char() == ',',
		)
	}
}

impl PopParsedFrom for Comma {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(comma)] if comma.as_char() == ',' => Ok(Self(comma)),
				other => Err(other),
			})
			.map_err(|spans| {
				errors.push(Error::new(ErrorPriority::GRAMMAR, "Expected `,`.", spans))
			})
	}
}

impl IntoTokens for Comma {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.into_tokens(root, tokens)
	}
}

/// `|`
#[derive(Clone)]
pub struct Or(pub Punct);

impl Default for Or {
	fn default() -> Self {
		Self(Punct::new('|', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

/// `|`
impl PeekFrom for Or {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(or)) if or.as_char() == '|' && or.spacing() == Spacing::Alone,
		)
	}
}

impl PopParsedFrom for Or {
	type Parsed = Self;
	fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
		input
			.pop_or_replace(|tts, _| match tts {
				[TokenTree::Punct(or)] if or.as_char() == '|' && or.spacing() == Spacing::Alone => {
					Ok(Self(or))
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
		self.0.into_tokens(root, tokens)
	}
}

/// `.`
#[derive(Clone)]
pub struct Dot(pub Punct);

impl Default for Dot {
	fn default() -> Self {
		Self(Punct::new('.', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

/// `.`
impl PeekFrom for Dot {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(dot)) if dot.as_char() == '.' && dot.spacing() == Spacing::Alone,
		)
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
					Ok(Self(dot))
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
		self.0.into_tokens(root, tokens)
	}
}

/// `:`
#[derive(Clone)]
pub struct Colon(pub Punct);

impl Default for Colon {
	fn default() -> Self {
		Self(Punct::new(':', Spacing::Alone).with_span(Span::mixed_site()))
	}
}

/// `:`
impl PeekFrom for Colon {
	fn peek_from(input: &Input) -> bool {
		matches!(
			input.front(),
			Some(TokenTree::Punct(colon)) if colon.as_char() == ':' && colon.spacing() == Spacing::Alone,
		)
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
					Ok(Self(colon))
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
		self.0.into_tokens(root, tokens)
	}
}

macro_rules! ident_token {
	($name:ident = $str:literal) => {
		#[doc = concat!("`", $str, "`")]
		#[derive(Clone)]
		pub struct $name(pub Ident);

		#[doc = concat!("`", $str, "`")]
		impl PeekFrom for $name {
			fn peek_from(input: &Input) -> bool {
				matches!(input.front(), Some(TokenTree::Ident(ident)) if ident == $str)
			}
		}

		impl PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
			where
				Self: Sized,
			{
				input
					.pop_or_replace(|t, _| match t {
						[TokenTree::Ident(ident)] if ident == $str => Ok(Self(ident)),
						other => Err(other),
					})
					.map_err(|spans| {
						errors.push(Error::new(ErrorPriority::TOKEN, concat!("Expected `", $str, "`."), spans))
					})
			}
		}

		impl IntoTokens for $name {
			fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
				self.0.into_tokens(root, tokens)
			}
		}
	};
}

ident_token!(As = "as");
ident_token!(Async = "async");
ident_token!(Await = "await");
ident_token!(Box = "box");
ident_token!(Const = "const");
ident_token!(For = "for");
ident_token!(In = "in");
ident_token!(Let = "let");
ident_token!(Pub = "pub");
ident_token!(SelfLowercase = "self");
ident_token!(Struct = "struct");
