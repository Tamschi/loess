#![forbid(unsafe_code)]

use proc_macro::TokenStream as TokenStream1;

extern crate proc_macro;

/// Old macro.
mod component;

/// New macro.
mod components;

#[proc_macro]
pub fn components(input: TokenStream1) -> TokenStream1 {
	hook_panics();
	components::components(input.into()).into()
}

#[proc_macro]
#[deprecated = "Use `components!` instead! It has better syntax."]
pub fn component(input: TokenStream1) -> TokenStream1 {
	hook_panics();
	component::component(input)
}

#[proc_macro]
pub fn bump_format(input: TokenStream1) -> TokenStream1 {
	component::bump_format(input)
}

#[proc_macro]
pub fn fragment(input: TokenStream1) -> TokenStream1 {
	component::fragment(input)
}

/// An attribute macro that discards its arguments and returns what it is applied to unchanged.
///
/// Used as stub when another attribute is not to be activated.
#[proc_macro_attribute]
pub fn discard_these_attribute_args(args: TokenStream1, item: TokenStream1) -> TokenStream1 {
	component::discard_these_attribute_args(args, item)
}

/// Returns just an `::asteracea::__::tracing::Span`,
/// preserving [`Span`] location but resolving it at [`Span::mixed_site()`](`Span::mixed_site`).
#[proc_macro]
pub fn fake_span(input: TokenStream1) -> TokenStream1 {
	component::fake_span(input)
}

/// Discards all tokens and outputs an empty block instead,
/// preserving [`Span`] location but resolving it at [`Span::mixed_site()`](`Span::mixed_site`).
#[proc_macro]
pub fn empty_block(input: TokenStream1) -> TokenStream1 {
	component::empty_block(input)
}

pub(crate) fn hook_panics() {
	std::panic::set_hook(Box::new(|panic_info| {
		let location = panic_info.location();

		let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
			s
		} else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
			s.as_str()
		} else {
			"(unknown panic type)"
		};
		eprintln!(
			"Asteracea proc macro panic at {}:{}\n\n{}",
			location.map(|l| l.file()).unwrap_or("None"),
			location
				.map(|l| l.line().to_string())
				.unwrap_or_else(|| "None".to_string()),
			payload
		);
	}))
}
