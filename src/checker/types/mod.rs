use std::num::NonZeroU32;
pub mod reference;
pub mod store;
use rustc_hash::FxHashMap;

use crate::{ast, checker::types::store::TypeStore, hir};

// TODO: This should probably be a u64
#[derive(Debug, Clone, Copy, Hash)]
pub struct TypeId {
	pub entry: u32,
}

impl TypeId {
	pub fn index(self) -> usize {
		self.entry as usize
	}

	pub fn unusable() -> TypeId {
		TypeId { entry: u32::MAX }
	}

	pub fn is_any_collapse(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.any_collapse_type_id)
	}

	pub fn is_noreturn(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.noreturn_type_id)
	}

	pub fn is_void(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.void_type_id)
	}

	pub fn is_untyped_number(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.number_type_id)
	}
	pub fn is_numeric(self, type_store: &TypeStore) -> bool {
		let range = type_store.number_type_id.entry..=type_store.f64_type_id.entry;
		range.contains(&self.entry) || self.is_any_collapse(type_store)
	}

	pub fn is_integer(self, type_store: &TypeStore, expression: &hir::Expression) -> bool {
		let range = type_store.i8_type_id.entry..=type_store.usize_type_id.entry;

		if range.contains(&self.entry) || self.is_any_collapse(type_store) {
			return true;
		}

		match &expression.kind {
			hir::ExpressionKind::NumberValue(value) => value.is_integer(),
			_ => false,
		}
	}

	pub fn is_pointer(self, type_store: &mut TypeStore) -> bool {
		let entry = type_store.type_entries.get(self);
		matches!(entry.kind, TypeEntryKind::Pointer { .. })
	}
	pub fn is_bool(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.bool_type_id)
	}

	pub fn is_string(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.string_type_id)
	}

	pub fn is_string_mut(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.string_mut_type_id)
	}

	pub fn is_format_string(self, type_store: &TypeStore) -> bool {
		type_store.direct_match(self, type_store.format_string_type_id)
	}

	pub fn as_pointed(self, type_store: &mut TypeStore) -> Option<AsPointed> {
		let entry = type_store.type_entries.get(self);
		match entry.kind {
			TypeEntryKind::Pointer(pointer) => Some(pointer.as_pointed()),
			_ => None,
		}
	}
}

