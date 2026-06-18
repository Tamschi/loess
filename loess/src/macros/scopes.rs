#[macro_export]
macro_rules! scopes {
	{
        $(#[$($attrs:tt)*])*
		$vis:vis $name:ident $(- $notVis:vis $not_name:ident)?: bool;

		$($rest:tt)*
	} => {
        $(#[$($attrs)*])*
		pub enum $name<T: ?$crate::__::Sized> {
			#[expect(missing_docs)]
			_Vacant($crate::__::PhantomData<T>, $crate::__::Infallible),
		}

		$(
            #[doc = $crate::__::concat!("Inverse [`Scope`](`", $crate::__::stringify!($crate::__::Scope), "`) of [`", $crate::__::stringify!($name), "`].")]
            pub enum $not_name<T: ?$crate::__::Sized> {
                #[expect(missing_docs)]
                _Vacant($crate::__::PhantomData<T>, $crate::__::Infallible),
            }
        )?

		const _: () = {
			$crate::__::thread_local! {
				static IN_SCOPE: $crate::__::Cell<$crate::__::bool> = $crate::__::Cell::new(false);
			}

			impl<T: ?$crate::__::Sized> $crate::scaffold::scoped::Scope for $name<T> {
				type Wrapped = T;

				fn is_in() -> $crate::__::bool {
					IN_SCOPE.with(|in_scope| in_scope.get())
				}
			}

			impl<T: ?$crate::__::Sized> $crate::PeekFrom for $name<T>
			where
				T: $crate::PeekFrom,
			{
				fn peek_from(input: &$crate::Input) -> $crate::__::bool {
					T::peek_from(input)
				}
			}

			impl<T: ?$crate::__::Sized> $crate::PopParsedFrom for $name<T>
			where
				T: $crate::PopParsedFrom,
			{
				type Parsed = T::Parsed;

				fn pop_parsed_from(
					input: &mut $crate::Input,
					errors: &mut $crate::Errors,
				) -> $crate::__::ControlFlow<$crate::__::Option<Self::Parsed>, $crate::__::Option<Self::Parsed>> {
					IN_SCOPE.with(|in_scope| {
						let prior = in_scope.replace(true);
						let catch_result =
							$crate::__::catch_unwind($crate::__::AssertUnwindSafe(move || T::pop_parsed_from(input, errors)));
						in_scope.set(prior);
						match catch_result {
							Ok(result) => result,
							Err(payload) => $crate::__::resume_unwind(payload),
						}
					})
				}
			}

			$(
				impl<T: ?$crate::__::Sized> $crate::scaffold::scoped::Scope for $not_name<T> {
					type Wrapped = T;

					fn is_in() -> $crate::__::bool {
						!IN_SCOPE.with(|in_scope| in_scope.get())
					}
				}

				impl<T: ?$crate::__::Sized> $crate::PeekFrom for $not_name<T>
				where
					T: $crate::PeekFrom,
				{
					fn peek_from(input: &$crate::Input) -> $crate::__::bool {
						T::peek_from(input)
					}
				}

				impl<T: ?$crate::__::Sized> $crate::PopParsedFrom for $not_name<T>
				where
					T: $crate::PopParsedFrom,
				{
					type Parsed = T::Parsed;

					fn pop_parsed_from(
						input: &mut $crate::Input,
						errors: &mut $crate::Errors,
					) -> $crate::__::ControlFlow<$crate::__::Option<Self::Parsed>, $crate::__::Option<Self::Parsed>> {
						IN_SCOPE.with(|in_scope| {
							let prior = in_scope.replace(false);
							let catch_result =
								$crate::__::catch_unwind($crate::__::AssertUnwindSafe(move || T::pop_parsed_from(input, errors)));
							in_scope.set(prior);
							match catch_result {
								Ok(result) => result,
								Err(payload) => $crate::__::resume_unwind(payload),
							}
						})
					}
				}
			)?
		};

		$crate::scopes!($($rest)*);
	};
	// End.
	{} => {};
}
