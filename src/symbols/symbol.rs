use std::{collections::hash_map, path::PathBuf};

use rustc_hash::FxHashMap;

use crate::{
	checker::types::TypeId, messages::Messages, reference::Ref, root_layers::RootLayer, span::Span,
};

#[derive(Debug, Clone)]
pub struct Symbol<'a> {
	pub name: &'a str,
	pub kind: SymbolKind<'a>,
	pub span: Option<Span>,
	pub used: bool,
	pub imported: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind<'a> {
	BuiltinType { type_id: TypeId, methods_index: usize },
	UserType { shape_index: usize, methods_index: usize },
	UserTypeGeneric { shape_index: usize, generic_index: usize },
	FunctionGeneric { function_shape_index: usize, generic_index: usize },
	Function { function_shape_index: usize },
	Const { constant_index: usize },
	Static { static_index: usize },
	Let { readable_index: usize },
	Mut { readable_index: usize },
	Module { layer: Ref<RootLayer<'a>> },
}

impl<'a> SymbolKind<'a> {
	pub fn can_shadow(&self) -> bool {
		matches!(
			self,
			SymbolKind::Let { .. }
				| SymbolKind::Mut { .. }
				| SymbolKind::FunctionGeneric { .. }
				| SymbolKind::UserTypeGeneric { .. }
		)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ExternLocation<'a> {
	pub span: Span,
	pub module_path: &'a PathBuf,
}

impl<'a> ExternLocation<'a> {
	pub fn new(span: Span, module_path: &'a PathBuf) -> Self {
		ExternLocation { span, module_path }
	}
}

#[derive(Debug)]
pub struct Externs<'a> {
	pub externs: FxHashMap<String, Vec<ExternLocation<'a>>>,
}

impl<'a> Externs<'a> {
	pub fn new() -> Self {
		Externs { externs: FxHashMap::default() }
	}

	pub fn push(&mut self, messages: &mut Messages<'a>, name: &str, span: Span) {
		let location = ExternLocation { span, module_path: &messages.file.path };

		match self.externs.entry(name.to_string()) {
			hash_map::Entry::Occupied(mut occupied) => {
				occupied.get_mut().push(location);
			}

			hash_map::Entry::Vacant(entry) => {
				entry.insert(vec![location]);
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Readable<'a> {
	pub name: &'a str,
	pub type_id: TypeId,
	pub kind: ReadableKind,
	pub is_pointer_access_mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadableKind {
	Let,
	Mut,
}

#[derive(Debug)]
pub struct Readables<'a> {
	pub starting_index: usize,
	pub readables: Vec<Readable<'a>>,
}

impl<'a> Readables<'a> {
	pub fn new() -> Self {
		Readables { starting_index: 0, readables: Vec::new() }
	}

	pub fn overall_len(&self) -> usize {
		self.readables.len()
	}

	pub fn push(
		&mut self,
		name: &'a str,
		type_id: TypeId,
		kind: ReadableKind,
		is_pointer_access_mutable: bool,
	) -> usize {
		let index = self.readables.len() - self.starting_index;
		let readable = Readable { name, type_id, kind, is_pointer_access_mutable };
		self.readables.push(readable);
		index
	}

	pub fn get(&self, index: usize) -> Option<Readable<'a>> {
		self.readables.get(index + self.starting_index).copied()
	}
}

impl<'a> std::fmt::Display for SymbolKind<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let name = match self {
			SymbolKind::BuiltinType { .. } => "a built in type",
			SymbolKind::UserType { .. } => "a type",
			SymbolKind::UserTypeGeneric { .. } => "a type generic parameter",
			SymbolKind::FunctionGeneric { .. } => "a function generic parameter",
			SymbolKind::Function { .. } => "a function",
			SymbolKind::Const { .. } => "a constant",
			SymbolKind::Static { .. } => "a static",
			SymbolKind::Let { .. } => "an immutable binding",
			SymbolKind::Mut { .. } => "a mutable binding",
			SymbolKind::Module { .. } => "an imported module",
		};

		f.write_str(name)
	}
}
