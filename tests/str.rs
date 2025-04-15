mod old {
	#![allow(deprecated)]

	asteracea::component! {
		pub Str()() -> &str

		{ "Testing…" }
	}
}

// Note: In this new macro, it's not possible to render
// values of a type not specified by the substrate.
asteracea::components! {
	pub Str -> web {
		"Testing…";
	}
}
