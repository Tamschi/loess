use loess::grammar;
use proc_macro2::TokenTree;

grammar! {
	pub enum Tuples: PeekFrom, PopFrom {
		One((TokenTree,)),
		Two((TokenTree,TokenTree,)),
		Three((TokenTree,TokenTree,TokenTree,)),
		Four((TokenTree,TokenTree,TokenTree,TokenTree,)),
		Five((TokenTree,TokenTree,TokenTree,TokenTree,TokenTree,)),

	} else unreachable!() as &str;
}
