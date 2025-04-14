#![allow(dead_code)]

#[allow(deprecated)]
mod old {
	asteracea::component! {
		Empty()()

		[]
	}
}

asteracea::components! {
	Empty -> web {}
	pub Empty2 -> web {}
	pub(crate) Empty3 -> web {}
}
