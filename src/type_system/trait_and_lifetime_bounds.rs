use crate::{
	io::Parse,
	tokens::{keywords::Static, punctuation::Underscore, LifetimeOrLabel, SPunct},
};

pub enum Lifetime {
	LifetimeOrLabel(LifetimeOrLabel),
	Static(SPunct<'\'', true>, Static),
	Transient(SPunct<'\'', true>, Underscore),
}

impl Parse<'_> for Lifetime {
	fn parse(input: &mut crate::io::Input<'_>) -> Self {
		if let (lol, Ok(())) = input.try_parse() {
			Self::LifetimeOrLabel(lol)
		} else if let ((s_punct, r#static), Ok(())) = input.try_parse() {
			Self::Static(s_punct, r#static)
		} else if let ((s_punct, underscore), Ok(())) = input.try_parse() {
			Self::Transient(s_punct, underscore)
		} else {
			input.error_expected()
		}
	}

	fn describe(w: &mut dyn std::fmt::Write) {
		w.write_char('(')?;
		LifetimeOrLabel::describe(w)?;
		w.write_str("|`'static`|`'_`)")
	}
}

impl Default for Lifetime {
	fn default() -> Self {
		Self::LifetimeOrLabel(LifetimeOrLabel::default())
	}
}
