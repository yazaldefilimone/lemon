use crate::{ast::Node, reference::Ref, symbols::scope::Symbols};
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct RootLayers<'a> {
	root: Ref<RootLayer<'a>>,
	pub root_name: String,
}

#[derive(Debug)]
pub struct RootLayer<'a> {
	pub name: &'a str,
	pub children: FxHashMap<&'a str, Ref<RootLayer<'a>>>,
	pub symbols: Symbols<'a>,
}

impl<'a> RootLayer<'a> {
	pub fn new(name: &'a str) -> Self {
		RootLayer { name, children: FxHashMap::default(), symbols: Symbols::new() }
	}
}

impl<'a> RootLayers<'a> {
	pub fn new(root_name: &'a str) -> Self {
		let root = Ref::new(RootLayer::new(root_name));
		Self { root, root_name: root_name.to_string() }
	}

	pub fn import(&mut self, path: &'a [&str]) -> Ref<RootLayer<'a>> {
		let mut current = self.root.clone();

		for segment in path {
			let module = current.inner_mut();
			let child = module.children.entry(*segment);
			let child = child.or_insert_with(|| Ref::new(RootLayer::new(segment))).clone();
			current = child;
		}
		current
	}

	pub fn lookup(&self, path: &[&str]) -> Option<Ref<RootLayer>> {
		let mut current = self.root.clone();

		for segment in path {
			let module = current.inner();
			let child = module.children.get(*segment)?.clone();
			current = child;
		}
		Some(current)
	}

	pub fn layer_for_module_name(&self, name: &Node<&'a str>) -> Option<Ref<RootLayer<'a>>> {
		if let Some(layer) = self.root.children.get(name.item) {
			return Some(layer.clone());
		}
		None
	}
}
