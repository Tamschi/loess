use std::collections::VecDeque;

use component::Component;
use loess::{Error, PopFrom};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;

pub fn components(input: TokenStream) -> TokenStream {
	let mut errors = vec![];
	let mut components = vec![];

	let mut input = input.into_iter().collect::<VecDeque<_>>();

	while !input.is_empty() {
		let before_len = input.len();

		match Component::pop_from(&mut input, &mut errors) {
			Ok(component) => components.push(component),
			Err(()) => break,
		}

		if input.len() == before_len {
			let token = input.pop_front().expect("unreachable");
			let span = token.span().resolved_at(Span::call_site());

			errors.push(Error::new(format!("Unexpected token: `{token}`"), [span]));
			break;
		}
	}

	quote_spanned! {Span::mixed_site()=>
		#(#errors)*
		#(#components)*
	}
}

mod component;
