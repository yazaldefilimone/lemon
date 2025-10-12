use crate::{
	reference::SliceRef,
	store::{builtin_types, function_store},
	symbols,
	types::{self, TypeId},
};

#[derive(Debug)]
pub struct TypeStore<'a> {
	pub primitive_symbols: SliceRef<symbols::Symbol<'a>>,
	pub entries: types::TypeEntries,
	pub builtin: builtin_types::BuiltinTypes,
	pub functions: function_store::FunctionStore<'a>,
}

impl<'a> TypeStore<'a> {
	pub fn new() -> Self {
		let mut entries = types::TypeEntries::new();
		let builtin = builtin_types::BuiltinTypes::new(&mut entries);
		let primitive_symbols = Vec::new();
		let functions = function_store::FunctionStore::new();
		Self { primitive_symbols: SliceRef::from_vec(primitive_symbols), entries, builtin, functions }
	}

	pub fn direct_match(&self, base: TypeId, other: TypeId) -> bool {
		base.entry == other.entry
	}

	pub fn type_name(&self, type_id: TypeId) -> String {
		self.internal_type_name(type_id)
	}

	fn internal_type_name(&self, type_id: TypeId) -> String {
		let entry = self.entries.get(type_id);
		match entry.kind {
			types::TypeEntryKind::Array(array) => array.name(&self),
			types::TypeEntryKind::BuiltinType(builtin) => builtin.name().to_owned(),
			types::TypeEntryKind::Pointer(pointer) => pointer.name(&self),
			types::TypeEntryKind::Slice(slice) => slice.name(&self),
			_ => format!("unknown {}", type_id),
		}
	}
}
