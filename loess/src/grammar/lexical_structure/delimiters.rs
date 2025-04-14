use std::collections::VecDeque;

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree, extra::DelimSpan};

use crate::{
	Error, ErrorPriority, Errors, Exhaustive, Input, IntoTokens, PeekFrom, PopFrom,
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
						[TokenTree::Group(braces)] if braces.delimiter() == $delimiter => Ok((
							braces.delim_span(),
							Input {
								tokens: braces.stream().into_iter().collect::<VecDeque<_>>(),
								end: braces.span_close(),
							},
						)),
						other => Err(other),
					})
					.map_err(|spans| {
						errors.push(Error::new(ErrorPriority::TOKEN, $error, spans))
					})?;

				Ok(Self {
					span,
					contents: Exhaustive::<T, UNCONSUMED_IN_DELIMITER>::pop_from(
						&mut contents,
						errors,
					)?
					.0,
				})
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