pub struct AsPointed {
	pub type_id: TypeId,
	pub mutable: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Layout {
	pub size: i64,
	pub alignment: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericKind {
	I8,
	I16,
	I32,
	I64,

	U8,
	U16,
	U32,
	U64,

	ISize,
	USize,

	F32,
	F64,
}

impl std::fmt::Display for NumericKind {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

impl NumericKind {
	pub fn name(self) -> &'static str {
		match self {
			NumericKind::I8 => "i8",
			NumericKind::I16 => "i16",
			NumericKind::I32 => "i32",
			NumericKind::I64 => "i64",
			NumericKind::U8 => "u8",
			NumericKind::U16 => "u16",
			NumericKind::U32 => "u32",
			NumericKind::U64 => "u64",
			NumericKind::ISize => "isize",
			NumericKind::USize => "usize",
			NumericKind::F32 => "f32",
			NumericKind::F64 => "f64",
		}
	}

	pub fn layout(self) -> Layout {
		match self {
			Self::ISize | Self::USize => Layout {
				size: std::mem::size_of::<usize>() as i64,
				alignment: std::mem::align_of::<usize>() as i64,
			},
			// NumericKind::ISize => Layout { size: 8, alignment: 8 },
			// NumericKind::USize => Layout { size: 8, alignment: 8 },
			NumericKind::I8 => Layout { size: 1, alignment: 1 },
			NumericKind::I16 => Layout { size: 2, alignment: 2 },
			NumericKind::I32 => Layout { size: 4, alignment: 4 },
			NumericKind::I64 => Layout { size: 8, alignment: 8 },
			NumericKind::U8 => Layout { size: 1, alignment: 1 },
			NumericKind::U16 => Layout { size: 2, alignment: 2 },
			NumericKind::U32 => Layout { size: 4, alignment: 4 },
			NumericKind::U64 => Layout { size: 8, alignment: 8 },

			NumericKind::F32 => Layout { size: 4, alignment: 4 },
			NumericKind::F64 => Layout { size: 8, alignment: 8 },
		}
	}

	pub fn is_signed(self) -> bool {
		match self {
			NumericKind::I8
			| NumericKind::I16
			| NumericKind::I32
			| NumericKind::I64
			| NumericKind::ISize => true,
			NumericKind::U8
			| NumericKind::U16
			| NumericKind::U32
			| NumericKind::U64
			| NumericKind::USize => false,
			NumericKind::F32 | NumericKind::F64 => true,
		}
	}

	pub fn max_value(self) -> i128 {
		let bit_count = self.layout().size as u32 * 8;
		if self.is_signed() {
			i128::pow(2, bit_count - 1) - 1
		} else {
			i128::pow(2, bit_count) - 1
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimativeKind {
	NoReturn,
	Void,

	UntypedNumber,

	Bool,
	Numeric(NumericKind),
	String,
	StringMut,
	FormatString,
	AnyCollapse,
}

impl PrimativeKind {
	pub fn name(self) -> &'static str {
		match self {
			PrimativeKind::Bool => "bool",
			PrimativeKind::Numeric(numeric) => numeric.name(),
			PrimativeKind::String => "str",
			PrimativeKind::StringMut => "strmut",
			PrimativeKind::FormatString => "fstr",
			PrimativeKind::AnyCollapse => "any collapse",
			PrimativeKind::NoReturn => "noreturn",
			PrimativeKind::Void => "void",
			PrimativeKind::UntypedNumber => "untyped number",
		}
	}

	pub fn layout(self) -> Layout {
		match self {
			PrimativeKind::AnyCollapse => Layout { size: 0, alignment: 1 },
			PrimativeKind::NoReturn => Layout { size: 0, alignment: 1 },
			PrimativeKind::Void => Layout { size: 0, alignment: 1 },
			PrimativeKind::UntypedNumber => unreachable!(),
			PrimativeKind::Bool => Layout { size: 1, alignment: 1 },
			PrimativeKind::Numeric(numeric) => numeric.layout(),
			PrimativeKind::String | PrimativeKind::StringMut | PrimativeKind::FormatString => {
				Layout { size: 8 * 2, alignment: 8 }
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TypeEntry {
	pub kind: TypeEntryKind,
	pub reference_entries: Option<u32>, // TODO: Use niche
	pub arrays_index: Option<NonZeroU32>,
	pub generic_poisoned: bool,
}

impl TypeEntry {
	pub fn new(
		kind: TypeEntryKind,
		reference_entries: Option<u32>,
		arrays_index: Option<NonZeroU32>,
		generic_poisoned: bool,
	) -> Self {
		Self { kind, reference_entries, arrays_index, generic_poisoned }
	}
	pub fn new_kind(kind: TypeEntryKind) -> Self {
		Self { kind, reference_entries: None, arrays_index: None, generic_poisoned: false }
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TypeEntryKind {
	Module,
	Type,
	BuiltinType(BuiltinType),
	UserType(UserType),
	Pointer(Pointer),
	Array(Array),
	Slice(Slice),
}

impl TypeEntryKind {
	#[allow(dead_code)] // TODO: Do we need to keep this around?
	pub fn name(self) -> &'static str {
		match self {
			TypeEntryKind::Module => "module",
			TypeEntryKind::Type | TypeEntryKind::BuiltinType { .. } | TypeEntryKind::UserType { .. } => {
				"type"
			}
			TypeEntryKind::Pointer { .. } => "pointer",
			TypeEntryKind::Array(_) => "array",
			TypeEntryKind::Slice(_) => "slice",
		}
	}

	pub fn methods_index(self) -> Option<usize> {
		use TypeEntryKind::*;
		match self {
			BuiltinType(builtin_type) => Some(builtin_type.methods_index),
			UserType(user_type) => Some(user_type.methods_index),
			_ => None,
		}
	}
	pub fn new_user_type(
		shape_index: usize,
		specialization_index: usize,
		methods_index: usize,
	) -> TypeEntryKind {
		TypeEntryKind::UserType(UserType { shape_index, specialization_index, methods_index })
	}

	pub fn new_builtin_type(kind: PrimativeKind, methods_index: usize) -> TypeEntryKind {
		TypeEntryKind::BuiltinType(BuiltinType { kind, methods_index })
	}

	pub fn new_pointer(type_id: TypeId, mutable: bool) -> TypeEntryKind {
		TypeEntryKind::Pointer(Pointer { type_id, mutable })
	}

	pub fn new_array(item_type_id: TypeId, length: u64, array_type_index: usize) -> TypeEntryKind {
		TypeEntryKind::Array(Array { item_type_id, length, array_type_index })
	}

	pub fn new_slice(item_type_id: TypeId, mutable: bool) -> TypeEntryKind {
		TypeEntryKind::Slice(Slice { item_type_id, mutable })
	}

	// pub fn fallback_methods_index(self, type_store: &mut TypeStore) -> Option<usize> {
	// 	match self {
	// 		TypeEntryKind::BuiltinType(builtin_type) if builtin_type.kind == PrimativeKind::StringMut => {
	// 			let string_entry = type_store.type_entries.get(type_store.string_type_id);
	// 			string_entry.kind.methods_index()
	// 		}

	// 		_ => None,
	// 	}
	// }
}

#[derive(Debug)]
pub struct MethodCollection<'a> {
	pub methods_by_name: FxHashMap<&'a str, usize>, // Indicies into methods vec below
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

#[derive(Debug, Clone, Copy)]
pub struct Array {
	pub item_type_id: TypeId,
	pub length: u64,
	pub array_type_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Slice {
	pub item_type_id: TypeId,
	pub mutable: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinType {
	pub kind: PrimativeKind,
	pub methods_index: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct UserType {
	pub shape_index: usize,
	pub specialization_index: usize,
	pub methods_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Pointer {
	pub type_id: TypeId,
	pub mutable: bool,
}

impl Pointer {
	pub fn as_pointed(self) -> AsPointed {
		AsPointed { type_id: self.type_id, mutable: self.mutable }
	}
}

#[derive(Debug, Clone)]
pub struct TypeEntries {
	entries: Vec<TypeEntry>,
}

impl TypeEntries {
	pub fn new() -> TypeEntries {
		TypeEntries {
			entries: Vec::with_capacity(128), // Pre-allocate for common case
		}
	}

	pub fn push(&mut self, entry: TypeEntry) -> TypeId {
		let id = self.entries.len() as u32;
		self.entries.push(entry);
		TypeId { entry: id }
	}

	pub fn get(&self, type_id: TypeId) -> &TypeEntry {
		&self.entries[type_id.index()]
	}

	pub fn get_mut(&mut self, type_id: TypeId) -> &mut TypeEntry {
		&mut self.entries[type_id.index()]
	}

	pub fn iter(&self) -> impl Iterator<Item = (TypeId, &TypeEntry)> {
		self.entries.iter().enumerate().map(|(i, entry)| (TypeId { entry: i as u32 }, entry))
	}

	pub fn len(&self) -> usize {
		self.entries.len()
	}

	pub fn is_empty(&self) -> bool {
		self.entries.is_empty()
	}
}
