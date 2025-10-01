use crate::{
	checker::types::{self, TypeEntryKind},
	messages,
	reference::SliceRef,
	symbols::{Symbol, SymbolKind},
};
use types::TypeId;

#[derive(Debug, Clone)]
pub struct TypeStore<'a> {
	pub debug_generics: bool,
	pub debug_type_ids: bool,

	pub primative_type_symbols: SliceRef<Symbol<'a>>,
	// pub method_collections: <Ref<Vec<types::MethodCollection<'a>>,
	pub type_entries: types::TypeEntries,

	// pub array_types: Ref<Vec<Ref<FxHashMap<u64, ArrayType>>>>,
	// pub user_types: Ref<Vec<Ref<UserType<'a>>>>,
	// pub method_collections: Ref<Vec<Ref<MethodCollection<'a>>>>,
	// pub implementations: Ref<Vec<ImplementationInfo>>,
	pub module_type_id: TypeId,
	pub type_type_id: TypeId,
	pub any_collapse_type_id: TypeId,
	pub noreturn_type_id: TypeId,
	pub void_type_id: TypeId,

	pub number_type_id: TypeId,

	pub i8_type_id: TypeId,
	pub i16_type_id: TypeId,
	pub i32_type_id: TypeId,
	pub i64_type_id: TypeId,

	pub u8_type_id: TypeId,
	pub u16_type_id: TypeId,
	pub u32_type_id: TypeId,
	pub u64_type_id: TypeId,

	pub isize_type_id: TypeId,
	pub usize_type_id: TypeId,

	pub f32_type_id: TypeId,
	pub f64_type_id: TypeId,

	pub bool_type_id: TypeId,
	pub string_type_id: TypeId,
	pub string_mut_type_id: TypeId,
	pub format_string_type_id: TypeId,
	// pub u8_slice_type_id: TypeId,
	// pub u8_slice_mut_type_id: TypeId,
}

#[derive(Debug)]
pub struct FunctionStore<'a> {
	pub functions: Vec<Function<'a>>,
}

impl<'a> FunctionStore<'a> {
	pub fn new() -> Self {
		Self { functions: Vec::new() }
	}
}

#[derive(Debug)]
struct Function<'a> {
	name: &'a str,
	generics: Vec<TypeId>,
	return_type: TypeId,
	parameters: Vec<TypeId>,
}

impl<'a> TypeStore<'a> {
	pub fn new() -> Self {
		use types::{NumericKind, PrimativeKind};
		let mut primative_type_symbols = Vec::new();
		let mut type_entries = types::TypeEntries::new();
		let mut method_collections = Vec::new();

		let module_type_id = type_entries.push(types::TypeEntry::new_kind(TypeEntryKind::Module));
		let type_type_id = type_entries.push(types::TypeEntry::new_kind(TypeEntryKind::Type));

		let mut push_primative = |name: Option<&'a str>, kind| {
			let methods_index = method_collections.len();
			method_collections.push(types::MethodCollection::blank());

			let kind = TypeEntryKind::new_builtin_type(kind, methods_index);
			let type_id = type_entries.push(types::TypeEntry::new_kind(kind));

			if let Some(name) = name {
				let kind = SymbolKind::BuiltinType { type_id, methods_index };
				let symbol = Symbol { name, kind, span: None, used: true, imported: false };
				primative_type_symbols.push(symbol);
			}

			type_id
		};

		let any_collapse_type_id = push_primative(None, PrimativeKind::AnyCollapse);
		let noreturn_type_id = push_primative(Some("noreturn"), PrimativeKind::NoReturn);
		let void_type_id = push_primative(Some("void"), PrimativeKind::Void);

		// NOTE: These numeric type ids must all be generated together for the `is_numeric` range check
		let number_type_id = push_primative(None, PrimativeKind::UntypedNumber);

		let i8_type_id = push_primative(Some("i8"), PrimativeKind::Numeric(NumericKind::I8));
		let i16_type_id = push_primative(Some("i16"), PrimativeKind::Numeric(NumericKind::I16));
		let i32_type_id = push_primative(Some("i32"), PrimativeKind::Numeric(NumericKind::I32));
		let i64_type_id = push_primative(Some("i64"), PrimativeKind::Numeric(NumericKind::I64));

