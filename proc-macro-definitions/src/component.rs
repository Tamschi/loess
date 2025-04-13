use lazy_static::lazy_static;

use proc_macro::TokenStream as TokenStream1;

use proc_macro2::{Span, TokenStream as TokenStream2};

use proc_macro_crate::{crate_name, FoundCrate};

use quote::{quote, quote_spanned};

use std::iter;

use syn::{
	parse::{Parse, ParseStream},
	parse_macro_input,
	spanned::Spanned,
	Error, Ident, Result,
};

use tap::Conv;

use self::{
	component_declaration::ComponentDeclaration,
	map_message::MapMessage,
	part::{GenerateContext, Part},
};

use workaround_module::Configuration;

pub(crate) mod component_declaration;

pub(crate) mod map_message;

pub(crate) mod part;

pub(crate) mod storage_configuration;

pub(crate) mod storage_context;

pub(crate) mod syn_ext;

pub(crate) mod util;

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
			"Asteracea proc macro panic at {} line {}\n\n{}",
			location.map(|l| l.file()).unwrap_or("None"),
			location
				.map(|l| l.line().to_string())
				.unwrap_or_else(|| "None".to_string()),
			payload
		);
	}))
}

/// Macro entry point.
pub fn component(input: TokenStream1) -> TokenStream1 {
	hook_panics();

	let component_declaration = parse_macro_input!(input as ComponentDeclaration);
	let tokens: TokenStream2 = component_declaration
		.into_tokens()
		.unwrap_or_else(|error| error.to_compile_error());
	tokens.into()
}

pub(crate) struct BumpFormat {
	pub(crate) asteracea: Ident,
	pub(crate) bump_span: Span,
	pub(crate) input: TokenStream2,
}

/// Macro entry point.
pub fn bump_format(input: TokenStream1) -> TokenStream1 {
	let bump_format = parse_macro_input!(input as BumpFormat);
	let mut tokens = TokenStream2::new();
	bump_format.to_tokens_with_context(
		&mut tokens,
		&GenerateContext {
			thread_safety: quote!(_),
			prefer_thread_safe: None,
		},
	);
	tokens.into()
}

impl Parse for BumpFormat {
	fn parse(input: ParseStream) -> Result<Self> {
		//TODO: This is pretty hacky.
		// Change it to a better location once that feature is stable in proc_macro2.
		let bump_span = input.cursor().span();
		let asteracea = asteracea_ident(bump_span);
		Ok(BumpFormat {
			asteracea,
			bump_span,
			input: input.parse()?,
		})
	}
}

impl BumpFormat {
	pub(crate) fn to_tokens_with_context(&self, output: &mut TokenStream2, cx: &GenerateContext) {
		let BumpFormat {
			asteracea,
			bump_span,
			input,
		} = self;
		let thread_safety = &cx.thread_safety;
		let bump = Ident::new("bump", bump_span.resolved_at(Span::call_site()));
		output.extend(quote! {
			#asteracea::lignin::Node::Text::<#thread_safety> {
				text: #asteracea::bumpalo::format!(in #bump, #input)
					.into_bump_str(),
				dom_binding: None, //TODO?: Add DOM binding support.
			}
		});
	}
}

pub(crate) enum FragmentConfiguration {}

impl Configuration for FragmentConfiguration {
	const NAME: &'static str = "fragment!";
	const CAN_CAPTURE: bool = false;
}

/// Macro entry point.
pub fn fragment(input: TokenStream1) -> TokenStream1 {
	let asteracea = asteracea_ident(Span::mixed_site());
	let body = parse_macro_input!(input as Part<FragmentConfiguration>)
		.part_tokens(&GenerateContext {
			thread_safety: quote!(_),
			prefer_thread_safe: None,
		})
		.unwrap_or_else(|error| error.to_compile_error());
	(quote_spanned! {Span::mixed_site()=>
		((|| -> ::std::result::Result<_, ::#asteracea::error::Escalation> {
			Ok(#body)
		})())
	})
	.into()
}

// TODO: Accept reexported asteracea module made available via `use`.
lazy_static! {
		static ref ASTERACEA_NAME: String = crate_name("asteracea")
			.map(|found| match found {
				FoundCrate::Itself => "asteracea".to_string(), // This happens in tests.
				FoundCrate::Name(name) => name,
			})
			.unwrap_or_else(|_| "asteracea".to_owned());
}

pub(crate) fn asteracea_ident(span: Span) -> Ident {
	Ident::new(&*ASTERACEA_NAME, span)
}

/// SEE: <https://github.com/rust-lang/rust/issues/34537#issuecomment-554590043>
pub(crate) mod workaround_module {
	pub trait Configuration {
		const NAME: &'static str;
		const CAN_CAPTURE: bool;
	}
}

pub(crate) fn warn(location: Span, message: &str) -> Result<()> {
	Err(Error::new(location, message.to_string()))
}

pub(crate) trait FailSoftly<T, E>: Sized {
	fn fail_softly(self, errors: &mut impl Extend<E>, fallback: impl FnOnce() -> T) -> T;
	fn fail_softly_into<E2: From<E>>(
		self,
		errors: &mut (impl IntoIterator<Item = E2> + Extend<E2>),
		fallback: impl FnOnce() -> T,
	) -> T;
}

impl<T, E> FailSoftly<T, E> for std::result::Result<T, E> {
	fn fail_softly(self, errors: &mut impl Extend<E>, fallback: impl FnOnce() -> T) -> T {
		self.unwrap_or_else(|error| {
			errors.extend(iter::once(error));
			fallback()
		})
	}

	fn fail_softly_into<E2: From<E>>(
		self,
		errors: &mut (impl IntoIterator<Item = E2> + Extend<E2>),
		fallback: impl FnOnce() -> T,
	) -> T {
		self.map_err(Into::into).fail_softly(errors, fallback)
	}
}

pub fn discard_these_attribute_args(args: TokenStream1, item: TokenStream1) -> TokenStream1 {
	drop(args);
	item
}

pub fn fake_span(input: TokenStream1) -> TokenStream1 {
	let span = input
		.conv::<TokenStream2>()
		.span()
		.resolved_at(Span::mixed_site());
	let asteracea = asteracea_ident(span);
	quote_spanned!(span=> ::#asteracea::__::tracing::Span).into()
}

pub fn empty_block(input: TokenStream1) -> TokenStream1 {
	let span = input
		.conv::<TokenStream2>()
		.span()
		.resolved_at(Span::mixed_site());
	quote_spanned!(span=> {}).into()
}
