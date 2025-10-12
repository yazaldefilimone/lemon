use crate::hir;

#[derive(Debug)]
pub struct FunctionStore<'a> {
	pub functions: Vec<hir::FunctionShape<'a>>,
}
impl<'a> FunctionStore<'a> {
	pub fn new() -> Self {
		Self { functions: Vec::new() }
	}

	pub fn add_shape(&mut self, shape: hir::FunctionShape<'a>) {
		self.functions.push(shape);
	}
}
