/// Defines keywords and identifiers (excluding lifetimes).
///
/// # Examples
///
/// ```rust
/// loess::words! {
/// 	// Defines a keyword.
/// 	keyword as Keyword;
///
/// 	// Defines a catch-other identifier. Must be last.
/// 	_ as Identifier;
/// }
/// ```
///
/// ```rust
/// loess::words! {
/// 	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// 	pub keyword as pub Keyword: doc, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt;
///
/// 	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// 	pub _ as pub Identifier: PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt;
/// }
/// ```
///
/// ## Exclude keywords without declaration
///
/// ```rust
/// loess::words! {
/// 	// Exclude keyword for catch-other identifier.
/// 	// Must not have otherwise optional elements.
/// 	keyword as _;
///
/// 	// Matches any except `keyword`.
/// 	pub _ as pub Identifier;
/// }
/// ```
///
/// ```rust,compile_fail
/// loess::words! {
/// 	keyword as _; //Error: Unused keyword exclusion: keyword
/// }
/// ```
///
/// ### For macros
///
/// ```rust
/// loess::words! {
/// 	keyword as _;
/// 	allow_prior_unused!;
/// }
/// ```
///
/// ```rust,compile_fail
/// loess::words! {
/// 	allow_prior_unused!;
/// 	keyword as _;
/// }
/// ```
#[macro_export]
macro_rules! words {
	($($input:tt)*) => {
		$crate::__words_muncher!([] [] $($input)*);
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __words_muncher {
	// Keyword.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]

		$(#[$($attr:tt)*])*
		$ident_vis:vis $kw:ident as $vis:vis $name:ident
		$(: $(
			$(doc $(@ $doc:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
			$(SimpleSpanned $(@ $SimpleSpanned:tt)?)?
			$(LocatedAt $(@ $LocatedAt:tt)?)?
			$(ResolvedAt $(@ $ResolvedAt:tt)?)?
		),*$(,)?)?
		;

		$($rest:tt)*
	) => {
		$($($(
			$(@ $doc)?
			#[doc = $crate::__::concat!('`', stringify!($kw), '`')]
		)?)*)?
		$(#[$($attr)*])*
		$vis struct $name($ident_vis $crate::__::Ident);

		$($(
			$(
				$(@ $PeekFrom)?
				$crate::__impl_word!(PeekFrom for $name, ident => ident == stringify!($kw));
			)?
			$(
				$(@ $PopFrom)?
				$crate::__impl_word!(
					PopFrom for $name,
					ident => ident == stringify!($kw),
					$crate::__::concat!("Expected `", stringify!($kw), "`."),
				);
			)?
			$(
				$(@ $IntoTokens)?
				$crate::__impl_word!(IntoTokens for $name);
			)?
			$(
				$(@ $SimpleSpanned)?
				$crate::__impl_word!(SimpleSpanned for $name);
			)?
			$(
				$(@ $LocatedAt)?
				$crate::__impl_word!(LocatedAt for $name);
			)?
			$(
				$(@ $ResolvedAt)?
				$crate::__impl_word!(ResolvedAt for $name);
			)?
		)*)?

		$crate::__words_muncher! {
			[$($kws)* $kw] [$($excluded_kws)*]
			$($rest)*
		}
	};

	// Exclude keyword.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]
		$kw:ident as _;
		$($rest:tt)*
	) => {
		$crate::__words_muncher! {
			[$($kws)*] [$($excluded_kws)* $kw]
			$($rest)*
		}
	};

	// Other identifier. Final.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]

		$(#[$($attr:tt)*])*
		$ident_vis:vis _ as $vis:vis $name:ident
		$(: $(
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
			$(SimpleSpanned $(@ $SimpleSpanned:tt)?)?
			$(LocatedAt $(@ $LocatedAt:tt)?)?
			$(ResolvedAt $(@ $ResolvedAt:tt)?)?
		),*$(,)?)?
		;

		$($($rest:tt)+)?
	) => {
		$(#[$($attr)*])*
		$vis struct $name($ident_vis $crate::__::Ident);

		const _: () = {
			#[allow(non_upper_case_globals)]
			const __LOESS__WORDS_EXCLUSIONS: &[&str] = &[
				$(stringify!($kws),)*
				$(stringify!($excluded_kws),)*
			];

			$($(
				$(
					$(@ $PeekFrom)?
					$crate::__impl_word!(
						PeekFrom for $name,
						ident => __LOESS__WORDS_EXCLUSIONS.into_iter().copied().all(|kw| ident != kw),
					);
				)?
				$(
					$(@ $PopFrom)?
					$crate::__impl_word!(
						PopFrom for $name,
						ident => __LOESS__WORDS_EXCLUSIONS.into_iter().copied().all(|kw| ident != kw),
						$crate::__::concat!("Expected ", stringify!($name), "."),
					);
				)?
				$(
					$(@ $IntoTokens)?
					$crate::__impl_word!(IntoTokens for $name);
				)?
				$(
					$(@ $SimpleSpanned)?
					$crate::__impl_word!(SimpleSpanned for $name);
				)?
				$(
					$(@ $LocatedAt)?
					$crate::__impl_word!(LocatedAt for $name);
				)?
				$(
					$(@ $ResolvedAt)?
					$crate::__impl_word!(ResolvedAt for $name);
				)?
			)*)?
		};

		$($crate::__::compile_error!($crate::__::concat!("Catch-other identifier must be last, but was followed by: ", $crate::__::stringify!($($rest)+)));)?
	};

	// For macro authors.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]
		allow_prior_unused!;
		$($rest:tt)*
	) => {
		$crate::__words_muncher! {
			[$($kws)* $($excluded_kws)*] []
			$($rest)*
		}
	};

	// Other end.
	([$($kws:tt)*] [$($excluded_kws:tt)*]) => {
		$(
			$crate::__::compile_error!($crate::__::concat!("Unused keyword exclusion: ", $crate::__::stringify!($excluded_kws)));
		)*
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_word {
	(PeekFrom for $name:ty, $ident:ident => $condition:expr$(,)?) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> bool {
				input.peek(|tts, _| matches!(tts, [$crate::__::TokenTree::Ident($ident)] if $condition && !<$crate::__::Ident as $crate::__::ToString>::to_string($ident).as_str().starts_with('\'')))
			}
		}
	};

	(PopFrom for $name:ty, $ident:ident => $condition:expr, $message:expr$(,)?) => {
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;

			fn pop_parsed_from(
				input: &mut $crate::Input,
				errors: &mut $crate::Errors,
			) -> $crate::__::ControlFlow<$crate::__::Option<Self>, $crate::__::Option<Self>> {
				input
					.pop_or_replace(|tts, _| match tts {
						[$crate::__::TokenTree::Ident($ident)] if $condition && !<$crate::__::Ident as $crate::__::ToString>::to_string(&$ident).as_str().starts_with('\'') => Ok(Self($ident)),
						tts => Err(tts),
					})
					.map_continue(Some)
					.map_break(|spans| {
						errors.push($crate::Error::new(
							$crate::ErrorPriority::TOKEN,
							$message,
							spans,
						));
						None
					})
			}
		}
	};

	(IntoTokens for $name:ty$(,)?) => {
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				self.0.into_tokens(root, tokens)
			}
		}
	};

	(SimpleSpanned for $name:ty$(,)?) => {
		impl $crate::SimpleSpanned for $name {
			fn span(&self) -> $crate::__::Span {
				self.0.span()
			}

			fn set_span(&mut self, span: $crate::__::Span) {
				self.0.set_span(span)
			}
		}
	};

	(LocatedAt for $name:ty$(,)?) => {
		impl $crate::LocatedAt for $name {
			fn located_at(mut self, span: $crate::__::Span) -> Self {
				self.0.set_span(self.0.span().located_at(span));
				self
			}
		}
	};

	(ResolvedAt for $name:ty$(,)?) => {
		impl $crate::ResolvedAt for $name {
			fn resolved_at(mut self, span: $crate::__::Span) -> Self {
				self.0.set_span(self.0.span().resolved_at(span));
				self
			}
		}
	};
}
