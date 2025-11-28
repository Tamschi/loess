use std::str::FromStr;

use proc_macro2::{Delimiter, Group, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::IntoTokens;

pub use crate::{block_directive, quote_one2, rust_statement_directive};

/// Parser- and serialiser-generator macro.
///
/// ```
/// use loess::{
/// 	grammar,
/// 	rust_grammar::{ // With the `"rust_grammar"` feature.
/// 		Identifier, Let, Parentheses, SquareBrackets, Visibility,
/// 	},
/// };
/// use proc_macro2::{Ident, TokenTree, Punct};
///
/// grammar! {
/// 	///
/// 	/// Has auto-documented grammar.
/// 	#[derive(Clone)]
/// 	pub enum Alternatives: doc, PeekFrom, PopFrom, IntoTokens {
/// 		Identifier(Identifier),
/// 		Paren(Parentheses),
/// 		Bracket(SquareBrackets<Vec<TokenTree>>),
/// 		Vis(Visibility),
/// 	} else "Expected Alternative.";
///
/// 	#[derive(Clone)]
/// 	/// `visibility` can't be first, as `Option` isn't `PeekFrom`.
/// 	/// However, `Visibility` itself is `PeekFrom` (checking for `pub`).
/// 	///
/// 	/// Fields are parsed and emitted in order.
/// 	pub struct StructuredSequence: PeekFrom, PopFrom, IntoTokens {
/// 		pub r#let: Let,
/// 		pub visibility: Option<Visibility>,
/// 		pub paren_ident: Parentheses<Ident>,
/// 		pub vec_punct: Vec<Punct>,
/// 	}
///
/// 	#[derive(Clone)]
/// 	/// Generated implementations for tuple structs are currently the most limited.
/// 	pub struct TupleSequence: PeekFrom, PopFrom (
/// 		pub Let,
/// 		pub Option<StructuredSequence>,
/// 		pub Parentheses<Ident>,
/// 		pub Vec<Punct>,
/// 	);
/// }
/// ```
///
/// [`grammar!`] is fully hygienic and uses `$crate`, so can rename dependencies freely.
#[macro_export]
macro_rules! grammar {
	//TODO: Change impl separator to `+`.
	{
		$(#[$($attr:tt)*])*
		$vis:vis enum $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {$(
			$(#[$($variant_attr:tt)*])*
			$variant:ident($($type:ty),*$(,)?)
		),*$(,)?} else $error:expr;

		$($tt:tt)*
	} => {
		#[cfg_attr(any($($($(all(), $(@ $doc)?)?)?)*), doc = $crate::grammar!(@enum_doc [$([$($type,)*])*]))]
		$(#[$($attr)*])*
		$vis enum $name {$(
			$(#[$($variant_attr)*])*
			$variant($(<$type as $crate::PopParsedFrom>::Parsed),*),
		)*}

		#[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				false
				$(|| $crate::grammar!(@peek_first $name input $($type,)*))*
			}
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				$crate::__::Result::Ok($(if let Some(values) = ($(<$type as $crate::PopParsedFrom>::peek_pop_parsed_from(input, errors)?),*) {
					Self::$variant(values)
				} else)* {
					return $crate::__::Result::Err(errors.push($crate::Error::new(
						$crate::ErrorPriority::GRAMMAR,
						$error,
						[input.front_span()],
					)));
				})
			}
		}

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				match self {
					$(Self::$variant(value) => $crate::IntoTokens::into_tokens(value, root, tokens),)*
				}
			}
		}

		$crate::grammar!($($tt)*);
	};
	{
		$(#[$($attr:tt)*])*
		$vis:vis struct $name:ident$(: $(
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
		),*)? {$(
			$(#[$($field_attr:tt)*])*
			$field_vis:vis $field:ident: $type:ty
		),*$(,)?}

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis struct $name {$(
			$(#[$($field_attr)*])*
			$field_vis $field: <$type as $crate::PopParsedFrom>::Parsed,
		)*}

		#[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				$crate::grammar!(@peek_first $name input $($type,)*)
			}
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				$crate::__::Result::Ok(Self {
					$($field: <$type as $crate::PopParsedFrom>::pop_parsed_from(input, errors)?,)*
				})
			}
		}

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				let Self {
					$($field,)*
				} = self;
				$($crate::IntoTokens::into_tokens($field, root, tokens);)*
			}
		}

		$crate::grammar!($($tt)*);
	};
	{
		$(#[$($attr:tt)*])*
		$vis:vis struct $name:ident$(: $(
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
		),*)? ($(
			$(#[$($field_attr:tt)*])*
			$field_vis:vis $type:ty
		),*$(,)?);

		$($tt:tt)*
	} => {
		$(#[$($attr)*])*
		$vis struct $name ($(
			$(#[$($field_attr)*])*
			$field_vis <$type as $crate::PopParsedFrom>::Parsed,
		)*);

		#[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				$crate::grammar!(@peek_first $name input $($type,)*)
			}
		}

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				$crate::__::Result::Ok(Self (
					$(<$type as $crate::PopParsedFrom>::pop_parsed_from(input, errors)?,)*
				))
			}
		}

		$crate::grammar!($($tt)*);
	};
	(@peek_first $name:ident $input:ident $type:ty, $($rest:ty,)*) => (
		<$type as $crate::PeekFrom>::peek_from($input)
	);
	(@peek_first $name:ident $input:ident) => (
		::core::compile_error!($crate::__::concat!("To implement `PeekFrom` for `", $crate::__::stringify!($name), "`, at least one field is necessary."))
	);
	(@enum_doc []) => (
		// Empty.
		""
	);
	(@enum_doc [[$($type0:ty,)*] $([$($type:ty,)*])*]) => (
		// Start.
		$crate::grammar!(@enum_doc [$([$($type,)*])*] [$("[`", $crate::__::stringify!($type0), "`] ", )*])
	);
	(@enum_doc [[$($type0:ty,)*] $([$($type:ty,)*])*] [$($output:tt)*]) => (
		// Continue.
		$crate::grammar!(@enum_doc [$([$($type,)*])*] [$($output)* "| ", $("[`", $crate::__::stringify!($type0), "`] ", )*])
	);
	(@enum_doc [] [$($output:tt)*]) => (
		// End.
		$crate::__::concat!($($output)*)
	);
	{$t:tt $($tt:tt)*} => {
		// Error
		::core::compile_error!($crate::__::concat!("Unexpected grammar input: ", $crate::__::stringify!($t $($tt)*)));
	};
	{} => {}; // Stop.
}

