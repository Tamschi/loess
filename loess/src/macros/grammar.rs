/// Parser- and serialiser-generator macro.
///
/// # Example
///
/// ```
/// use loess::{
/// 	grammar, words,
/// 	scaffold::{Parentheses, SquareBrackets},
/// };
/// use proc_macro2::{Ident, TokenTree, Punct};
///
/// words! {
/// 	#[derive(Clone)]
/// 	pub let as Let: doc, PeekFrom, PopFrom, IntoTokens;
///
/// 	#[derive(Clone)]
/// 	pub pub as Pub: doc, PeekFrom, PopFrom, IntoTokens;
///
/// 	#[derive(Clone)]
/// 	pub _ as Identifier: PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt;
/// }
///
/// grammar! {
/// 	#[derive(Clone)]
/// 	pub struct Visibility: PeekFrom, PopFrom, IntoTokens {
/// 		pub r#pub: Pub,
/// 		parens: Option<Parentheses>,
/// 	}
///
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
#[macro_export]
macro_rules! grammar {
	//TODO: Change impl separator to `+`?
	//TODO: Return placeholder if the last field did.

	// enum {}
	{
		$(#[$($attr:tt)*])*
		$vis:vis enum $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)? $(via $PeekFromViaType:ident)?)?
			$(PopFrom $(@ $PopFrom:tt)? $(via $PopFromViaType:ident)?)?
			$(IntoTokens $(@ $IntoTokens:tt)? $(via $IntoTokensViaType:ident)?)?
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
		$crate::grammar!(@PeekFrom for enum $name $($($($(via $PeekFromViaType)?)?)*)?, [$([$($type),*]),*]);

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		$crate::grammar!(@PopFrom for enum $name $($($($(via $PopFromViaType)?)?)*)?, [$($variant[$($type),*]),*], $error);

		#[cfg(any($($($(all(), $(@ $IntoTokens)?)?)?)*))]
		$crate::grammar!(@IntoTokens for enum $name $($($($(via $IntoTokensViaType)?)?)*)?, [$($variant[$($type),*]),*], $error);

		$crate::grammar!($($tt)*);
	};

	// struct {}
	{
		$(#[$($attr:tt)*])*
		$vis:vis struct $name:ident$(: $(
			$(PeekFrom $(@ $PeekFrom:tt)? $(via $PeekFromViaType:ident)?)?
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
		$crate::grammar!(@PeekFrom for struct $name $($($($(via $PeekFromViaType)?)?)*)?, $($type),*);

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, $crate::__::Option<Self>> {
				let mut failed = false;
				let this = Self {
					$($field: <$type as $crate::PopParsedFrom>::pop_parsed_from(input, errors).or_else(|fallback| fallback.map_or($crate::__::Err($crate::__::None), |e| { failed = true; $crate::__::Ok(e) }))?,)*
				};
				(if failed { |e| $crate::__::Result::Err($crate::__::Some(e)) } else { $crate::__::Result::Ok })(this)
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

	// struct ()
	{
		$(#[$($attr:tt)*])*
		$vis:vis struct $name:ident$(: $(
			$(PeekFrom $(@ $PeekFrom:tt)? $(via $PeekFromViaType:ident)?)?
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
		$crate::grammar!(@PeekFrom for struct $name $($($($(via $PeekFromViaType)?)?)*)?, $($type),*);

		#[cfg(any($($($(all(), $(@ $PopFrom)?)?)?)*))]
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, $crate::__::Option<Self>> {
				let mut failed = false;
				let this = Self (
					$(<$type as $crate::PopParsedFrom>::pop_parsed_from(input, errors).or_else(|fallback| fallback.map_or($crate::__::Err($crate::__::None), |e| { failed = true; $crate::__::Ok(e) }))?,)*
				);
				(if failed { |e| $crate::__::Result::Err($crate::__::Some(e)) } else { $crate::__::Result::Ok })(this)
			}
		}

		$crate::grammar!($($tt)*);
	};

	(@PeekFrom for enum $name:ident, [$([$($type:ty),*$(,)?]),*$(,)?]$(,)?) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				false
				$(|| $crate::grammar!(@peek_first $name input $($type,)*))*
			}
		}
	};
	(@PeekFrom for struct $name:ident, $($type:ty),*$(,)?) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				$crate::grammar!(@peek_first $name input $($type,)*)
			}
		}
	};
	(@PeekFrom for $_either:tt $name:ident via $PeekFromViaType:ident, $($_ignored:tt)*) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				<$PeekFromViaType as $crate::PeekFrom>::peek_from(input)
			}
		}
	};

	(@PopFrom for enum $name:ident, [$($variant:ident[$($type:ty),*$(,)?]),*$(,)?], $error:expr$(,)?) => {
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, $crate::__::Option<Self>> {
				$( $crate::grammar!(@PopFromVariantBranch(input, errors) for $variant[$($type),*]); )*
				{
					errors.push($crate::Error::new(
						$crate::ErrorPriority::GRAMMAR,
						$error,
						[input.front_span()],
					));
					return $crate::__::Result::Err($crate::__::None);
				}
			}
		}
	};
	(@PopFromVariantBranch($input:ident) for $variant:ident[]) => ( return $crate::__::Result::Ok(Self::$variant()); );
	(@PopFromVariantBranch($input:ident, $errors:ident) for $variant:ident[$type_0:ty$(, $($types_rest:ty),*$(,)?)?]) => {
		if let Some(value_0) = <$type_0 as $crate::PopParsedFrom>::peek_pop_parsed_from($input, $errors).map_err(|_| $crate::__::None)? {
			return $crate::__::Result::Ok(Self::$variant(value_0$(, $(<$types_rest as $crate::PopParsedFrom>::pop_parsed_from($input, $errors).map_err(|_| $crate::__::None)?),*)?));
		}
	};

	//TODO
	(@PopFrom for struct $name:ident, $($type:ty),*$(,)?) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> $crate::__::bool {
				$crate::grammar!(@peek_first $name input $($type,)*)
			}
		}
	};
	(@PopFrom for $_either:tt $name:ident via $PopFromViaType:ident, $($_ignored:tt)*) => {
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, $crate::__::Option<Self>> {
				$crate::__::Result::Ok(
					<Self as $crate::__::From<<$PopFromViaType as $crate::PopParsedFrom>::Parsed>>::from(
						<$PopFromViaType as $crate::PopParsedFrom>::pop_parsed_from(input, errors).map_err(|_| $crate::__::None)?,
					),
				)
			}
		}
	};

	(@IntoTokens for enum $name:ident, [$($variant:ident[$($type:ty),*$(,)?]),*$(,)?], $error:expr$(,)?) => {
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				todo!("grammar!(@IntoTokens for enum $name:ident, …)")
			}
		}
	};
	(@IntoTokens for $_either:tt $name:ident via $IntoTokensViaType:ident, $($_ignored:tt)*) => {
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				$crate::__::Result::Ok(
					<$IntoTokensViaType as $crate::PopParsedFrom>::into_tokens(
						<$IntoTokensViaType as $crate::__::From<Self>>::from(self),
						root,
						tokens,
					),
				)
			}
		}
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
