#![allow(dead_code)]

#[allow(deprecated)]
mod old {
	asteracea::component! {
		Empty()()

		[]
	}
}

asteracea::components! {
	Empty -> web {a a self 1}
	Empty2 -> web {a 2 3}
	a 4
}
