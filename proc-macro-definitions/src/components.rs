use std::collections::VecDeque;

use component::Component;
use loess::{Errors, Input, IntoTokens, PopFrom};
use proc_macro2::{Span, TokenStream};

pub fn components(input: TokenStream) -> TokenStream {
	let mut errors = Errors::new();

	let components = Vec::<Component>::pop_from(
		&mut Input {
			tokens: input.into_iter().collect::<VecDeque<_>>(),
			end: Span::mixed_site(),
		},
		&mut errors,
	)
	.unwrap_or_default();

	// dbg!((&errors, &components));

	let root = TokenStream::new();
	let mut output = TokenStream::new();
	errors.into_tokens(&root, &mut output);
	for component in components {
		component.into_tokens(&root, &mut output)
	}
	output
}

mod component;
