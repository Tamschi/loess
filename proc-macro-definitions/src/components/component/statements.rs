use loess::grammar;

grammar! {
	pub enum Statement: PopFrom, IntoTokens {
		Child(child::Child),
	} else "Expected Asteracea statement.";
}

mod child;