		let u8_type_id = push_primative(Some("u8"), PrimativeKind::Numeric(NumericKind::U8));
		let u16_type_id = push_primative(Some("u16"), PrimativeKind::Numeric(NumericKind::U16));
		let u32_type_id = push_primative(Some("u32"), PrimativeKind::Numeric(NumericKind::U32));
		let u64_type_id = push_primative(Some("u64"), PrimativeKind::Numeric(NumericKind::U64));

		let isize_type_id = push_primative(Some("isize"), PrimativeKind::Numeric(NumericKind::ISize));
		let usize_type_id = push_primative(Some("usize"), PrimativeKind::Numeric(NumericKind::USize));

		let f32_type_id = push_primative(Some("f32"), PrimativeKind::Numeric(NumericKind::F32));
		let f64_type_id = push_primative(Some("f64"), PrimativeKind::Numeric(NumericKind::F64));

		let bool_type_id = push_primative(Some("bool"), PrimativeKind::Bool);
		let string_type_id = push_primative(Some("str"), PrimativeKind::String);
		let string_mut_type_id = push_primative(Some("strmut"), PrimativeKind::StringMut);
		let format_string_type_id = push_primative(Some("fstr"), PrimativeKind::FormatString);

		// let u8_slice_type_id = push_primative(None, types::PrimativeKind::Slice(u8_type_id));
		// let u8_slice_mut_type_id = push_primative(None, types::PrimativeKind::SliceMut(u8_type_id));

		TypeStore {
			debug_generics: false,
			debug_type_ids: false,
			primative_type_symbols: SliceRef::from_vec(primative_type_symbols),
			type_entries,
			module_type_id,
			type_type_id,
			any_collapse_type_id,
			noreturn_type_id,
			void_type_id,
			number_type_id,
			i8_type_id,
			i16_type_id,
			i32_type_id,
			i64_type_id,
			u8_type_id,
			u16_type_id,
			u32_type_id,
			u64_type_id,
			isize_type_id,
			usize_type_id,
			f32_type_id,
			f64_type_id,
			bool_type_id,
			string_type_id,
			string_mut_type_id,
			format_string_type_id,
			// u8_slice_type_id,
			// u8_slice_mut_type_id,
		}
	}

	pub fn direct_match(&self, type_id: TypeId, other: TypeId) -> bool {
		type_id.entry == other.entry
	}

	pub fn get_type(&self, type_id: TypeId) -> &types::TypeEntry {
		self.type_entries.get(type_id)
	}

	pub fn get_type_mut(&mut self, type_id: TypeId) -> &mut types::TypeEntry {
		self.type_entries.get_mut(type_id)
	}

	pub fn create_type(&mut self, kind: TypeEntryKind) -> TypeId {
		let entry = types::TypeEntry {
			kind,
			reference_entries: None,
			arrays_index: None,
			generic_poisoned: false,
		};
		self.type_entries.push(entry)
	}

	pub fn get_primitive_type(&self, kind: types::PrimativeKind) -> TypeId {
		use types::NumericKind;
		match kind {
			types::PrimativeKind::NoReturn => self.noreturn_type_id,
			types::PrimativeKind::Void => self.void_type_id,
			types::PrimativeKind::UntypedNumber => self.number_type_id,
			types::PrimativeKind::Bool => self.bool_type_id,
			types::PrimativeKind::String => self.string_type_id,
			types::PrimativeKind::StringMut => self.string_mut_type_id,
			types::PrimativeKind::FormatString => self.format_string_type_id,
			types::PrimativeKind::AnyCollapse => self.any_collapse_type_id,
			types::PrimativeKind::Numeric(numeric) => match numeric {
				NumericKind::I8 => self.i8_type_id,
				NumericKind::I16 => self.i16_type_id,
				NumericKind::I32 => self.i32_type_id,
				NumericKind::I64 => self.i64_type_id,
				NumericKind::U8 => self.u8_type_id,
				NumericKind::U16 => self.u16_type_id,
				NumericKind::U32 => self.u32_type_id,
				NumericKind::U64 => self.u64_type_id,
				NumericKind::ISize => self.isize_type_id,
				NumericKind::USize => self.usize_type_id,
				NumericKind::F32 => self.f32_type_id,
				NumericKind::F64 => self.f64_type_id,
			},
		}
	}
}
