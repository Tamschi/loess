#[cfg(doc)]
use proc_macro2::Span;

#[cfg(doc)]
use crate::{IntoTokens, grammar, raw_quote_into_mixed_site};

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

/// Simple generic quotation macro that works well with Loess's types.
///
/// //TODO: Document parameters.
/// //TODO: Document snippets.
///
/// Uses `{#identifier … }`-style directives.
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
/// ## Directives
///
/// //TODO: Update and correct!
///
/// ### `{#paste $($expr:expr),*$(,)?}`
///
/// Emits each `$expr` as/through [`IntoTokens`].
///
/// ### `{#if $expr:expr, $($tt:tt)* }`<br>`{#if let $pattern:pat = $expr:expr, $($tt:tt)* }`<br>`{#for $pattern:pat in $span:expr, $($tt:tt)* }`<br>`{#while $expr:expr, $($tt:tt)* }`<br>`{#while let $pattern:pat = $expr:expr, $($tt:tt)* }`<br>`{#loop $($tt:tt)* }`<br>`{#break $($expr:expr)? }`
///
/// Expand into flow control statements.
///
/// ### `{#else $($tt:tt)* }`
///
/// Expands into an `else`-branch.
///
/// ### `{#root}`
///
/// Pastes a clone of the `$root` given to the initial call.
///
/// ### `{#hash $($tt:tt)* }`
///
/// Emits `{#` … `}`.
#[macro_export]
macro_rules! quote_into_mixed_site {
	    ($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		    let root: &$crate::__::TokenStream = $root;
		    let tokens = $tokens;
		    let mut not_if = false;
		    $( $crate::__::quote_one!(span root tokens not_if, $tt); )*
	    });
}

/// Like [`quote_into_mixed_site!`], but resolved according to `$span`.
#[macro_export]
macro_rules! quote_into_same_site {
	    ($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		    let span: $crate::__::Span = $span;
		    let root: &$crate::__::TokenStream = $root;
		    let tokens = $tokens;
		    let mut not_if = false;
		    $( $crate::__::quote_one!(span root tokens not_if, $tt); )*
	    });
}

/// Like [`quote_into_mixed_site!`], but resolved according to [`Span::call_site()`].
#[macro_export]
macro_rules! quote_into_call_site {
	    ($span:expr, $root:expr, $tokens:expr, [$($tt:tt)*]$(,)?) => ({
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::call_site());
		    let root: &$crate::__::TokenStream = $root;
		    let tokens = $tokens;
		    let mut not_if = false;
		    $( $crate::__::quote_one!(span root tokens not_if, $tt); )*
	    });
}

/// Simple generic quotation macro that efficiently emits tokens verbatim.
///
/// # Parameters
///
/// ## `$span`: [`Span`]
///
/// A `Span` that controls which part of the input errors are reported on and which
/// hygiene context certain identifiers are resolved with. In most cases, you should use
/// an as-specific-as-possible `Span` from your macro input here, so that the user of your
/// macro will have an easier time solving issues.
///
/// [`raw_quote_into_mixed_site!`] automatically uses `mixed_site` resolution on quoted
/// tokens (but not pasted [`IntoTokens`] values!). This isolates resolution for scoped
/// bindings (but not items, so please use fully qualified paths and ideally the `$crate`-
/// `root` pattern from Loess's README that can be viewed [in the root module](crate).)
#[macro_export]
macro_rules! raw_quote_into_mixed_site {
	    ($span:expr, $tokens:expr, [$($tt:tt)*$(,)?]) => {{
		    let span: $crate::__::Span = $span.resolved_at($crate::__::Span::mixed_site());
		    $crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
	    }};
}

