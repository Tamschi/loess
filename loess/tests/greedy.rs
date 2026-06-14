use loess::{
	Errors, Input, PopParsedFrom, punctuation, raw_quote_into_mixed_site, scaffold::Greedy,
};
use proc_macro2::{Span, TokenStream};

punctuation! {
	#[derive(Debug)] (;) as Semi: Default, PeekFrom, PopFrom { pub semi }
}

macro_rules! input {
	($($tt:tt)*) => ({
		let mut tokens = TokenStream::new();
		raw_quote_into_mixed_site!(Span::mixed_site(), &mut tokens, {
			$($tt)*
		});
		let input = Input { tokens: tokens.into_iter().collect(), end: Span::call_site() };
		input
	});
}

#[test]
pub fn empty() {
	let mut input = input!();
	let mut errors = Errors::new();
	let parsed =
		<Greedy<Vec<Semi>> as PopParsedFrom>::pop_parsed_from(&mut input, &mut errors).unwrap();

	assert_eq!(input.len(), 0);
	assert!(errors.into_of_highest_priority().next().is_none());
	assert!(parsed.is_empty());
}

#[test]
pub fn all() {
	let mut input = input!(;;;;;);
	let mut errors = Errors::new();
	let parsed =
		<Greedy<Vec<Semi>> as PopParsedFrom>::pop_parsed_from(&mut input, &mut errors).unwrap();

	assert_eq!(input.len(), 0);
	assert!(errors.into_of_highest_priority().next().is_none());
	assert_eq!(parsed.len(), 5);
}

#[test]
pub fn not_all() {
	let mut input = input!(;;;+);
	let mut errors = Errors::new();
	let parsed =
		<Greedy<Vec<Semi>> as PopParsedFrom>::pop_parsed_from(&mut input, &mut errors).unwrap();

	assert_eq!(input.len(), 1);
	assert!(errors.into_of_highest_priority().next().is_none());
	assert_eq!(parsed.len(), 3);
}
