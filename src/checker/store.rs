use crate::{
	checker::types::{TypeEntries, TypeId},
	reference::SliceRef,
	symbols::Symbol,
};

#[derive(Debug, Clone)]
pub struct TypeStore<'a> {
	pub debug_generics: bool,
	pub debug_type_ids: bool,

	pub primative_type_symbols: SliceRef<Symbol<'a>>,

	pub type_entries: TypeEntries,

	// pub array_types: Ref<Vec<Ref<FxHashMap<u64, ArrayType>>>>,
	// pub user_types: Ref<Vec<Ref<UserType<'a>>>>,
	// pub method_collections: Ref<Vec<Ref<MethodCollection<'a>>>>,
	// pub implementations: Ref<Vec<ImplementationInfo>>,
	module_type_id: TypeId,
	type_type_id: TypeId,
	any_collapse_type_id: TypeId,
	noreturn_type_id: TypeId,
	void_type_id: TypeId,

	number_type_id: TypeId,

	i8_type_id: TypeId,
	i16_type_id: TypeId,
	i32_type_id: TypeId,
	i64_type_id: TypeId,

	u8_type_id: TypeId,
	u16_type_id: TypeId,
	u32_type_id: TypeId,
	u64_type_id: TypeId,

	isize_type_id: TypeId,
	usize_type_id: TypeId,

	f32_type_id: TypeId,
	f64_type_id: TypeId,

	bool_type_id: TypeId,
	string_type_id: TypeId,
	string_mut_type_id: TypeId,
	format_string_type_id: TypeId,

	u8_slice_type_id: TypeId,
	u8_slice_mut_type_id: TypeId,
}

#[derive(Debug)]
pub struct FunctionStore<'a> {
	functions: Vec<Function<'a>>,
}

impl<'a> FunctionStore<'a> {
	fn new() -> Self {
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
