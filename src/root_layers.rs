use rustc_hash::FxHashMap;

use crate::{reference::Ref, symbols::scope::Symbols};

#[derive(Debug)]
pub struct RootLayers<'a> {
	root: Ref<Module<'a>>,
	pub root_name: String,
}

#[derive(Debug)]
pub struct Module<'a> {
	pub name: &'a str,
	pub children: FxHashMap<&'a str, Ref<Module<'a>>>,
	pub symbols: Symbols<'a>,
}

impl<'a> Module<'a> {
	pub fn new(name: &'a str) -> Self {
		Module { name, children: FxHashMap::default(), symbols: Symbols::new() }
	}
}

impl<'a> RootLayers<'a> {
	pub fn new(root_name: &'a str) -> Self {
		let root = Ref::new(Module::new(root_name));
		Self { root, root_name: root_name.to_string() }
	}

	pub fn import_module(&mut self, path: &'a [&str]) -> Ref<Module<'a>> {
		let mut current = self.root.clone();

		for segment in path {
			let module = current.inner_mut();
			let child = module.children.entry(*segment);
			let child = child.or_insert_with(|| Ref::new(Module::new(segment))).clone();
			current = child;
		}
		current
	}

	pub fn lookup_module(&self, path: &[&str]) -> Option<Ref<Module>> {
		let mut current = self.root.clone();

		for segment in path {
			let module = current.inner();
			let child = module.children.get(*segment)?.clone();
			current = child;
		}
		Some(current)
	}
}