/// Simple generic quotation (statement) macro that works well with Loess's types.
///
/// Uses `{#identifier … }`-style directives (see below).
///
/// Applies [`Span::mixed_site()`] resolution to quoted tokens, but locates them at `$span`.
///
/// Note that this macro emits punctuation verbatim rather than splitting Rust operators!
/// When emitting Rust code, consider spacing consecutive operators for possibly maybe
/// better forwards-compatibility with future Rust edition consumers of your proc macro.
///
/// ```rust
/// use loess::{quote_into_mixed_site, SimpleSpanned};
/// use proc_macro2::{Ident, TokenStream};
///
/// fn my_quote(id1: Ident, id2: Option<Ident>, root: &TokenStream) -> TokenStream {
/// 	let mut output = TokenStream::new();
///
/// 	quote_into_mixed_site!(id1.span(), root, &mut output, {
/// 		pub struct {#(id1)};
///
/// 		{#if let Some(id2) = id2 {
/// 			{#located_at(id2.span()) {
/// 				pub struct {#(id2)};
/// 			}}
/// 		} else {
/// 			{#error { "`id2` is missing." }}
/// 		}}
/// 	});
///
/// 	output
/// }
/// ```
///
/// # Parameters
///
/// ## <code>$span: [`Span`]</code>
///
/// A `Span` that controls which part of the input errors are reported on and which
/// hygiene context certain identifiers are resolved with. In most cases, you should use
/// an as-specific-as-possible `Span` from your macro input here, so that the user of your
/// macro will have an easier time solving issues.
///
/// [`raw_quote_into_mixed_site!`] automatically uses `mixed_site` resolution on quoted
/// tokens (but not pasted [`IntoTokens`] values!). This isolates resolution for scoped
/// bindings (but not items, so please use fully qualified paths and ideally the `$crate`-
/// `$root` pattern from Loess's README that can be viewed [in the root module](crate),
/// with [`quote_into_mixed_site`] instead of this macro).
///
/// ## <code>$tokens: impl [`Extend`]&lt;[`TokenTree`]></code>
///
/// The collection (or other sink) of [`TokenTree`]s to extend.
///
/// # `[$($tt:tt)*]`
///
/// Within square brackets, tokens to emit with `$span` as [`Span`] into `$tokens`.
///
/// Directives are supported within the square brackets, unless noted otherwise.
///
/// # Directives
///
/// Most directives are expanded to emit tokens dynamically and/or into control flow statements.
///
/// Certain directives do neither¹ and instead modify the context of how tokens are emitted.
///
/// Nested directives are supported unless noted otherwise.
///
/// ¹ These generally do expand to an explicit block still, just so there is no wrong shadowing
///   when you inline the macro into your source code. Outside of that, macro hygiene would be
///   enough to apply the right identifier distinctions, though.
///
/// ## Emitting directives
///
/// ### `{#( $($expr:expr),*$(,)? )}`
///
/// Emits each `$expr` as/through [`IntoTokens`], without adjusting [`Span`]s.
///
/// Note that `&T: IntoTokens` where `T: IntoTokens` (via [`Clone`]),
/// so you can prefix pasted expressions with `&` to easily clone them.
///
/// ### `{#raw { $($tt:tt)* }}`
///
/// More efficiently emits `$($tt)*` verbatim, by [`stringify!`]ing it in bulk but
/// without support for nested directives. If you have long sections of verbatim tokens,
/// using this directive may speed up your build and potentially runtime, even if there's
/// nothing inside that you couldn't emit otherwise.
///
/// ### `{#error { $($tt:tt)* }}` <sub>uses <code>$root[`::core`]</code></sub>
///
/// Emits a [`compile_error!`]. `$($tt:tt)*` must emit a string literal, optionally followed by a `,`.
///
/// ### `{#root}`
///
/// Pastes a clone of the `$root` given to the initial call.
///
/// ## Context directives
///
/// ### `{#mixed_site { $($tt:tt)* }}`
///
/// Nested tokens will be resolved with mixed site hygiene and warnings on them will be suppressed.
///
/// (The location for diagnostics remains unchanged.)
///
/// ### `{#call_site { $($tt:tt)* }}`
///
/// Nested tokens will be resolved with call site hygiene and warnings on them appear to the caller.
///
/// (The location for diagnostics remains unchanged.)
///
/// ### `{#located_at($span2:expr) { $($tt:tt)* }}`
///
/// Nested tokens will use `$span2`'s location for diagnostics, but keep the outer hygiene scope.
///
/// ### `{#resolved_at($span2:expr) { $($tt:tt)* }}`
///
/// Nested tokens will use `$span2`'s hygiene scope, but keep the outer location information.
///
/// ### `{#with_exact_span($span:expr) { $($tt:tt)* }}`
///
/// Nested tokens are emitted exactly with copies of `$span` as [`Span`].
///
/// ## Statement directives
///
/// ### `{#let $pat:pat = $expr:expr $(else { $($else:tt)* })?;}`
///
/// Expands into a `let` binding with optional divergent `else` branch.
///
/// ### `{#break $($label:lifetime)? $($expr:expr)?;}`
///
/// Expands into a `break` statement with optional label and optional expression.
///
/// ### `{#continue $($label:lifetime)?;}`
///
/// Expands into a `continue` statement with optional label.
///
/// ### `{#return $($expr:expr)?;}`
///
/// Expands into a `return` statement with optional expression.
///
/// ## Block directives
///
/// ### `{#if $(let $pat:pat =)? $expr:expr { $($tt:tt)* }}`
///
/// Expands into an `if`-statement that conditionally emits the nested quote.
///
/// ### `{#match $expr:expr { $($tt:tt)*  }}`
///
/// Expands into a `match` statement (which must be exhaustive).
///
/// The body of this directive is that of a normal `match` statement, including the option
/// to use inner attributes on the `match` and outer attributes on the branches, except
/// that branches must use curly braces (`=> { $(tt:tt)* }`) and that tokens inside those
/// braces are interpreted as conditionally emitted nested quote.
///
/// ### `{# $($label:lifetime:)? $(loop)? { $($tt:tt)* }}`
///
/// Expands into a block or `loop`-statement with optional label.
///
/// The nested quote expands into the loop's body.
///
/// ### `{# $($label:lifetime:)? for $pat:pat in $expr:expr { $($tt:tt)* }}`
///
/// Expands into a `for in` loop with optional label.
///
/// The nested quote expands into the loop's body.
///
/// ### `{# $($label:lifetime:)? while $(let $pat:pat =)? $expr:expr { $($tt:tt)* }}`
///
/// Expands into a `while` or `while let` loop with optional label.
///
/// The nested quote expands into the loop's body.
///
/// ### `else`
///
/// `else` can be inserted before the outer closing brace of an `if`- or conditional loop
/// directive, and must be directly followed by another block directive *without* outer
/// braces or label.
///
/// This means you can chain block directives as follows:
///
/// ```rust
/// use loess::quote_into_mixed_site;
/// use proc_macro2::{Span, TokenStream};
///
/// fn my_quote(span: Span, root: &TokenStream, output: &mut TokenStream) {
/// 	quote_into_mixed_site!(span, root, output, {
///
/// 		// Emits `b`.
/// 		{#if false {
/// 			a
/// 		} else {
/// 			b
/// 		}}
///
/// 		// Emits `e`.
/// 		{#if false {
/// 			a
/// 		} else for _ in 0..0 {
/// 			b
/// 		} else while let Some(0) = None {
/// 			c
/// 		} else match 0 {
/// 			1 => { d }
/// 			_ => { e }
/// 		}}
///
/// 	});
/// }
/// ```
#[macro_export]
macro_rules! quote_into_mixed_site {
	($span:expr, $root:expr, $tokens:expr, {$($tt:tt)*}$(,)?) => ({
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		let root: &$crate::__::TokenStream = $root;
		let tokens = $tokens;
		$( $crate::__::quote_one2!(span root tokens, $tt); )*
	});
}

