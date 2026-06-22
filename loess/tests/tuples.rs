use loess::grammar;
use proc_macro2::TokenTree;

grammar! {
	//TODO: This doesn't actually implement IntoTokens yet.
	pub enum Tuples: PeekFrom, PopFrom, IntoTokens {
		One((TokenTree,)),
		Two((TokenTree,TokenTree,)),
		Three((TokenTree,TokenTree,TokenTree,)),
		Four((TokenTree,TokenTree,TokenTree,TokenTree,)),
		Five((TokenTree,TokenTree,TokenTree,TokenTree,TokenTree,)),

	} else unreachable!() as &str;
}
