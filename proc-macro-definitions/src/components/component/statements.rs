use loess::{
	grammar,
	grammar::{CurlyBraces, Semi},
};

grammar! {
	pub enum Statement: PopFrom, IntoTokens {
		Block(CurlyBraces<Vec<Statement>>),
		Semi(Semi),
		Child(child::Child),
	} else "Expected Asteracea statement.";
}

mod child;
