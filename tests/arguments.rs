mod old {
	#![allow(deprecated)]

	use std::any::TypeId;

	use asteracea::component;
	use bumpalo::Bump;
	use rhizome::sync::Node;

	component! {
		pub Greeting()(
			greeting: &str = "Hello!",
		) -> Sync

		<span
			."class" = "greeting"
			!(greeting)
		>
	}

	asteracea::component! {
		pub Classic()(
			class?: &'bump str,
		) -> Sync

		<div
			."class"? = {class} // `Option<_>`-typed!
		>
	}

	asteracea::component! {
	  Inner()(
		class?: &'bump str,
	  )

	  <span ."class"? = {class}>
	}

	asteracea::component! {
	  Middle()(
		class?: &'bump str,
	  )

	  <*Inner .class? = {class}>
	}

	asteracea::component! {
	  Outer()()

	  [
		<*Middle> "\n"
		<*Middle .class = {"bourgeoisie"}>
	  ]
	}

	#[test]
	pub(crate) fn test() {
		let outer = Box::pin(
			Outer::new(
				Node::new(TypeId::of::<()>()).as_ref(),
				Outer::new_args_builder().build(),
			)
			.unwrap(),
		);
		outer
			.as_ref()
			.render(&Bump::new(), Outer::render_args_builder().build())
			.unwrap();

		// TODO: Test output.
	}
}

use std::any::TypeId;

use bumpalo::Bump;
use rhizome::sync::Node;

asteracea::components! {
	pub Greeting[greeting: &str = "Hello!"] -> web {
		span["class" = "greeting"] {
			"{greeting}";
		}
	}

	pub Classic[class?: &'bump str] -> web {
		div["class"? = class]; // `Option<_>`-typed!
	}

	  Inner[class?: &'bump str] -> web {
		span["class"? = class];
	}

	Middle[class?: &'bump str] -> web {
		Inner[class? = class];
	}

	Outer -> web {
		Middle; "\n";
		Middle[class = "bourgeoisie"];
	}
}

#[test]
pub(crate) fn test() {
	let outer = Box::pin(
		Outer::new(
			Node::new(TypeId::of::<()>()).as_ref(),
			Outer::new_args_builder().build(),
		)
		.unwrap(),
	);
	outer
		.as_ref()
		.render(&Bump::new(), Outer::render_args_builder().build())
		.unwrap();

	// TODO: Test output.
}
