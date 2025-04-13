use proc_macro2::{Punct, Spacing, Span, TokenTree};

use crate::{Error, PopFrom, PopOrReplaceExt, WithSpanExt};

/// See <https://doc.rust-lang.org/stable/reference/tokens.html?highlight=arrow#punctuation> as of 2025-04-13.
pub struct RArrow {
	pub minus: Punct,
	pub gt: Punct,
}

impl Default for RArrow {
	fn default() -> Self {
		Self {
			minus: Punct::new('-', Spacing::Joint).with_span(Span::mixed_site()),
			gt: Punct::new('>', Spacing::Alone).with_span(Span::mixed_site()),
		}
	}
}

impl PopFrom for RArrow {
	fn pop_from(
		input: &mut std::collections::VecDeque<proc_macro2::TokenTree>,
		errors: &mut Vec<crate::Error>,
	) -> Result<Self, ()> {
		input
			.pop_or_replace(|ts| match ts {
				[TokenTree::Punct(minus), TokenTree::Punct(gt)]
					if minus.as_char() == '-'
						&& minus.spacing() == Spacing::Joint
						&& gt.as_char() == '>'
						&& gt.spacing() == Spacing::Alone =>
				{
					Ok(Self { minus, gt })
				}
				other => Err(other),
			})
			.map_err(|spans| errors.push(Error::new("Expected `->`.", spans)))
	}
}
