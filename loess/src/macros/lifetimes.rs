/// Defines key and catch-other lifetimes.
///
/// # Examples
///
/// ```rust
/// loess::lifetimes! {
/// 	// Defines a key lifetime.
/// 	('key) as LifetimeKey;
///
/// 	// Defines a catch-other lifetime. Must be last.
/// 	_ as Lifetime;
/// }
/// ```
///
/// ```rust
/// loess::lifetimes! {
/// 	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// 	pub ('key) as pub LifetimeKey: doc, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt;
///
/// 	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// 	pub _ as pub Lifetime: PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt;
/// }
/// ```
///
/// ## Exclude key lifetimes without declaration
///
/// ```rust
/// loess::lifetimes! {
/// 	// Exclude 'key for catch-other lifetime.
/// 	// Must not have otherwise optional elements.
/// 	('key) as _;
///
/// 	// Matches any except `'key`.
/// 	pub _ as pub Lifetime;
/// }
/// ```
///
/// ```rust,compile_fail
/// loess::lifetimes! {
/// 	('key) as _; //Error: Unused key lifetime exclusion: 'key
/// }
/// ```
///
/// ### For macros
///
/// ```rust
/// loess::lifetimes! {
/// 	('key) as _;
/// 	allow_prior_unused!;
/// }
/// ```
///
/// ```rust,compile_fail
/// loess::lifetimes! {
/// 	allow_prior_unused!;
/// 	('key) as _;
/// }
/// ```
#[macro_export]
macro_rules! lifetimes {
	($($input:tt)*) => {
		$crate::__lifetimes_muncher!([] [] $($input)*);
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __lifetimes_muncher {
	// Keylifetime.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]

		$(#[$($attr:tt)*])*
		$ident_vis:vis ($kw:lifetime) as $vis:vis $name:ident
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
				$crate::__impl_lifetime!(PeekFrom for $name, ident => ident == stringify!($kw));
			)?
			$(
				$(@ $PopFrom)?
				$crate::__impl_lifetime!(
					PopFrom for $name,
					ident => ident == stringify!($kw),
					$crate::__::concat!("Expected `", stringify!($kw), "`."),
				);
			)?
			$(
				$(@ $IntoTokens)?
				$crate::__impl_lifetime!(IntoTokens for $name);
			)?
			$(
				$(@ $SimpleSpanned)?
				$crate::__impl_lifetime!(SimpleSpanned for $name);
			)?
			$(
				$(@ $LocatedAt)?
				$crate::__impl_lifetime!(LocatedAt for $name);
			)?
			$(
				$(@ $ResolvedAt)?
				$crate::__impl_lifetime!(ResolvedAt for $name);
			)?
		)*)?

		$crate::__lifetimes_muncher! {
			[$($kws)* $kw] [$($excluded_kws)*]
			$($rest)*
		}
	};

	// Exclude key lifetime.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]
		($kw:lifetime) as _;
		$($rest:tt)*
	) => {
		$crate::__lifetimes_muncher! {
			[$($kws)*] [$($excluded_kws)* $kw]
			$($rest)*
		}
	};

	// Other lifetime. Final.
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
					$crate::__impl_lifetime!(
						PeekFrom for $name,
						ident => __LOESS__WORDS_EXCLUSIONS.into_iter().copied().all(|kw| ident != kw),
					);
				)?
				$(
					$(@ $PopFrom)?
					$crate::__impl_lifetime!(
						PopFrom for $name,
						ident => __LOESS__WORDS_EXCLUSIONS.into_iter().copied().all(|kw| ident != kw),
						$crate::__::concat!("Expected ", stringify!($name), "."),
					);
				)?
				$(
					$(@ $IntoTokens)?
					$crate::__impl_lifetime!(IntoTokens for $name);
				)?
				$(
					$(@ $SimpleSpanned)?
					$crate::__impl_lifetime!(SimpleSpanned for $name);
				)?
				$(
					$(@ $LocatedAt)?
					$crate::__impl_lifetime!(LocatedAt for $name);
				)?
				$(
					$(@ $ResolvedAt)?
					$crate::__impl_lifetime!(ResolvedAt for $name);
				)?
			)*)?
		};

		$($crate::__::compile_error!($crate::__::concat!("Catch-other lifetime must be last, but was followed by: ", $crate::__::stringify!($($rest)+)));)?
	};

	// For macro authors.
	(
		[$($kws:tt)*] [$($excluded_kws:tt)*]
		allow_prior_unused!;
		$($rest:tt)*
	) => {
		$crate::__lifetimes_muncher! {
			[$($kws)* $($excluded_kws)*] []
			$($rest)*
		}
	};

	// Other end.
	([$($kws:tt)*] [$($excluded_kws:tt)*]) => {
		$(
			$crate::__::compile_error!($crate::__::concat!("Unused key lifetime exclusion: ", $crate::__::stringify!($excluded_kws)));
		)*
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_lifetime {
	(PeekFrom for $name:ty, $ident:ident => $condition:expr$(,)?) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> bool {
				input.peek(|tts, _| matches!(tts, [$crate::__::TokenTree::Ident($ident)] if $condition && <$crate::__::Ident as $crate::__::ToString>::to_string($ident).as_str().starts_with('\'')))
			}
		}
	};

	(PopFrom for $name:ty, $ident:ident => $condition:expr, $message:expr$(,)?) => {
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;

			fn pop_parsed_from(
				input: &mut $crate::Input,
				errors: &mut $crate::Errors,
			) -> Result<Self::Parsed, ()> {
				input
					.pop_or_replace(|tts, _| match tts {
						[$crate::__::TokenTree::Ident($ident)] if $condition && <$crate::__::Ident as $crate::__::ToString>::to_string(&$ident).as_str().starts_with('\'') => Ok(Self($ident)),
						tts => Err(tts),
					})
					.map_err(|spans| {
						errors.push($crate::Error::new(
							$crate::ErrorPriority::TOKEN,
							$message,
							spans,
						))
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