/// Like [`quote_into_mixed_site!`], but using `$span` directly for quoted tokens.
#[macro_export]
macro_rules! quote_into_with_exact_span {
	($span:expr, $root:expr, $tokens:expr, {$($tt:tt)*}$(,)?) => ({
		let span: $crate::__::Span = $span;
		let root: &$crate::__::TokenStream = $root;
		let tokens = $tokens;
		$( $crate::__::quote_one2!(span root tokens, $tt); )*
	});
}

/// Like [`quote_into_mixed_site!`], but resolved according to [`Span::call_site()`].
#[macro_export]
macro_rules! quote_into_call_site {
	($span:expr, $root:expr, $tokens:expr, {$($tt:tt)*}$(,)?) => ({
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		let root: &$crate::__::TokenStream = $root;
		let tokens = $tokens;
		$( $crate::__::quote_one2!(span root tokens, $tt); )*
	});
}

/// Simple generic quotation (statement) macro that efficiently emits tokens verbatim.
///
/// # Parameters
///
/// ## <code>$span: [`Span`]</code>
///
/// A `Span` that controls which part of the input errors are reported on and which
/// hygiene context certain identifiers are resolved with. In most cases, you should use
/// an as-specific-as-possible `Span` from your macro input here, so that the user of your
/// macro will have an easier time solving issues.
///
/// [`raw_quote_into_mixed_site!`] automatically uses `mixed_site` resolution on quoted
/// tokens (but not pasted [`IntoTokens`] values!). This isolates resolution for scoped
/// bindings (but not items, so please use fully qualified paths and ideally the `$crate`-
/// `$root` pattern from Loess's README that can be viewed [in the root module](crate),
/// with [`quote_into_mixed_site`] instead of this macro).
///
/// ## <code>$tokens: impl [`Extend`]&lt;[`TokenTree`]></code>
///
/// The collection (or other sink) of [`TokenTree`]s to extend.
///
/// # `[$($tt:tt)*]`
///
/// Within square brackets: Tokens to emit verbatim but with `$span` as [`Span`] into `$tokens`.
#[macro_export]
macro_rules! raw_quote_into_mixed_site {
	($span:expr, $tokens:expr, {$($tt:tt)*}$(,)?) => {{
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	}};
	($span:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => {{
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	}};
}

/// Like [`raw_quote_into_mixed_site!`], but using `$span` directly for quoted tokens.
#[macro_export]
macro_rules! raw_quote_into_with_exact_span {
	($span:expr, $tokens:expr, {$($tt:tt)*}$(,)?) => {
		$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	};
	($span:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => {
		$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	};
}

/// Like [`raw_quote_into_mixed_site!`], but resolved according to [`Span::call_site()`].
#[macro_export]
macro_rules! raw_quote_into_call_site {
	($span:expr, $tokens:expr, {$($tt:tt)*}$(,)?) => {{
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	}};
	($span:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => {{
		let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! quote_one2 {
		// #() (shortcut: paste none)
		($span:tt $root:tt $tokens:tt, {#()}) => { };
		// #(expr) (shortcut: paste one)
		($span:tt $root:tt $tokens:tt, {#($expr:expr$(,)?)}) => {
			$crate::IntoTokens::into_tokens($expr, $root, $tokens);
		};
		// #(…expr) (paste tuple)
		($span:tt $root:tt $tokens:tt, {#($($expr:expr),*$(,)?)}) => {
			$crate::IntoTokens::into_tokens($crate::__::Paste(($($expr,)*)), $root, $tokens);
		};

		// #raw
		($span:tt $root:tt $tokens:tt, {#raw { $($tt:tt)* }}) => {
			$crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
		};

		// #error
		($span:tt $root:tt $tokens:tt, {#error { $($tt:tt)* }}) => {{
				$crate::IntoTokens::into_tokens($crate::__::Clone::clone($root), $root, $tokens);
				$crate::__::raw($span, $tokens, "::core::compile_error!");
				$crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
					let mut inner_tokens = $crate::__::TokenStream::new();
					$( $crate::__::quote_one2!($span $root (&mut inner_tokens), $tt); )*
					inner_tokens
				});
		}};

		// #root
		($span:tt $root:tt $tokens:tt, {#root}) => {
			$crate::IntoTokens::into_tokens($crate::__::Clone::clone($root), $root, $tokens);
		};

		// Block directives without bare expressions can be matched directly:

		// #mixed_site
		($span:tt $root:tt $tokens:tt, {#mixed_site { $($tt:tt)* }}) => {{
			let span = $span.resolved_at($crate::__::Span::mixed_site());
			$( $crate::__::quote_one2!(span $root $tokens, $tt); )*
		}};

		// #call_site
		($span:tt $root:tt $tokens:tt, {#call_site { $($tt:tt)* }}) => {{
			let span = $span.resolved_at($crate::__::Span::call_site());
			$( $crate::__::quote_one2!(span $root $tokens, $tt); )*
		}};

		// #located_at
		($span:tt $root:tt $tokens:tt, {#located_at($span2:expr) { $($tt:tt)* }}) => {{
			let span = $span.located_at($span2);
			$( $crate::__::quote_one2!(span $root $tokens, $tt); )*
		}};

		// #resolved_at
		($span:tt $root:tt $tokens:tt, {#resolved_at($span2:expr) { $($tt:tt)* }}) => {{
			let span = $span.resolved_at($span2);
			$( $crate::__::quote_one2!(span $root $tokens, $tt); )*
		}};

		// #with_exact_span
		($_span:tt $root:tt $tokens:tt, {#with_exact_span($span:expr) { $($tt:tt)* }}) => {{
			let span: $crate::__::Span = $span;
			$( $crate::__::quote_one2!(span $root $tokens, $tt); )*
		}};

		// #let
		($span:tt $root:tt $tokens:tt, {#let $($tt:tt)*}) => {
			$crate::__::rust_statement_directive!([let] $($tt)*);
		};

		// #break
		($span:tt $root:tt $tokens:tt, {#break $($tt:tt)*}) => {
			$crate::__::rust_statement_directive!([break] $($tt)*);
		};

		// #continue
		($span:tt $root:tt $tokens:tt, {#continue $($tt:tt)*}) => {
			$crate::__::rust_statement_directive!([continue] $($tt)*);
		};

		// #return
		($span:tt $root:tt $tokens:tt, {#return $($tt:tt)*}) => {
			$crate::__::rust_statement_directive!([return] $($tt)*);
		};

		// block-only directive (`{}`)
		($span:tt $root:tt $tokens:tt, {#{$($nested:tt)*} $($($unexpected:tt)+)?}) => {
			{
				$( $crate::__::quote_one2!($span $root $tokens, $nested); )*
			}

			$( $crate::__::compile_error!($crate::__::concat!("Unexpected tokens after `#{ … }`-directive:", $(" `", $crate::__::stringify!($unexpected), "`"),* )); )?
		};

		// block directives (ident)
		($span:tt $root:tt $tokens:tt, {#$ident:ident $($tt:tt)*}) => {
			$crate::__::block_directive!($span $root $tokens, [] [$ident $ident] $([$tt $tt])*);
		};

		// block directives (label)
		($span:tt $root:tt $tokens:tt, {#$label:lifetime $($tt:tt)*}) => {
			$crate::__::block_directive!($span $root $tokens, [] [$label $label] $([$tt $tt])*);
		};

		// ($span:tt $root:tt $tokens:tt, {#macro $path:path $([
		// 	//TODO: Ideas wanted! Ideally called macros should be self-describing in terms of what context they want,
		// 	//TODO: ideally by listing identifiers in a way where this macro can check for validity and report issues,
		// 	//TODO: but the system shouldn't be too complicated.
		// 	//TODO: Maybe the macro initially takes a "callback" and emits its requested context parameters and path to stage two into that.
		// 	//TODO: That callback can then call stage two with the requested arguments or error. The quoted tokens are passed along at each step.
		// 	$(
		// 		$(loess $(@ $loess:tt)?)?
		// 		$(span $(@ $span_:tt)?)?
		// 		$(root $(@ $root_:tt)?)?
		// 		$(tokens $(@ $tokens_:tt)?)?
		// 	),* $(, $(@ $comma:tt)?)?
		// ])?, $($tt:tt)*}) => {
		// 	$path!(
		// 		$([
		// 			$(
		// 				$($crate $(@ $loess_)?)?
		// 				$($span $(@ $span_)?)?
		// 				$($root $(@ $root_)?)?
		// 				$($tokens $(@ $tokens_)?)?
		// 			),* $(, $(@ $comma)?)?
		// 		])?
		// 		$($tt)*
		// 	);
		// };

		// {}
		($span:tt $root:tt $tokens:tt, {$($tt:tt)*}) => {
			$crate::__::grouped($span, $crate::__::Brace, $tokens, {
				let mut inner_tokens = $crate::__::TokenStream::new();
				$( $crate::__::quote_one2!($span $root (&mut inner_tokens), $tt); )*
				inner_tokens
			});
		};

		// []
		($span:tt $root:tt $tokens:tt, [$($tt:tt)*]) => {
			$crate::__::grouped($span, $crate::__::Bracket, $tokens, {
				let mut inner_tokens = $crate::__::TokenStream::new();
				$( $crate::__::quote_one2!($span $root (&mut inner_tokens), $tt); )*
				inner_tokens
			});
		};

		// ()
		($span:tt $root:tt $tokens:tt, ($($tt:tt)*)) => {
			$crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
				let mut inner_tokens = $crate::__::TokenStream::new();
				$( $crate::__::quote_one2!($span $root (&mut inner_tokens), $tt); )*
				inner_tokens
			});
		};

		// other tokens
		($span:tt $root:tt $tokens:tt, $other:tt) => (
			// Fortunately `'` can't arrive as trailing punctuation inside `$other` here (It's always either a literal or inside a lifetime),
			// so it's at least reasonable to expect `stringify!` to insert a space iff the trailing punctuation is spaced and otherwise not.
			$crate::__::tt($span, $tokens, const { $crate::__::strip_dot($crate::__::stringify!($other .)) } );
		);

		// End.
		($span:tt $root:tt $tokens:tt,) => {};
	}

#[doc(hidden)]
#[macro_export]
macro_rules! rust_statement_directive {
		// End.
		([$($tt:tt)*] ;) => { $($tt)+; };

		// Unexpected after `;`.
		([$($tt:tt)*] ; $($rest:tt)+ ) => {
			$($tt)+; // Expand the complete statement to avoid cascading errors.

			// It would be possible to allow further statement directives here,
			// but that's a design question that can be answered another time.
			// $crate::__::rust_statement_directive!([] $($rest)+);

			$crate::__::compile_error!($crate::__::concat!("Encountered tokens after `;` in statement directive:", $(" `", $crate::__::stringify!($rest), "`", )*));
		};

		// Next.
		([$($tt:tt)*] $next:tt $($rest:tt)+ ) => {
			$crate::__::rust_statement_directive!([$($tt)* $next] $($rest)+);
		};

		// Incomplete.
		([$($tt:tt)*] $incomplete:tt ) => {
			$crate::__::compile_error!($crate::__::concat!("Incomplete statement directive: Expected `;` after this `", $crate::__::stringify!($incomplete), "`."));
		};

		// Incomplete (start token only).
		([$($tt:tt)*]) => {
			$crate::__::compile_error!($crate::__::concat!("Incomplete statement directive: Expected `;` after", $(" `", $crate::__::stringify!($tt), "`")*, "."));
		};
	}

#[doc(hidden)]
#[macro_export]
macro_rules! block_directive {
		// `match`
		($span:tt $root:tt $tokens:tt, [[match $match:tt] $([$header:tt $_header:tt])*] [{ $(#![$($attr:tt)*])* $($(#[$($arm_attr:tt)*])* $pattern:pat $(if $guard_expression:expr)? => { $($nested:tt)* })* } $_:tt] $($([$unexpected:tt $_unexpected:tt])+)? ) => {
			$match $($header)* {
				$(#![$($attr)*])*
				$(
					$(#[$($arm_attr)*])*
					$pattern $(if $guard_expression)? => {
						$( $crate::__::quote_one2!($span $root $tokens, $nested); )*
					}
				)*
			}

			$( $crate::__::compile_error!($crate::__::concat!("Unexpected tokens after `#match`-directive:", $(" `", $crate::__::stringify!($unexpected), "`"),* )); )?
		};

		// Erroneous `match`
		($span:tt $root:tt $tokens:tt, [[match $match:tt] $([$header:tt $_header:tt])*] [{ $($nested:tt)* } $_:tt] $($([$unexpected:tt $_unexpected:tt])+)? ) => {
			$crate::__::compile_error!($crate::__::concat!("Unexpected `#match`-arm somewhere among here (make sure to use curly braces!):", $(" `", $crate::__::stringify!($nested), "`"),* ));

			$( $crate::__::compile_error!($crate::__::concat!("Unexpected tokens after `#match`-directive:", $(" `", $crate::__::stringify!($unexpected), "`"),* )); )?
		};

		// End.
		($span:tt $root:tt $tokens:tt, [$([$header:tt $_header:tt])*] [{ $($nested:tt)* } $_:tt] ) => {
			$($header)* {
				$( $crate::__::quote_one2!($span $root $tokens, $nested); )*
			}
		};

		// Unexpected `;`.
		($span:tt $root:tt $tokens:tt, [$([$header:tt $_header:tt])*] [; $semi:tt] $($rest:tt)* ) => {
			$crate::__::compile_error!($crate::__::concat!("Unexpected `", $crate::__::stringify!($semi), "` in block directive."));
		};

		// `else 'label` -> error (at least for the time being, though this *could* be allowed).
		($span:tt $root:tt $tokens:tt, [$($_header:tt)*] [{ $($_nested:tt)* } $_block:tt] [else $else:tt] [$label:lifetime $_label:tt] $($_rest:tt)* ) => {
			$crate::__::compile_error!($crate::__::concat!("Unexpected `", $crate::__::stringify!($label), "` after `", $crate::__::stringify!($else), "`."));
		};

		// `if … { … } else`
		($span:tt $root:tt $tokens:tt, [$([$label:lifetime $_label:tt] [: $colon:tt])? [if $if:tt] $([$header:tt $_header:tt])*] [{ $($nested:tt)* } $_block:tt] [else $else:tt] $($rest:tt)* ) => {
			$($label $colon)? $if $($header)* {
				$( $crate::__::quote_one2!($span $root $tokens, $nested); )*
			} $else {
				$crate::__::block_directive!($span $root $tokens, [] $($rest)*);
			}
		};

		// `for … { … } else`
		($span:tt $root:tt $tokens:tt, [$([$label:lifetime $_label:tt] [: $colon:tt])? [for $for:tt] $([$header:tt $_header:tt])*] [{ $($nested:tt)* } $_block:tt] [else $else:tt] $($rest:tt)* ) => {{
			let mut skip_else = false;
			$($label $colon)? $for $($header)* {
				skip_else = true;
				$( $crate::__::quote_one2!($span $root $tokens, $nested); )*
			} if skip_else {} $else {
				$crate::__::block_directive!($span $root $tokens, [] $($rest)*);
			}
		}};

		// `while … { … } else`
		($span:tt $root:tt $tokens:tt, [$([$label:lifetime $_label:tt] [: $colon:tt])? [while $while:tt] $([$header:tt $_header:tt])*] [{ $($nested:tt)* } $_block:tt] [else $else:tt] $($rest:tt)* ) => {{
			let mut skip_else = false;
			$($label $colon)? $while $($header)* {
				skip_else = true;
				$( $crate::__::quote_one2!($span $root $tokens, $nested); )*
			} if skip_else {} $else {
				$crate::__::block_directive!($span $root $tokens, [] $($rest)*);
			}
		}};

		// Unexpected `else`.
		($span:tt $root:tt $tokens:tt, [$([$header:tt $_header:tt])*] [else $else:tt] $($rest:tt)* ) => {
			$crate::__::compile_error!($crate::__::concat!("Unexpected `", $crate::__::stringify!($else), "`."));
		};

		// Next.
		($span:tt $root:tt $tokens:tt, [$($tt:tt)*] $next:tt $($rest:tt)* ) => {
			$crate::__::block_directive!($span $root $tokens, [$($tt)* $next] $($rest)*);
		};

		// Incomplete.
		($span:tt $root:tt $tokens:tt, [$($tt:tt)*] [$current:tt $_current:tt] ) => {
			$crate::__::compile_error!($crate::__::concat!("Incomplete block directive: Expected `{` after this `", $crate::__::stringify!($current), "`."));
		};

		// Incomplete (start token only).
		($span:tt $root:tt $tokens:tt, [] [$current:tt $_current:tt]) => {
			$crate::__::compile_error!($crate::__::concat!("Incomplete block directive: Expected `{` after `", $crate::__::stringify!($current), "`."));
		};
	}

pub fn grouped(
	span: Span,
	delimiter: Delimiter,
	tokens: &mut impl Extend<TokenTree>,
	stream: TokenStream,
) {
	let mut group = Group::new(delimiter, stream);
	group.set_span(span);
	tokens.extend([TokenTree::Group(group)]);
}

fn assign_span(
	span: Span,
	ts: impl IntoIterator<Item = TokenTree>,
) -> impl IntoIterator<Item = TokenTree> {
	ts.into_iter().map(move |tt| match tt {
		TokenTree::Group(group) => {
			let group = Group::new(
				group.delimiter(),
				assign_span(span, group.stream()).into_iter().collect(),
			);
			TokenTree::Group(group)
		}
		mut tt @ (TokenTree::Ident(_) | TokenTree::Punct(_) | TokenTree::Literal(_)) => {
			tt.set_span(span);
			tt
		}
	})
}

pub fn raw(span: Span, tokens: &mut impl Extend<TokenTree>, stringified: &str) {
	tokens.extend(assign_span(
			span,
			TokenStream::from_str(stringified).expect("Failed to parse stringified tokens, somehow. (Are you using the internal API from another crate? Please don't.)"),
		))
}

pub const fn strip_dot(s: &str) -> &str {
	s.split_at(s.len() - 1).0
}

pub fn tt(span: Span, tokens: &mut impl Extend<TokenTree>, stringified: &str) {
	// Note that there can actually be multiple tokens here, since `$tt:tt` grabs lifetimes and certain operators in one go!
	let mut ts = TokenStream::from_str(stringified).expect("Failed to parse stringified tokens, somehow. (Are you using the internal API from another crate? Please don't.)").into_iter().collect::<Box<[_]>>();

	// The last token tree's spacing information is lost, but it's easy enough to restore it here:
	if let TokenTree::Punct(trailing_punct) = ts
		.last_mut()
		.expect("always at least one token (Please don't use the internal API from other crates.)")
	{
		*trailing_punct = Punct::new(
			trailing_punct.as_char(),
			match stringified
				.chars()
				.last()
				.expect("always at least one char")
				.is_ascii_whitespace()
			{
				true => Spacing::Alone,
				false => Spacing::Joint,
			},
		)
	}

	tokens.extend(assign_span(span, ts));
}

/// For now this is easiest, but an arbitrary-length macro unroll (mind the eval order!)
/// may be better when high-argument paths often go unused.
pub struct Paste<T>(pub T);

impl<T0: IntoTokens, T1: IntoTokens> IntoTokens for Paste<(T0, T1)> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
	}
}

impl<T0: IntoTokens, T1: IntoTokens, T2: IntoTokens> IntoTokens for Paste<(T0, T1, T2)> {
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
	}
}

impl<T0: IntoTokens, T1: IntoTokens, T2: IntoTokens, T3: IntoTokens> IntoTokens
	for Paste<(T0, T1, T2, T3)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
	}
}

impl<T0: IntoTokens, T1: IntoTokens, T2: IntoTokens, T3: IntoTokens, T4: IntoTokens> IntoTokens
	for Paste<(T0, T1, T2, T3, T4)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
	}
}

impl<T0: IntoTokens, T1: IntoTokens, T2: IntoTokens, T3: IntoTokens, T4: IntoTokens, T5: IntoTokens>
	IntoTokens for Paste<(T0, T1, T2, T3, T4, T5)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
	}
}

impl<
	T0: IntoTokens,
	T1: IntoTokens,
	T2: IntoTokens,
	T3: IntoTokens,
	T4: IntoTokens,
	T5: IntoTokens,
	T6: IntoTokens,
> IntoTokens for Paste<(T0, T1, T2, T3, T4, T5, T6)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
		self.0.6.into_tokens(root, tokens);
	}
}

impl<
	T0: IntoTokens,
	T1: IntoTokens,
	T2: IntoTokens,
	T3: IntoTokens,
	T4: IntoTokens,
	T5: IntoTokens,
	T6: IntoTokens,
	T7: IntoTokens,
> IntoTokens for Paste<(T0, T1, T2, T3, T4, T5, T6, T7)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
		self.0.6.into_tokens(root, tokens);
		self.0.7.into_tokens(root, tokens);
	}
}

impl<
	T0: IntoTokens,
	T1: IntoTokens,
	T2: IntoTokens,
	T3: IntoTokens,
	T4: IntoTokens,
	T5: IntoTokens,
	T6: IntoTokens,
	T7: IntoTokens,
	T8: IntoTokens,
> IntoTokens for Paste<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
		self.0.6.into_tokens(root, tokens);
		self.0.7.into_tokens(root, tokens);
		self.0.8.into_tokens(root, tokens);
	}
}

impl<
	T0: IntoTokens,
	T1: IntoTokens,
	T2: IntoTokens,
	T3: IntoTokens,
	T4: IntoTokens,
	T5: IntoTokens,
	T6: IntoTokens,
	T7: IntoTokens,
	T8: IntoTokens,
	T9: IntoTokens,
> IntoTokens for Paste<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
		self.0.6.into_tokens(root, tokens);
		self.0.7.into_tokens(root, tokens);
		self.0.8.into_tokens(root, tokens);
		self.0.9.into_tokens(root, tokens);
	}
}

impl<
	T0: IntoTokens,
	T1: IntoTokens,
	T2: IntoTokens,
	T3: IntoTokens,
	T4: IntoTokens,
	T5: IntoTokens,
	T6: IntoTokens,
	T7: IntoTokens,
	T8: IntoTokens,
	T9: IntoTokens,
	T10: IntoTokens,
> IntoTokens for Paste<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
		self.0.6.into_tokens(root, tokens);
		self.0.7.into_tokens(root, tokens);
		self.0.8.into_tokens(root, tokens);
		self.0.9.into_tokens(root, tokens);
		self.0.10.into_tokens(root, tokens);
	}
}

impl<
	T0: IntoTokens,
	T1: IntoTokens,
	T2: IntoTokens,
	T3: IntoTokens,
	T4: IntoTokens,
	T5: IntoTokens,
	T6: IntoTokens,
	T7: IntoTokens,
	T8: IntoTokens,
	T9: IntoTokens,
	T10: IntoTokens,
	T11: IntoTokens,
> IntoTokens for Paste<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
{
	fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
		self.0.0.into_tokens(root, tokens);
		self.0.1.into_tokens(root, tokens);
		self.0.2.into_tokens(root, tokens);
		self.0.3.into_tokens(root, tokens);
		self.0.4.into_tokens(root, tokens);
		self.0.5.into_tokens(root, tokens);
		self.0.6.into_tokens(root, tokens);
		self.0.7.into_tokens(root, tokens);
		self.0.8.into_tokens(root, tokens);
		self.0.9.into_tokens(root, tokens);
		self.0.10.into_tokens(root, tokens);
		self.0.11.into_tokens(root, tokens);
	}
}
