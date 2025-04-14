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
	a
}
