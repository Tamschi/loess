use crate::grammar;

use super::Or;

grammar! {
	pub struct Pattern: PopFrom, IntoTokens {
		pub or: Option<Or>,
		pub first: PatternNoTopAlt,
		pub further: Greedy<Vec<Pattern_>>,
	}

	pub struct Pattern_: PopFrom, IntoTokens {
		pub or: Or,
		pub pattern: PatternNoTopAlt,
	}

	pub enum PatternNoTopAlt: PopFrom, IntoTokens {
		PatternWithoutRange(PatternWithoutRange),
        RangePattern(RangePattern),
	} else "Expected Pattern.";
}
