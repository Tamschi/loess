use std::collections::VecDeque;
use std::{
	any::TypeId,
	panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree, extra::DelimSpan};

use crate::{
	Error, ErrorPriority, Errors, Exhaustive, HandledPanic, Input, IntoTokens, PeekFrom, PopFrom,
	error_priorities::UNCONSUMED_IN_DELIMITER,
};

macro_rules! delimiter_struct {
	($name:ident, $delimiter:expr, $error:literal) => {
		pub struct $name<T = TokenStream> {
			pub span: DelimSpan,
			pub contents: T,
		}

		/// Checks for the delimiters **and contents**.
		impl<T: PeekFrom> PeekFrom for $name<T> {
			fn peek_from(input: &Input) -> bool {
				match input.front() {
					Some(TokenTree::Group(group)) if group.delimiter() == $delimiter => {
						T::peek_from(&Input {
							tokens: group.stream().into_iter().collect(),
							end: group.span_close(),
						})
					}
					_ => false,
				}
			}
		}

		impl<T> $name<T> {
			pub fn map<U>(self, f: impl FnOnce(T) -> U) -> $name<U> {
				let Self { span, contents } = self;
				$name {
					span,
					contents: f(contents),
				}
			}

			pub fn try_map<U, E>(
				self,
				f: impl FnOnce(T) -> Result<U, E>,
			) -> Result<$name<U>, $name<E>> {
				let Self { span, contents } = self;
				$name {
					span,
					contents: f(contents),
				}
				.transpose()
			}
		}

		impl<T, E> $name<Result<T, E>> {
			pub fn transpose(self) -> Result<$name<T>, $name<E>> {
				let Self { span, contents } = self;
				match contents {
					Ok(contents) => Ok($name { span, contents }),
					Err(contents) => Err($name { span, contents }),
				}
			}
		}

		impl<T: PopFrom> PopFrom for $name<T> {
			fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()> {
				let (span, mut contents) = input
					.pop_or_replace(|ts| match ts {
						[TokenTree::Group(group)] if group.delimiter() == $delimiter => Ok((
							group.delim_span(),
							Input {
								tokens: group.stream().into_iter().collect::<VecDeque<_>>(),
								end: group.span_close(),
							},
						)),
						other => Err(other),
					})
					.map_err(|spans| {
						errors.push(Error::new(ErrorPriority::TOKEN, $error, spans))
					})?;

				match catch_unwind(AssertUnwindSafe(|| {
					Ok(Self {
						span,
						contents: Exhaustive::<T, UNCONSUMED_IN_DELIMITER>::pop_from(
							&mut contents,
							errors,
						)?
						.0,
					})
				})) {
					Ok(result) => result,
					Err(panic) => {
						errors.push(Error::new(
							ErrorPriority::PANIC,
							&format!(
								"proc macro panicked: {:?}",
								if panic.as_ref().is::<HandledPanic>() {
									resume_unwind(panic)
								} else if let Some(message) =
									panic.as_ref().downcast_ref::<String>()
								{
									message.clone()
								} else if let Some(message) =
									panic.as_ref().downcast_ref::<&'static str>()
								{
									message.to_string()
								} else {
									// Unhandled panic type.
									resume_unwind(panic)
								}
							),
							[contents.front_span()],
						));
						resume_unwind(Box::new(HandledPanic));
					}
				}
			}
		}

		impl<T: IntoTokens> IntoTokens for $name<T> {
			fn into_tokens(self, root: &TokenStream, tokens: &mut impl Extend<TokenTree>) {
				let mut group = Group::new($delimiter, self.contents.collect_tokens(root));
				group.set_span(self.span.join());
				tokens.extend([TokenTree::Group(group)]);
			}
		}
	};
}

delimiter_struct!(CurlyBraces, Delimiter::Brace, "Expected `{`.");
delimiter_struct!(SquareBrackets, Delimiter::Bracket, "Expected `[`.");
delimiter_struct!(Parentheses, Delimiter::Parenthesis, "Expected `(`.");
