use crate::ast;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct MethodCollection<'a> {
	pub methods_by_name: FxHashMap<&'a str, usize>,
	pub methods: Vec<MethodInfo>,
}

impl<'a> MethodCollection<'a> {
	pub fn blank() -> Self {
		Self { methods_by_name: FxHashMap::default(), methods: Vec::new() }
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MethodInfo {
	pub function_shape_index: usize,
	pub kind: ast::Node<ast::MethodKind>,
}
