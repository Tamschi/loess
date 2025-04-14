use std::collections::VecDeque;

use component::Component;
use loess::{Errors, Input, PopFrom};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;

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

	dbg!((&errors, &components));

	//TODO: Interlace errors and components?
	quote_spanned! {Span::mixed_site()=>
		#errors
		#(#components)*
	}
}

mod component;
