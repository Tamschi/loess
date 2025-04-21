#[cfg(doc)]
use proc_macro2::{Span, TokenTree};

#[cfg(doc)]
use crate::{IntoTokens, grammar, quote_into_mixed_site, raw_quote_into_mixed_site};

/// Parser- and printer-generator macro.
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
			    $variant($($type),*),
		    )*}

		    #[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		    impl $crate::PeekFrom for $name {
			    fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				    false
				    $(|| $crate::grammar!(@peek_first $name input $($type,)*))*
			    }
		    }

		    #[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		    impl $crate::PopFrom for $name {
			    fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				    $crate::__::Result::Ok($(if let Some(values) = ($(<$type as $crate::PopFrom>::peek_pop_from(input, errors)?),*) {
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
			    $field_vis $field: $type,
		    )*}

		    #[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		    impl $crate::PeekFrom for $name {
			    fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				    $crate::grammar!(@peek_first $name input $($type,)*)
			    }
		    }

		    #[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		    impl $crate::PopFrom for $name {
			    fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				    $crate::__::Result::Ok(Self {
					    $($field: <$type as $crate::PopFrom>::pop_from(input, errors)?,)*
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
			    $field_vis $type,
		    )*);

		    #[cfg(any($($($(all(), $(@ $PeekFrom)?)?)?)*))]
		    impl $crate::PeekFrom for $name {
			    fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				    $crate::grammar!(@peek_first $name input $($type,)*)
			    }
		    }

		    #[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		    impl $crate::PopFrom for $name {
			    fn pop_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, ()> {
				    $crate::__::Result::Ok(Self (
					    $(<$type as $crate::PopFrom>::pop_from(input, errors)?,)*
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
/// use loess::{quote_into_mixed_site, rust_grammar::Identifier, SimpleSpanned};
/// use proc_macro2::TokenStream;
///
/// fn my_quote(id1: Identifier, id2: Option<Identifier>, root: &TokenStream) -> TokenStream {
/// 	let mut output = TokenStream::new();
///
/// 	quote_into_mixed_site!(id1.span(), root, &mut output, [
/// 		pub struct {#paste id1};
///
/// 		{#if let Some(id2) = id2,
/// 			{#located_at id2.span(),
/// 				pub struct {#paste id2};
/// 			}
/// 		} {#else,
/// 			{#error "`id2` is missing."}
/// 		}
/// 	]);
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
/// Most directives are expanded to emit tokens dynamically and/or into control flow statements.
///
/// Certain directives do neither¹ and instead modify the context of how tokens are emitted.
///
/// ¹ These generally do expand to an explicit block still, just so there is no wrong shadowing
///   when you inline the macro into your source code. Outside of that, macro hygiene would be
///   enough to apply the right identifier distinctions, though.
///
/// # Directives
///
/// Nested directives are supported unless noted otherwise.
///
/// ## Emitting directives
///
/// ### `{#paste $($expr:expr),*$(,)?}`
///
/// Emits each `$expr` as/through [`IntoTokens`], without adjusting [`Span`]s.
///
/// ### `{#raw $($tt:tt)*}`
///
/// More efficiently emits `$($tt)*` verbatim, by [`stringify!`]ing it in bulk but
/// without support for nested directives. If you have long sections of verbatim tokens,
/// using this directive may speed up your build and potentially runtime, even if there's
/// nothing inside that you couldn't emit otherwise.
///
/// ### `{#error $($tt:tt)*}` <sub>uses <code>$root[`::core`]</code></sub>
///
/// Emits a [`compile_error!`]. `$($tt:tt)*` must emit a string literal, optionally followed by a `,`.
///
/// ### `{#root}`
///
/// Pastes a clone of the `$root` given to the initial call.
///
/// ## Context directives
///
/// ### `{#mixed_site $($tt:tt)* }`
///
/// Nested tokens will be resolved with mixed site hygiene and warnings on them will be suppressed.
///
/// (The location for diagnostics remains unchanged.)
///
/// ### `{#call_site $($tt:tt)* }`
///
/// Nested tokens will be resolved with call site hygiene and warnings on them appear to the caller.
///
/// (The location for diagnostics remains unchanged.)
///
/// ### `{#located_at $span2:expr, $($tt:tt)* }`
///
/// Nested tokens will use `$span2`'s location for diagnostics, but keep the outer hygiene scope.
///
/// ### `{#resolved_at $span2:expr, $($tt:tt)* }`
///
/// Nested tokens will use `$span2`'s hygiene scope, but keep the outer location information.
///
/// ### `{#with_exact_span $span:expr, $($tt:tt)* }`
///
/// Nested tokens are emitted exactly with copies of `$span` as [`Span`].
///
/// ## Control flow directives
///
/// ### `{#let $pat:pat = $expr:expr $(, else { $($else:tt)* })?$(;)?}`
///
/// Expands into a `let` binding with optional divergent `else` branch.
///
/// ### `{#break $($label:lifetime)? $($expr:expr)?$(;)?}`
///
/// Expands into a `break` statement with optional label and optional expression.
///
/// ### `{#continue $($label:lifetime)?$(;)?}`
///
/// Expands into a `continue` statement with optional label.
///
/// ### `{#return $($expr:expr)?$(;)?}`
///
/// Expands into a `return` statement with optional expression.
///
/// ### `{# $(else)? if $(let $pat:pat =)? $expr:expr, $($tt:tt)*}`
///
/// Expands into an `if`-statement that conditionally emits nested tokens.
///
/// This may be prefixed by `else`, which makes the `if` itself conditional (see below).
///
/// ### `{#else, $($tt:tt)* }`
///
/// Expands into a fallback branch that emits nested tokens only iff the preceding control
/// flow statement in the same scope was either skipped or skipped its body completely.
///
/// This works after `{#if … }` and as part of an `{#else if … }` chain, but also after
/// conditional loop directives `{#loop, … }` (where it indicates that their body was never entered).
///
/// > It would have been difficult and potentially slow to limit where the directive can appear.
///
/// ### `{#match $expr:expr, … }`
///
/// Expands into a `match` statement (which must be exhaustive). Note that the macro
/// currently only recognises it when the branches are all well-formed too.
///
/// The body of this directive is that of a normal `match` statement (without an extra pair of braces),
/// including the option to use inner attributes on the `match` and outer attributes on the branches,
/// except that branches must use curly braces (`=> { $(tt:tt)* }`) and that tokens inside those braces
/// are interpreted as conditionally emitted nested quote.
///
/// ### `{# $($label:lifetime:)? $(loop)?, $($tt:tt)* }`
///
/// Expands into a block or `loop`-statement with optional label.
///
/// ### `{# $($label:lifetime:)? for $pat:pat in $expr:expr, $($tt:tt)* }`
///
/// Expands into a `for in` loop with optional label.
///
/// ### `{# $($label:lifetime:)? while $(let $pat:pat =)? $expr:expr, $($tt:tt)*}`
///
/// Expands into a `while` or `while let` loop with optional label.
#[macro_export]
macro_rules! quote_into_mixed_site {
	    ($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		    let root: &$crate::__::TokenStream = $root;
		    let tokens = $tokens;
		    let mut enter_else = false;
		    $( $crate::__::quote_one!(span root tokens enter_else, $tt); )*
	    });
}

/// Like [`quote_into_mixed_site!`], but using `$span` directly for quoted tokens.
#[macro_export]
macro_rules! quote_into_with_exact_span {
	    ($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		    let span: $crate::__::Span = $span;
		    let root: &$crate::__::TokenStream = $root;
		    let tokens = $tokens;
		    let mut enter_else = false;
		    $( $crate::__::quote_one!(span root tokens enter_else, $tt); )*
	    });
}

/// Like [`quote_into_mixed_site!`], but resolved according to [`Span::call_site()`].
#[macro_export]
macro_rules! quote_into_call_site {
	    ($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		    let root: &$crate::__::TokenStream = $root;
		    let tokens = $tokens;
		    let mut enter_else = false;
		    $( $crate::__::quote_one!(span root tokens enter_else, $tt); )*
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
	    ($span:expr, $tokens:expr, [$($tt:tt)*$(,)?]) => {{
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		    $crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	    }};
}

/// Like [`raw_quote_into_mixed_site!`], but using `$span` directly for quoted tokens.
#[macro_export]
macro_rules! raw_quote_into_with_exact_span {
	    ($span:expr, $tokens:expr, [$($tt:tt)*$(,)?]) => {
		    $crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	    };
}

/// Like [`raw_quote_into_mixed_site!`], but resolved according to [`Span::call_site()`].
#[macro_export]
macro_rules! raw_quote_into_call_site {
	    ($span:expr, $tokens:expr, [$($tt:tt)*$(,)?]) => {{
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		    $crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	    }};
}

#[doc(hidden)]
pub mod __ {
	#![allow(missing_docs)] // Internal.

	use core::str::FromStr;

	pub use core::{
		clone::Clone, compile_error, concat, iter::Extend, primitive::bool, result::Result,
		stringify,
	};

	use proc_macro2::{Delimiter, Group, Punct, Spacing};

	pub use proc_macro2::{
		Delimiter::{Brace, Bracket, Parenthesis},
		Span, TokenStream, TokenTree,
	};

	pub use crate::quote_one;

	#[doc(hidden)]
	#[macro_export]
	macro_rules! quote_one {
			// #paste
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#paste $($expr:expr),*$(,)?}) => {
			    $( $crate::IntoTokens::into_tokens($expr, $root, $tokens); )*
		    };

			// #raw
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#raw $($tt:tt)*}) => {
			    $crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
		    };

			// #error
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#error $($tt:tt)*}) => {{
				    $crate::IntoTokens::into_tokens($crate::__::Clone::clone($root), $root, $tokens);
				    $crate::__::raw($span, $tokens, "::core::compile_error!");
				    $crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
					    let mut inner_tokens = $crate::__::TokenStream::new();
					    let mut enter_else = false;
					    $( $crate::__::quote_one!($span $root (&mut inner_tokens) enter_else, $tt); )*
					    inner_tokens
				    });
				    $crate::__::raw($span, $tokens, ";");
		    }};

			// #root
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#root}) => {
			    $crate::IntoTokens::into_tokens($crate::__::Clone::clone($root), $root, $tokens);
		    };

			// #mixed_site
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#mixed_site $($tt:tt)*}) => {{
			    let span = $span.resolved_at($crate::__::Span::mixed_site());
			    $( $crate::__::quote_one!(span $root $tokens $enter_else, $tt); )*
		    }};

			// #call_site
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#call_site $($tt:tt)*}) => {{
			    let span = $span.resolved_at($crate::__::Span::call_site());
			    $( $crate::__::quote_one!(span $root $tokens $enter_else, $tt); )*
		    }};

			// #located_at
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#located_at $span2:expr, $($tt:tt)*}) => {{
			    let span = $span.located_at($span2);
			    $( $crate::__::quote_one!(span $root $tokens $enter_else, $tt); )*
		    }};

			// #resolved_at
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#resolved_at $span2:expr, $($tt:tt)*}) => {{
			    let span = $span.resolved_at($span2);
			    $( $crate::__::quote_one!(span $root $tokens $enter_else, $tt); )*
		    }};

			// #with_exact_span
		    ($_span:tt $root:tt $tokens:tt $enter_else:tt, {#with_exact_span $span:expr, $($tt:tt)*}) => {{
			    let span: $crate::__::Span = $span;
			    $( $crate::__::quote_one!(span $root $tokens $enter_else, $tt); )*
		    }};

		    // ($span:tt $root:tt $tokens:tt $enter_else:tt, {#macro $path:path $([
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

			// #let
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#let $pat:pat = $expr:expr $(, else { $($else:tt)* })?$(;)?}) => {
			    let $pat = $expr $(else { $($else)* })?;
		    };

			// #break 'label
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#break $label:lifetime $($expr:expr)?$(;)?}) => {
			    break $label $($expr)?;
		    };

			// #break
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#break $($expr:expr)?$(;)?}) => {
			    break $($expr)?;
		    };

			// #continue
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#continue $($label:lifetime)?$(;)?}) => {
			    continue $($label)?;
		    };

			// #return
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#return $($expr:expr)?$(;)?}) => {
			    return $($expr)?;
		    };

			// #(else )if
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {# $(else $(@ $else:tt)?)? if $(let $pat:pat =)? $expr:expr, $($tt:tt)*}) => {
			    // Handles both `#if` and `#else if`.
			    if true $(&& $enter_else $(@ $else)?)? {
				    if $(let $pat =)? $expr {
					    $enter_else = false;
					    let mut enter_else = false;
					    $( $crate::__::quote_one!($span $root $tokens enter_else, $tt); )*
				    } else {
					    $enter_else = true;
				    }
			    }
		    };

			// #else
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#else, $($tt:tt)*}) => {
			    if $enter_else {
				    $enter_else = false;
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root $tokens enter_else, $tt); )*
			    }
		    };

			// #match
		    //TODO: Better error handling for this specifically, since it matches the match arms.
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#match $expr:expr,
			    $(#![$($match_attr:tt)*])*
			    $(
				    $(#[$($arm_attr:tt)*])*
				    $pat:pat $(if $arm_expr:expr)? => {
					    $($tt:tt)*
				    }
			    )*
		    }) => {
			    $enter_else = false;
			    let mut enter_else = false;
			    match $expr {
				    $(#![$($match_attr)*])*
				    $(
					    $(#[$($arm_attr)*])*
					    $pat $(if $arm_expr)? => {
						    $( $crate::__::quote_one!($span $root $tokens enter_else, $tt); )*
					    }
				    )*
			    }
		    };

			// block expansion and #loop
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#$($label:lifetime:)? $(loop $(@ $loop:tt)?)?, $($tt:tt)*}) => {
			    // Handles both blocks and unconditional loops.
			    $enter_else = false;
			    $($label:)? $(loop $(@ $loop)?)? {
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root $tokens enter_else, $tt); )*
			    }
		    };

			// #for in
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#$($label:lifetime:)? for $pat:pat in $expr:expr, $($tt:tt)*}) => {
			    $enter_else = true;
			    $($label:)? for $pat in $expr {
				    $enter_else = false;
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root $tokens $enter_else, $tt); )*
			    }
		    };

			// #while
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#$($label:lifetime:)? while $(let $pat:pat = )?$expr:expr, $($tt:tt)*}) => {
			    $enter_else = true;
			    $($label:)? while $(let $pat = )?$expr {
				    $enter_else = false;
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root $tokens $enter_else, $tt); )*
			    }
		    };

		    //TODO: Error handling with syntax help, about here in the pattern order.

			// reserved `#identifier`
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#$reserved:ident $($tt:tt)*}) => {
			    $crate::__::compile_error!($crate::__::concat!("`{#", $crate::__::stringify!($reserved), "… }` is either reserved within Loess's quotes or its pattern wasn't matched. (Did you mean `{#paste ", $crate::__::stringify!($reserved), "… }` or `{#, #", $crate::__::stringify!($reserved), "… }`?)"));
		    };

			// reserved `#'lifetime`
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, {#$reserved:lifetime $($tt:tt)*}) => {
			    $crate::__::compile_error!($crate::__::concat!("`{#", $crate::__::stringify!($reserved), "… }` is either reserved within Loess's quotes or its pattern wasn't matched. (Did you mean `{#", $crate::__::stringify!($reserved), ":, … }` or `{#", $crate::__::stringify!($reserved), ": for … in …, … }`?)"));
		    };

			// {}
		    ($span:tt $root:tt $tokens:tt $_enter_else:tt, {$($tt:tt)*}) => {
			    $crate::__::grouped($span, $crate::__::Brace, $tokens, {
				    let mut inner_tokens = $crate::__::TokenStream::new();
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root (&mut inner_tokens) enter_else, $tt); )*
				    inner_tokens
			    });
		    };

			// []
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, [$($tt:tt)*]) => {
			    $crate::__::grouped($span, $crate::__::Bracket, $tokens, {
				    let mut inner_tokens = $crate::__::TokenStream::new();
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root (&mut inner_tokens) enter_else, $tt); )*
				    inner_tokens
			    });
		    };

			// ()
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, ($($tt:tt)*)) => {
			    $crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
				    let mut inner_tokens = $crate::__::TokenStream::new();
				    let mut enter_else = false;
				    $( $crate::__::quote_one!($span $root (&mut inner_tokens) enter_else, $tt); )*
				    inner_tokens
			    });
		    };

			// other tokens
		    ($span:tt $root:tt $tokens:tt $enter_else:tt, $other:tt) => (
			    // Fortunately `'` can't arrive as trailing punctuation inside `$other` here (It's always either a literal or inside a lifetime),
			    // so it's at least reasonable to expect `stringify!` to insert a space iff the trailing punctuation is spaced and otherwise not.
			    $crate::__::tt($span, $tokens, const { $crate::__::strip_dot($crate::__::stringify!($other .)) } );
		    );

			// End.
		    ($span:tt $root:tt $tokens:tt $enter_else:tt,) => {};
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
		if let TokenTree::Punct(trailing_punct) = ts.last_mut().expect(
			"always at least one token (Please don't use the internal API from other crates.)",
		) {
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
}
