use crate::{ast, reference::Ref};
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
	// pub symbols: Symbols<'a>,
}

impl<'a> RootLayer<'a> {
	pub fn new(name: &'a str) -> Self {
		RootLayer {
			name,
			children: FxHashMap::default(),
			// symbols: Symbols::new()
		}
	}
}

impl<'a> RootLayers<'a> {
	pub fn new(root_name: &'a str) -> Self {
		let root = Ref::new(RootLayer::new(root_name));
		Self { root, root_name: root_name.to_string() }
	}

	pub fn import(&mut self, path: &'a str) -> Ref<RootLayer<'a>> {
		todo!()
	}

	pub fn lookup(&self, path: &'a str) -> Option<Ref<RootLayer<'a>>> {
		todo!()
	}

	pub fn layer_for_module_name(&self, name: &ast::Node<&'a str>) -> Option<Ref<RootLayer<'a>>> {
		if let Some(layer) = self.root.children.get(name.item) {
			return Some(layer.clone());
		}
		None
	}

	pub fn empty() -> Self {
		Self::new("")
	}
}
