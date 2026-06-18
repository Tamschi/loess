use std::{
	convert::identity,
	ops::ControlFlow::{self, Break, Continue},
};

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
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::ControlFlow<$crate::__::Option<Self>, $crate::__::Option<Self>> {
				$crate::__::PopFromAccumulator::new()
					$( .step(|| <$type as $crate::PopParsedFrom>::pop_parsed_from(input, errors))? )*
					.map(|$($field),*| Self { $($field),* })
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
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::ControlFlow<$crate::__::Option<Self>, $crate::__::Option<Self>> {
				$crate::__::PopFromAccumulator::new()
					$( .step(||<$type as $crate::PopParsedFrom>::pop_parsed_from(input, errors))? )*
					.map(Self)
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
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::ControlFlow<$crate::__::Option<Self>, $crate::__::Option<Self>> {
				$( $crate::grammar!(@PopFromVariantBranch(input, errors) for $variant[$($type),*]); )*
				{
					errors.push($crate::Error::new(
						$crate::ErrorPriority::GRAMMAR,
						$error,
						[input.front_span()],
					));
					return $crate::__::Break($crate::__::None);
				}
			}
		}
	};
	(@PopFromVariantBranch($input:ident) for $variant:ident[]) => ( return $crate::__::Result::Ok(Self::$variant()); );
	(@PopFromVariantBranch($input:ident, $errors:ident) for $variant:ident[$type_0:ty$(, $($types_rest:ty),*$(,)?)?]) => {
		if <$type_0 as $crate::PeekFrom>::peek_from($input) {
			return $crate::__::PopFromAccumulator::new()
				.step(|| <$type_0 as $crate::PopParsedFrom>::pop_parsed_from($input, $errors))?
				$($( .step(||<$types_rest as $crate::PopParsedFrom>::pop_parsed_from($input, $errors))? )*)?
				.map(Self::$variant)
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
			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::ControlFlow<$crate::__::Option<Self>, $crate::__::Option<Self>> {
				$crate::__::Continue(
					<$PopFromViaType as $crate::PopParsedFrom>::pop_parsed_from(input, errors).map_break(|_| $crate::__::None)?
						.map(<Self as $crate::__::From<<$PopFromViaType as $crate::PopParsedFrom>::Parsed>>::from),
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

pub struct PopFromAccumulator<Acc> {
	aligned: bool,
	output: Option<Acc>,
}

impl PopFromAccumulator<()> {
	pub fn new() -> Self {
		Self {
			aligned: true,
			output: Some(()),
		}
	}
}

impl<Acc> PopFromAccumulator<Acc> {
	pub fn step<T, Parsed>(
		self,
		step: impl FnOnce() -> ControlFlow<Option<T>, Option<T>>,
	) -> ControlFlow<Option<Parsed>, PopFromAccumulator<(Acc, T)>> {
		if self.aligned {
			let step = step();
			Continue(PopFromAccumulator {
				aligned: step.is_continue(),
				output: self.output.zip(step.continue_ok().unwrap_or_else(identity)),
			})
		} else {
			Break(None)
		}
	}
}

macro_rules! TupleChain {
	([$acc:tt]) => ($acc);
	([$acc:tt] $next:tt $($rest:tt)*) => (TupleChain!([($acc, $next)] $($rest)*));
	($($rest:tt)*) => (TupleChain!([()] $($rest)*));
}

macro_rules! closure_chain {
	($f:tt [$acc:tt] [$($tt:tt)*]) => (|$acc| $f($($tt),*));
	($f:tt [$acc:tt] [$($tt:tt)*] $next:tt $($rest:tt)*) => (closure_chain!($f [($acc, $next)] [$($tt)* $next] $($rest)*));
	($f:tt $($rest:tt)*) => (closure_chain!($f [()] [] $($rest)*));
}

macro_rules! impl_map {
	([$($tt:tt)*]) => {
		impl<$($tt),*> PopFromAccumulator<TupleChain!($($tt)*)> {
			pub fn map<Parsed>(
				self,
				f: impl FnOnce($($tt),*) -> Parsed,
			) -> ControlFlow<Option<Parsed>, Option<Parsed>> {
				(if self.aligned {
					Continue
				} else {
					Break
				})(self.output.map({
					#[allow(non_snake_case)]
					{ closure_chain!(f $($tt)*) }
				}))
			}
		}
	};
	([$($tt:tt)*] $next:tt $($rest:tt)*) => {
		impl_map!([$($tt)*]);
		impl_map!([$($tt)* $next] $($rest)*);
	};
	($($rest:tt)*) => {
		impl_map!([] $($rest)*);
	};
}

impl_map!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23);
