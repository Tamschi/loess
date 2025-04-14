use std::collections::VecDeque;

use component::Component;
use loess::{Errors, PopFrom};
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;

pub fn components(input: TokenStream) -> TokenStream {
	let mut input = input.into_iter().collect::<VecDeque<_>>();

	let mut errors = Errors::new();

	//TODO: Greedy parsing.
	let components = Vec::<Component>::pop_from(&mut input, &mut errors).unwrap_or_default();

	dbg!((&errors, &components));

	//TODO: Interlace errors and components?
	quote_spanned! {Span::mixed_site()=>
		#errors
		#(#components)*
	}
}

mod component;
