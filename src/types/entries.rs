use crate::store::type_store;

use super::{PrimativeKind, TypeId};
use std::num::NonZeroU32;

#[derive(Debug, Clone)]
pub struct TypeEntries {
	pub entries: Vec<TypeEntry>,
}

impl TypeEntries {
	pub fn new() -> Self {
		Self { entries: Vec::with_capacity(128) }
	}
	pub fn push(&mut self, entry: TypeEntry) -> TypeId {
		let id = self.entries.len() as u32;
		self.entries.push(entry);
		TypeId { entry: id }
	}
	pub fn get(&self, id: TypeId) -> &TypeEntry {
		&self.entries[id.index()]
	}
	pub fn get_mut(&mut self, id: TypeId) -> &mut TypeEntry {
		&mut self.entries[id.index()]
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TypeEntry {
	pub kind: TypeEntryKind,
	pub reference_entries: Option<u32>,
	pub arrays_index: Option<NonZeroU32>,
	pub generic_poisoned: bool,
}

impl TypeEntry {
	pub fn new(kind: TypeEntryKind) -> Self {
		Self { kind, reference_entries: None, arrays_index: None, generic_poisoned: false }
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TypeEntryKind {
	BuiltinType(BuiltinType),
	UserType(UserType),
	Pointer(Pointer),
	Array(Array),
	Slice(Slice),
	Module,
	Type,
}

impl TypeEntryKind {
	pub fn builtin(kind: PrimativeKind, methods_index: usize) -> Self {
		Self::BuiltinType(BuiltinType { kind, methods_index })
	}

	#[inline(always)]
	pub fn builtin_i8(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::i8(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_i16(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::i16(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_i32(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::i32(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_i64(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::i64(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_u8(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::u8(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_u16(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::u16(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_u32(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::u32(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_u64(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::u64(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_f32(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::f32(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_f64(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::f64(), methods_index)
	}
	pub fn builtin_isize(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::isize(), methods_index)
	}
	pub fn builtin_usize(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::usize(), methods_index)
	}
	#[inline(always)]
	pub fn builtin_void(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::Void, methods_index)
	}
	#[inline(always)]
	pub fn builtin_bool(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::Bool, methods_index)
	}
	#[inline(always)]
	pub fn builtin_string(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::String, methods_index)
	}
	#[inline(always)]
	pub fn builtin_string_mut(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::StringMut, methods_index)
	}
	#[inline(always)]
	pub fn builtin_format_string(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::FormatString, methods_index)
	}
	#[inline(always)]
	pub fn builtin_any_collapse(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::AnyCollapse, methods_index)
	}
	#[inline(always)]
	pub fn builtin_no_return(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::NoReturn, methods_index)
	}
	#[inline(always)]
	pub fn builtin_untyped_number(methods_index: usize) -> Self {
		Self::builtin(PrimativeKind::UntypedNumber, methods_index)
	}

	#[inline(always)]
	pub fn user(shape_index: usize, specialization_index: usize, methods_index: usize) -> Self {
		Self::UserType(UserType { shape_index, specialization_index, methods_index })
	}

	#[inline(always)]
	pub fn pointer(type_id: TypeId, mutable: bool) -> Self {
		Self::Pointer(Pointer { type_id, mutable })
	}

	#[inline(always)]
	pub fn array(item_type_id: TypeId, length: u64, array_type_index: usize) -> Self {
		Self::Array(Array { item_type_id, length, array_type_index })
	}

	#[inline(always)]
	pub fn slice(item_type_id: TypeId, mutable: bool) -> Self {
		Self::Slice(Slice { item_type_id, mutable })
	}
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinType {
	pub kind: PrimativeKind,
	pub methods_index: usize,
}
impl BuiltinType {
	pub fn name(&self) -> &str {
		self.kind.name()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct UserType {
	pub shape_index: usize,
	pub specialization_index: usize,
	pub methods_index: usize,
}
impl UserType {
	pub fn name(&self) -> String {
		"user-type".into()
	}
}
#[derive(Debug, Clone, Copy)]
pub struct Pointer {
	pub type_id: TypeId,
	pub mutable: bool,
}

impl Pointer {
	pub fn name(&self, store: &type_store::TypeStore) -> String {
		let inner = store.type_name(self.type_id);
		if self.mutable {
			format!("*mut {inner}")
		} else {
			format!("*{inner}")
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub struct Array {
	pub item_type_id: TypeId,
	pub length: u64,
	pub array_type_index: usize,
}

impl Array {
	pub fn name(&self, _store: &type_store::TypeStore) -> String {
		"UserType".to_owned()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Slice {
	pub item_type_id: TypeId,
	pub mutable: bool,
}

impl Slice {
	pub fn name(&self, store: &type_store::TypeStore) -> String {
		let inner = store.type_name(self.item_type_id);
		if self.mutable {
			format!("*mut [{}]", inner)
		} else {
			format!("*[{inner}]")
		}
	}
}
