use super::{type_id::TypeId, Number, Type};

#[derive(Debug)]
pub struct TypeStore {
	types: Vec<Type>,
}

impl TypeStore {
	pub fn new(types: Vec<Type>) -> Self {
		Self { types }
	}
	pub fn add_type(&mut self, ty: Type) -> TypeId {
		// todo: cache other type_id.. e.g. NumRage... or ConstType...
		let next_id = self.types.len();
		self.types.push(ty);
		TypeId(next_id as u64)
	}

	pub fn get_type(&self, type_id: TypeId) -> Option<&Type> {
		self.types.get(type_id.as_usize())
	}
}

impl Default for TypeStore {
	fn default() -> Self {
		let types = vec![
			Type::Void,   // 0
			Type::Bool,   // 1
			Type::Str,    // 2
			Type::String, // 3
			Type::Char,   // 4
			// isize
			Number::I8.as_type(),    // 5
			Number::I16.as_type(),   // 6
			Number::I32.as_type(),   // 7
			Number::I64.as_type(),   // 8
			Number::Isize.as_type(), // 9
			// usize
			Number::U8.as_type(),    // 10
			Number::U16.as_type(),   // 11
			Number::U32.as_type(),   // 12
			Number::U64.as_type(),   // 13
			Number::Usize.as_type(), // 14
			// float
			Number::F32.as_type(), // 15
			Number::F64.as_type(), // 16
			// internal
			Type::Nothing, // 17
		];
		assert_eq!(types.len(), TypeId::LENGTH);
		Self::new(types)
	}
}
