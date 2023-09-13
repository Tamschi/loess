use proc_macro2::Ident;

use crate::io::{Input, Parse};

pub struct Identifier {
	pub ident: Ident,
}

impl Parse<'_> for Identifier {
	fn parse(input: &mut Input<'_>) -> Self {
		todo!()
	}
}
