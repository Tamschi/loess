use loess::grammar;
use proc_macro2::TokenTree;

grammar! {
	pub enum Enum: PeekFrom, PopFrom {
		Tuple(TokenTree, TokenTree),
		Single(TokenTree),
		Token,
	} else "message";

	pub enum Enum2: PeekFrom, PopFrom {
		Empty(),
	} else "message";
}

grammar! {
	pub enum Enum3 {} else unreachable!();
}
