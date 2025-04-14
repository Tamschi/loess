use std::mem::size_of;

#[test]
fn stateless_components_are_zero_sized_old() {
	#![allow(deprecated)]

	asteracea::component! {
		Empty()()
		""
	}

	assert_eq!(size_of::<Empty>(), 0)
}

#[test]
fn stateless_components_are_zero_sized() {
	asteracea::components! {
		Empty -> web {}
	}

	assert_eq!(size_of::<Empty>(), 0)
}
