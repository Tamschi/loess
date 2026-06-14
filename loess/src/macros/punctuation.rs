#[macro_export]
macro_rules! punctuation {
	{
		$(#[$($attr:tt)*])*
		($($punct:tt)+) $(not before [$($not:tt)*])?
		as $vis:vis $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(Default $(@ $Default:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
			$(SimpleSpanned $(@ $SimpleSpanned:tt)?)?
			$(LocatedAt $(@ $LocatedAt:tt)?)?
			$(ResolvedAt $(@ $ResolvedAt:tt)?)?
		),*)? {
			$(
				$(#[$($field_attr:tt)*])*
				$punct_vis:vis $punct_name:ident
			),+$(,)?
		}$(;)?

		$($rest:tt)*
	} => {
		$crate::__validate_punctuation!($($punct)*);

		#[cfg_attr(
			any($($($(all(), $(@ $doc)?)?)?)*),
			doc = $crate::__::concat!(
				'`', $crate::__::stringify!($($punct)+), '`',
				$(" <sub>not before [", $(" `", $crate::__::stringify!($not), "`",)* "]</sub>",)?
			),
		)]
		$(#[$($attr)*])*
		$vis struct $name {
			$(
				$(#[$($field_attr)*])*
				$punct_vis $punct_name: $crate::__::Punct,
			)+
		}

		// Implementations.
		const _: () = {
			const OP: &str = $crate::__::stringify!($($punct)+);
			const NOT: &str = $crate::__::concat!($($crate::__::stringify!($($not)+))?);

			#[cfg(any($($($(all() $(@ $Default)?)?)*)?))]
			$crate::__impl_punctuation!(Default for $name { $($punct_name),* }, OP);

			$($(
				$(
					$(@ $PeekFrom)?
					$crate::__impl_punctuation!(PeekFrom for $name, OP, NOT);
				)?
				//TODO
			)*)?

			#[cfg(any($($($(all() $(@ $PopFrom)?)?)*)?))]
			$crate::__impl_punctuation!(PopFrom for $name { $($punct_name),* }, OP, NOT);

			#[cfg(any($($($(all() $(@ $IntoTokens)?)?)*)?))]
			$crate::__impl_punctuation!(IntoTokens for $name { $($punct_name),* }, OP, NOT);
		};

		$crate::punctuation!($($rest)*);
	};

	{
		$(#[$($attr:tt)*])*
		($($punct:tt)+) $(not before [$($not:tt)*])?
		as $vis:vis $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(Default $(@ $Default:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
			$(SimpleSpanned $(@ $SimpleSpanned:tt)?)?
			$(LocatedAt $(@ $LocatedAt:tt)?)?
			$(ResolvedAt $(@ $ResolvedAt:tt)?)?
		),*)?(
			$(#[$($field_attr:tt)*])*
			$($punct_vis:vis),+
		);

		$($rest:tt)*
	} => {
		$crate::__validate_punctuation!($($punct)*);

		#[cfg_attr(
			any($($($(all(), $(@ $doc)?)?)?)*),
			doc = $crate::__::concat!(
				'`', $crate::__::stringify!($($punct)+), '`',
				$(" <sub>not before [", $(" `", $crate::__::stringify!($not), "`",)* "]</sub>",)?
			),
		)]
		$(#[$($attr)*])*
		$vis struct $name(
			$($punct_vis $crate::__::Punct),+
		);

		// Implementations.
		const _: () = {
			const OP: &str = $crate::__::stringify!($($punct)+);
			const NOT: &str = $crate::__::concat!($($crate::__::stringify!($($not)+))?);
			$($(
				$(
					$(@ $PeekFrom)?
					$crate::__impl_punctuation!(PeekFrom for $name, OP, NOT);
				)?
				//TODO
			)*)?
		};

		$crate::punctuation!($($rest)*);
	};

	{
		$(#[$($attr:tt)*])*
		($($punct:tt)+) $(not before [$($not:tt)*])?
		as $vis:vis $name:ident$(: $(
			$(doc $(@ $doc:tt)?)?
			$(Default $(@ $Default:tt)?)?
			$(PeekFrom $(@ $PeekFrom:tt)?)?
			$(PopFrom $(@ $PopFrom:tt)?)?
			$(IntoTokens $(@ $IntoTokens:tt)?)?
			$(SimpleSpanned $(@ $SimpleSpanned:tt)?)?
			$(LocatedAt $(@ $LocatedAt:tt)?)?
			$(ResolvedAt $(@ $ResolvedAt:tt)?)?
		),*)?;

		$($rest:tt)*
	} => {
		$crate::__validate_punctuation!($($punct)*);

		#[cfg_attr(
			any($($($(all(), $(@ $doc)?)?)?)*),
			doc = $crate::__::concat!(
				'`', $crate::__::stringify!($($punct)+), '`',
				$(" <sub>not before [", $(" `", $crate::__::stringify!($not), "`",)* "]</sub>",)?
			),
		)]
		$(#[$($attr)*])*
		$vis struct $name;

		// Implementations.
		const _: () = {
			const OP: &str = $crate::__::stringify!($($punct)+);
			const NOT: &str = $crate::__::concat!($($crate::__::stringify!($($not)+))?);
			$($(
				$(
					$(@ $Default)?
					$crate::__impl_punctuation!(Default for $name, OP);
				)?
				$(
					$(@ $PeekFrom)?
					$crate::__impl_punctuation!(PeekFrom for $name, OP, NOT);
				)?
				$(
					$(@ $PopFrom)?
					$crate::__impl_punctuation!(PopFrom for $name, OP, NOT);
				)?
				//TODO
			)*)?
		};

		$crate::punctuation!($($rest)*);
	};

	// End.
	{} => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __validate_punctuation {
	($other:ident $($rest:tt)*) => {
		$crate::__::compile_error!($crate::__::concat!("Expected punct, but found: ", $crate::__::stringify!($other)));
		$crate::__validate_punctuation!($($rest)*);
	};

	($other:block $($rest:tt)*) => {
		$crate::__::compile_error!($crate::__::concat!("Expected punct, but found: ", $crate::__::stringify!($other)));
		$crate::__validate_punctuation!($($rest)*);
	};

	($other:lifetime $($rest:tt)*) => {
		$crate::__::compile_error!($crate::__::concat!("Expected punct, but found: ", $crate::__::stringify!($other)));
		$crate::__validate_punctuation!($($rest)*);
	};

	// The `literal` fragment doesn't fall back if it encounters `-`.
	(- $($rest:tt)*) => {
		$crate::__validate_punctuation!($($rest)*);
	};
	($other:literal $($rest:tt)*) => {
		$crate::__::compile_error!($crate::__::concat!("Expected punct, but found: ", $crate::__::stringify!($other)));
		$crate::__validate_punctuation!($($rest)*);
	};

	(($($other:tt)*) $($rest:tt)*) => {
		$crate::__::compile_error!($crate::__::concat!("Expected punct, but found: (", $crate::__::stringify!($($other),*), ")"));
		$crate::__validate_punctuation!($($rest)*);
	};

	($tt:tt $($rest:tt)*) => ( $crate::__validate_punctuation!($($rest)*); );

	// End.
	() => ();
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_punctuation {
	(PeekFrom for $name:ident, $OP:expr, $NOT:expr) => {
		impl $crate::PeekFrom for $name {
			fn peek_from(input: &$crate::Input) -> bool {
				const LEN: usize = $OP.len();

				//FIXME: Should be a constant assert once possible.
				$crate::__::debug_assert!(
					!$OP.contains(' '),
					"Unexpected space in punctuation definition `{}`.",
					$OP,
				);

				//TODO: Assert length.

				input.peek(|tts: [&$crate::__::TokenTree; LEN], mut rest| {
					tts.into_iter().enumerate().all(|(i, tt)| match tt {
						$crate::__::TokenTree::Punct(punct)
							if punct.as_char() == $OP.chars().nth(i).expect("") =>
						{
							if i < const { LEN - 1 } {
								punct.spacing() == $crate::__::Spacing::Joint
							} else {
								punct.spacing() == $crate::__::Spacing::Alone || {
									if let Some($crate::__::TokenTree::Punct(next)) = rest.next() {
										!$NOT.contains(next.as_char())
									} else {
										true
									}
								}
							}
						}
						_ => false,
					})
				})
			}
		}
	};

	(Default for $name:ident, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				todo!("punctuation!: `Default` for unit punctuation");
			}
		}
	};

	(Default for $name:ident { $punct_name_0:ident$(,)? }, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				let mut chars = $OP.chars();
				let this = Self {
					$punct_name_0: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Alone).with_span($crate::__::Span::mixed_site()),
				};
				$crate::__::debug_assert_eq!(chars.next(), $crate::__::None);
				this
			}
		}
	};

	(Default for $name:ident { $punct_name_0:ident, $punct_name_1:ident$(,)? }, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				let mut chars = $OP.chars();
				let this = Self {
					$punct_name_0: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_1: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Alone).with_span($crate::__::Span::mixed_site()),
				};
				$crate::__::debug_assert_matches!(chars.next(), $crate::__::None);
				this
			}
		}
	};

	(Default for $name:ident { $punct_name_0:ident, $punct_name_1:ident, $punct_name_2:ident$(,)? }, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				let mut chars = $OP.chars();
				let this = Self {
					$punct_name_0: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_1: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_2: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Alone).with_span($crate::__::Span::mixed_site()),
				};
				$crate::__::debug_assert_matches!(chars.next(), $crate::__::None);
				this
			}
		}
	};

	(Default for $name:ident { $punct_name_0:ident, $punct_name_1:ident, $punct_name_2:ident, $punct_name_3:ident$(,)? }, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				let mut chars = $OP.chars();
				let this = Self {
					$punct_name_0: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_1: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_2: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_3: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Alone).with_span($crate::__::Span::mixed_site()),
				};
				$crate::__::debug_assert_matches!(chars.next(), $crate::__::None);
				this
			}
		}
	};

	(Default for $name:ident { $punct_name_0:ident, $punct_name_1:ident, $punct_name_2:ident, $punct_name_3:ident$(,)? }, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				let mut chars = $OP.chars();
				let this = Self {
					$punct_name_0: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_1: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_2: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_3: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Joint).with_span($crate::__::Span::mixed_site()),
					$punct_name_4: $crate::__::Punct::new(chars.next().expect(""), $crate::__::Spacing::Alone).with_span($crate::__::Span::mixed_site()),
				};
				$crate::__::debug_assert_matches!(chars.next(), $crate::__::None);
				this
			}
		}
	};

	(Default for $name:ident { $($punct_name:ident),*$(,)? }, $OP:expr) => {
		impl $crate::__::Default for $name {
			fn default() -> Self {
				todo!("punctuation!: `Default` for struct punctuation with this many `Punct`s");
			}
		}
	};

	(PopFrom for $name:ident { $($punct_name:ident),*$(,)? }, $OP:expr, $NOT:expr) => {
		impl $crate::PopParsedFrom for $name {
			type Parsed = Self;

			fn pop_parsed_from(input: &mut $crate::Input, errors: &mut $crate::Errors) -> $crate::__::Result<Self, Option<Self>> {
				if <Self as $crate::PeekFrom>::peek_from(input) {
					$crate::__::Result::Ok(Self {
						$($punct_name: $crate::PopFrom::pop_from(input, errors).expect($crate::__::concat!("Infallible unless `<", $crate::__::stringify!($name), " as PeekFrom>::peek_from` misbehaves.")),)*
					})
				} else {
					errors.push(Error::new(
						$crate::ErrorPriority::TOKEN,
						$crate::__::format!("Expected `{}`.", $OP),
						[input.front_span()],
					));
					$crate::__::Result::Err(Some(<Self as $crate::__::Default>::default()))
				}
			}
		}
	};

	(IntoTokens for $name:ident { $($punct_name:ident),*$(,)? }, $OP:expr, $NOT:expr) => {
		impl $crate::IntoTokens for $name {
			fn into_tokens(self, root: &$crate::__::TokenStream, tokens: &mut impl $crate::__::Extend<$crate::__::TokenTree>) {
				todo!("__impl_punctuation!(IntoTokens for $name:ident …)")
			}
		}
	};
}
