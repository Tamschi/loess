use std::panic::{catch_unwind, AssertUnwindSafe};

use component::Component;
use loess::{Error, ErrorPriority, Errors, HandledPanic, Input, IntoTokens, PopFrom};
use proc_macro2::{Span, TokenStream};

pub fn components(input: TokenStream) -> TokenStream {
	let mut input = Input {
		tokens: input.into_iter().collect(),
		end: Span::mixed_site(),
	};

	let mut errors = Errors::new();

	let mut components = vec![];
	'panic_to_error: {
		match catch_unwind(AssertUnwindSafe(|| {
			components = Vec::<Component>::pop_from(&mut input, &mut errors).unwrap_or_default()
		})) {
			Ok(()) => (),
			Err(panic) => errors.push(Error::new(
				ErrorPriority::PANIC,
				&format!(
					"macro panicked: {:?}",
					if panic.as_ref().is::<HandledPanic>() {
						break 'panic_to_error;
					} else if let Some(message) = panic.as_ref().downcast_ref::<String>() {
						message.clone()
					} else if let Some(message) = panic.as_ref().downcast_ref::<&'static str>() {
						message.to_string()
					} else {
						"(unhandled panic type)".to_string()
					}
				),
				[input.front_span()],
			)),
		}
	}

	let root = TokenStream::new();
	let mut output = TokenStream::new();
	errors.into_tokens(&root, &mut output);
	for component in components {
		component.transform(&root, &mut output)
	}
	output
}

mod component;
