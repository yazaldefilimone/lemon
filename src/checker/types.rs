use std::num::NonZeroU32;

// TODO: This should probably be a u64
#[derive(Debug, Clone, Copy, Hash)]
pub struct TypeId {
	entry: u32,
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
}

impl PrimativeKind {
	pub fn name(self) -> &'static str {
		match self {
			PrimativeKind::NoReturn => "noreturn",
			PrimativeKind::Void => "void",
			PrimativeKind::UntypedNumber => "untyped number",
			PrimativeKind::Bool => "bool",
			PrimativeKind::Numeric(numeric) => numeric.name(),
			PrimativeKind::String => "str",
			PrimativeKind::StringMut => "strmut",
			PrimativeKind::FormatString => "fstr",
		}
	}

	pub fn layout(self) -> Layout {
		match self {
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
	kind: PrimativeKind,
	methods_index: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct UserType {
	shape_index: usize,
	specialization_index: usize,
	methods_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Pointer {
	type_id: TypeId,
	mutable: bool,
}

#[derive(Debug, Clone)]
pub struct TypeEntries {
	local_chunks: Vec<Option<Vec<TypeEntry>>>,
	global_chunks: Vec<Vec<TypeEntry>>,
}

const TYPE_ENTRY_CHUNK_MAX_LENGTH: usize = 50;

impl TypeEntries {
	pub fn new() -> TypeEntries {
		TypeEntries {
			local_chunks: Vec::new(),
			global_chunks: vec![Vec::with_capacity(TYPE_ENTRY_CHUNK_MAX_LENGTH)],
		}
	}

	pub fn push_entry(&mut self, entry: TypeEntry) -> TypeId {
		let last_chunk = self.global_chunks.last_mut().unwrap();

		if last_chunk.len() >= TYPE_ENTRY_CHUNK_MAX_LENGTH {
			self.global_chunks.push(Vec::with_capacity(TYPE_ENTRY_CHUNK_MAX_LENGTH));
		}

		let full_chunks = self.global_chunks.len() - 1;
		let chunk = self.global_chunks.last_mut().unwrap();
		let index = chunk.len() + (full_chunks * TYPE_ENTRY_CHUNK_MAX_LENGTH);
		chunk.push(entry);
		TypeId { entry: index as u32 }
	}

	pub fn get(&mut self, type_id: TypeId) -> TypeEntry {
		let overall_index = type_id.entry as usize;
		let index = overall_index % TYPE_ENTRY_CHUNK_MAX_LENGTH;
		let chunk_index = overall_index / TYPE_ENTRY_CHUNK_MAX_LENGTH;

		// Atualiza local chunk se necessário
		if chunk_index >= self.local_chunks.len() || self.local_chunks[chunk_index].is_none() {
			self.update_chunk(chunk_index);
		}

		self.local_chunks[chunk_index].as_ref().unwrap()[index]
	}

	fn update_chunk(&mut self, chunk_index: usize) {
		while self.local_chunks.len() <= chunk_index {
			self.local_chunks.push(None);
		}

		let local_chunk = &mut self.local_chunks[chunk_index];
		if local_chunk.is_some() {
			local_chunk.as_mut().unwrap().clear();
			local_chunk.as_mut().unwrap().extend_from_slice(&self.global_chunks[chunk_index]);
		} else {
			*local_chunk = Some(self.global_chunks[chunk_index].clone());
		}
	}
}