/// Like [`raw_quote_into_mixed_site!`], but resolved according to `$span`.
#[macro_export]
macro_rules! raw_quote_into_same_site {
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
		    //TODO: Missing directives.
		    //TODO: Error handling with syntax help.
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#paste $($expr:expr),*$(,)?}) => {
			    $( $crate::IntoTokens::into_tokens($expr, $root, $tokens); )*
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#raw $($tt:tt)*}) => {
			    $crate::__::raw($span, $tokens, $crate::__::stringify!($($tt)*));
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#error $($tt:tt)*}) => {{
				    $crate::IntoTokens::into_tokens($crate::__::Clone::clone($root), $root, $tokens);
				    $crate::__::raw($span, $tokens, "::core::compile_error!");
				    $crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
					    let mut inner_tokens = $crate::__::TokenStream::new();
					    let mut not_if = false;
					    $( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
					    inner_tokens
				    });
				    $crate::__::raw($span, $tokens, ";");
		    }};
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#root}) => {
			    $crate::IntoTokens::into_tokens($crate::__::Clone::clone($root), $root, $tokens);
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#mixed_site $($tt:tt)*}) => {{
			    let span = $span.resolved_at($crate::__::Span::mixed_site());
			    $( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		    }};
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#call_site $($tt:tt)*}) => {{
			    let span = $span.resolved_at($crate::__::Span::call_site());
			    $( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		    }};
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#located_at $span2:expr, $($tt:tt)*}) => {{
			    let span = $span.located_at($span2);
			    $( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		    }};
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#resolved_at $span2:expr, $($tt:tt)*}) => {{
			    let span = $span.resolved_at($span2);
			    $( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		    }};
		    ($_span:tt $root:tt $tokens:tt, {#with_exact_span $span:expr, $($tt:tt)*}) => {{
			    let span: $crate::__::Span = $span;
			    $( $crate::__::quote_one!(span $root $tokens $not_if, $tt); )*
		    }};
		    // ($span:tt $root:tt $tokens:tt $not_if:tt, {#macro $path:path $([
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
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#let $pat:pat = $expr:expr $(, else { $($else:tt)* })?$(;)?}) => {
			    let $pat = $expr;
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#break $label:lifetime $($expr:expr)?$(;)?}) => {
			    break $label $($expr)?;
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#break $($expr:expr)?$(;)?}) => {
			    break $($expr)?;
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#continue $($label:lifetime)?$(;)?}) => {
			    continue $($label)?;
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#return $($expr:expr)?$(;)?}) => {
			    return $($expr)?;
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {# $(else $(@ $else:tt)?)? if $(let $pat:pat =)? $expr:expr, $($tt:tt)*}) => {
			    // Handles both `#if` and `#else if`.
			    if true $(&& $not_if $(@ $else)?)? {
				    if $(let $pat =)? $expr {
					    $not_if = false;
					    let mut not_if = false;
					    $( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
				    } else {
					    $not_if = true;
				    }
			    }
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#else, $($tt:tt)*}) => {
			    if $not_if {
				    $not_if = false;
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
			    }
		    };

		    //TODO: Better error handling for this specifically, since it matches the match arms.
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#match $expr:expr,
			    $(#![$($match_attr:tt)*])*
			    $(
				    $(#[$($arm_attr:tt)*])*
				    $pat:pat $(if $arm_expr:expr)? => {
					    $($tt:tt)*
				    }
			    )*
		    }) => {
			    $not_if = false;
			    let mut not_if = false;
			    match $expr {
				    $(#![$($match_attr)*])*
				    $(
					    $(#[$($arm_attr)*])*
					    $pat $(if $arm_expr)? => {
						    $( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
					    }
				    )*
			    }
		    };

		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#$($label:lifetime:)? $(loop $(@ $loop:tt)?)?, $($tt:tt)*}) => {
			    // Handles both blocks and unconditional loops.
			    $not_if = false;
			    $($label:)? $(loop $(@ $loop)?)? {
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root $tokens not_if, $tt); )*
			    }
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#$($label:lifetime:)? for $pat:pat in $expr:expr, $($tt:tt)*}) => {
			    $not_if = true;
			    $($label:)? for $pat in $expr {
				    $not_if = false;
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root $tokens $not_if, $tt); )*
			    }
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#$($label:lifetime:)? while $(let $pat:pat = )?$expr:expr, $($tt:tt)*}) => {
			    $not_if = true;
			    $($label:)? while $(let $pat = )?$expr {
				    $not_if = false;
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root $tokens $not_if, $tt); )*
			    }
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#$reserved:ident $($tt:tt)*}) => {
			    $crate::__::compile_error!($crate::__::concat!("`{#", $crate::__::stringify!($reserved), "… }` is reserved within Loess's quotes. (Did you mean `{#paste ", $crate::__::stringify!($reserved), "… }` or `{#, #", $crate::__::stringify!($reserved), "… }`?)"));
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, {#$reserved:lifetime $($tt:tt)*}) => {
			    $crate::__::compile_error!($crate::__::concat!("`{#", $crate::__::stringify!($reserved), "… }` is reserved within Loess's quotes. (Did you mean `{#", $crate::__::stringify!($reserved), ":, … }` or `{#", $crate::__::stringify!($reserved), ": for … in …, … }`?)"));
		    };
		    ($span:tt $root:tt $tokens:tt $_not_if:tt, {$($tt:tt)*}) => {
			    $crate::__::grouped($span, $crate::__::Brace, $tokens, {
				    let mut inner_tokens = $crate::__::TokenStream::new();
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
				    inner_tokens
			    });
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, [$($tt:tt)*]) => {
			    $crate::__::grouped($span, $crate::__::Bracket, $tokens, {
				    let mut inner_tokens = $crate::__::TokenStream::new();
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
				    inner_tokens
			    });
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, ($($tt:tt)*)) => {
			    $crate::__::grouped($span, $crate::__::Parenthesis, $tokens, {
				    let mut inner_tokens = $crate::__::TokenStream::new();
				    let mut not_if = false;
				    $( $crate::__::quote_one!($span $root (&mut inner_tokens) (&mut not_if), $tt); )*
				    inner_tokens
			    });
		    };
		    ($span:tt $root:tt $tokens:tt $not_if:tt, $other:tt) => (
			    // Fortunately `'` can't arrive as trailing punctuation inside `$other` here (It's always either a literal or inside a lifetime),
			    // so it's at least reasonable to expect `stringify!` to insert a space iff the trailing punctuation is spaced and otherwise not.
			    $crate::__::tt($span, $tokens, const { $crate::__::strip_dot($crate::__::stringify!($other .)) } );
		    );
		    ($span:tt $root:tt $tokens:tt $not_if:tt,) => {}; // End.
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
