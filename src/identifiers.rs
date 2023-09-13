use proc_macro2::{Ident, Span};

use crate::io::{Input, Parse};

pub struct Identifier {
	pub ident: Ident,
}

impl Parse<'_> for Identifier {
	fn parse(input: &mut Input<'_>) -> Self {
		todo!("except keywords and reserved words")
	}

	fn describe(w: &mut dyn std::fmt::Write) {
		w.write_str("IDENTIFIER")
	}
}

impl Default for Identifier {
	fn default() -> Self {
		Self {
			ident: Ident::new("MISSING", Span::mixed_site()),
		}
	}
}
