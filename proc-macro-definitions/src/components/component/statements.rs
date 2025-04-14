use loess::{Error, ErrorPriority, Errors, Input, PopFrom};

pub enum Statement {
	Child(child::Child),
}

impl PopFrom for Statement {
	fn pop_from(input: &mut Input, errors: &mut Errors) -> Result<Self, ()>
	where
		Self: Sized,
	{
		Ok(
			if let Some(child) = child::Child::peek_pop_from(input, errors)? {
				Self::Child(child)
			} else {
				errors.push(Error::new(
					ErrorPriority::GRAMMAR,
					"Expected Asteracea statement.",
					[input.front_span()],
				));
				return Err(());
			},
		)
	}
}

mod child;
