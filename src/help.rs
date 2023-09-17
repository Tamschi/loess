use std::{
	cell::RefCell,
	iter::{self, IntoIterator},
	marker::PhantomData,
	vec::Vec,
};

#[derive(Debug, Clone, Default)]
pub struct DiagnosticsList<'a> {
	diagnostics: RefCell<Vec<Diagnostic<'a>>>,
}

impl<'a> DiagnosticsList<'a> {
	pub fn push(&self, diagnostic: Diagnostic<'a>) {
		self.diagnostics.borrow_mut().push(iter::once(diagnostic))
	}
}

impl<'a> IntoIterator for DiagnosticsList<'a> {
	type Item = Diagnostic<'a>;

	type IntoIter = <Vec<Diagnostic<'a>> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.diagnostics.into_inner().into_iter()
	}
}

impl<'a> Extend<Diagnostic<'a>> for &DiagnosticsList<'a> {
	fn extend<T: IntoIterator<Item = Diagnostic<'a>>>(&mut self, iter: T) {
		self.diagnostics.borrow_mut().extend(iter)
	}
}

#[non_exhaustive]
pub struct Diagnostic<'a> {
	pub todo: PhantomData<&'a ()>,
}

#[non_exhaustive]
pub enum DiagnosticType {
	Error,
}
