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
	Empty4() -> web {}
	Empty5[] -> web {}
	Empty6()[] -> web {}
	const Empty7 -> web {}
	async Empty8 -> web {}
	const async Empty9 -> web {}
	const async Empty10()[] -> web {}
}
