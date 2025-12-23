use sealed::Sealed;

/// [`Infallible`](`std::convert::Infallible`) - Parsing can't be failing when this appears as [`Err`] payload (unless it already was). Note that this often just means parsing must have recovered at that point, but errors still may have been emitted.
pub type Infallible = std::convert::Infallible;

/// [`()`](https://doc.rust-lang.org/stable/std/primitive.unit.html) - Parsing can fail, only without a placeholder.
pub type NoPlaceholder = ();

/// [`Option<T>`] - Parsing can fail, but potentially still produces a placeholder.
pub type PotentiallyPlaceholder<T> = Option<T>;

/// [`(T,)`](https://doc.rust-lang.org/stable/std/primitive.tuple.html) - Parsing can fail, but even while failing, it must produce some sort of usable stand-in value.
///
/// It would be nicer if this was just `T`, but without specialisation I can't blanket-implement [`Remnant`] for that.
pub type Placeholder<T> = (T,);

pub(crate) mod sealed {
	use std::convert::Infallible;

	use super::Remnant;

	pub trait Sealed<T> {}

	impl<T> Sealed<T> for () {}
	impl<T> Sealed<T> for Infallible {}
	impl<T> Sealed<T> for Option<T> {}
	impl<T> Sealed<T> for (T,) {}
	impl<R, T> Sealed<Box<T>> for Box<R> where R: Remnant<T> {}
}

/// Type-state-ish residual value that may potentially carry some useful information still.
///
/// Sealed. Has combinators to produce mapped or merged [`Err`] payloads in generic [`PopParsedFrom`](`crate::PopParsedFrom`) implementations.
pub trait Remnant<T>: Sealed<T> {
	type Option: Remnant<T>;
	type Mapped<U>: Remnant<U>;

	fn retrieve(self) -> Option<T>;
	fn into_some(self) -> Self::Option;
	fn none() -> Self::Option;
	fn map<U>(self, f: impl FnOnce(T) -> U) -> Self::Mapped<U>;
}

impl<T> Remnant<T> for () {
	type Option = ();
	type Mapped<U> = ();

	fn retrieve(self) -> Option<T> {
		None
	}

	fn into_some(self) -> Self::Option {
		self
	}

	fn none() -> Self::Option {
		()
	}

	fn map<U>(self, _: impl FnOnce(T) -> U) -> Self::Mapped<U> {
		self
	}
}

impl<T> Remnant<T> for Infallible {
	type Option = ();
	type Mapped<U> = Infallible;
	fn retrieve(self) -> Option<T> {
		match self {}
	}

	fn into_some(self) -> Self::Option {
		match self {}
	}

	fn none() -> Self::Option {
		()
	}

	fn map<U>(self, _: impl FnOnce(T) -> U) -> Self::Mapped<U> {
		self
	}
}

impl<T> Remnant<T> for Option<T> {
	type Option = Option<T>;
	type Mapped<U> = Option<U>;

	fn retrieve(self) -> Option<T> {
		self
	}

	fn into_some(self) -> Self::Option {
		self
	}

	fn none() -> Self::Option {
		None
	}

	fn map<U>(self, f: impl FnOnce(T) -> U) -> Self::Mapped<U> {
		self.map(f)
	}
}

impl<T> Remnant<T> for (T,) {
	type Option = Option<T>;
	type Mapped<U> = (U,);

	fn retrieve(self) -> Option<T> {
		Some(self.0)
	}

	fn into_some(self) -> Self::Option {
		Some(self.0)
	}

	fn none() -> Self::Option {
		None
	}

	fn map<U>(self, f: impl FnOnce(T) -> U) -> Self::Mapped<U> {
		(f(self.0),)
	}
}

impl<R, T> Remnant<Box<T>> for Box<R>
where
	R: Remnant<T>,
{
	type Option = Box<R::Option>;
	type Mapped<U> = R::Mapped<U>;

	fn retrieve(self) -> Option<Box<T>> {
		(*self).retrieve().map(Box::new)
	}

	fn into_some(self) -> Self::Option {
		(*self).into_some().into()
	}

	fn none() -> Self::Option {
		R::none().into()
	}

	fn map<U>(self, f: impl FnOnce(Box<T>) -> U) -> Self::Mapped<U> {
		(*self).map(|t| f(Box::new(t)))
	}
}
