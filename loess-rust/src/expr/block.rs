use loess::{grammar, scaffold::CurlyBraces};

grammar! {
	#[derive(Clone)]
	#[non_exhaustive]
	/// [BlockExpression](https://doc.rust-lang.org/stable/reference/expressions/block-expr.html?highlight=BlockExpression#r-expr.block.syntax)
	pub struct BlockExpression: PeekFrom, PopFrom, IntoTokens {
		/// Continue inside with [`BlockExpressionContent`].
		braces: CurlyBraces,
	}
}
