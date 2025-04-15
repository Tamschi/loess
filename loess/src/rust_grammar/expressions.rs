use crate::grammar;

grammar! {
	pub enum ExpressionExceptStructExpression: PopFrom, IntoTokens {
		WithoutBlock(ExpressionWithoutBlockExceptStructExpression),
		WithBlock(ExpressionWithBlock),
	} else "Expected Expression except StructExpression.";

	pub enum ExpressionWithBlock {
		Block(BlockExpression),
		ConstBlock(ConstBlockExpression),
		UnsafeBlock(UnsafeBlockExpression),
        
	} else "Expected ExpressionWithBlock";
}
