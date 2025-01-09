use serde::{Deserialize, Serialize};

use super::Number;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeId(pub(crate) u64);

impl TypeId {
	pub const VOID: TypeId = TypeId(0);
	pub const BOOL: TypeId = TypeId(1);
	pub const STR: TypeId = TypeId(2);
	pub const STRING: TypeId = TypeId(3);
	pub const CHAR: TypeId = TypeId(4);

	// isize
	pub const I8: TypeId = TypeId(5);
	pub const I16: TypeId = TypeId(6);
	pub const I32: TypeId = TypeId(7);
	pub const I64: TypeId = TypeId(8);
	pub const ISIZE: TypeId = TypeId(9);
	// usize
	pub const U8: TypeId = TypeId(10);
	pub const U16: TypeId = TypeId(11);
	pub const U32: TypeId = TypeId(12);
	pub const U64: TypeId = TypeId(13);
	pub const USIZE: TypeId = TypeId(14);
	// float
	pub const F32: TypeId = TypeId(15);
	pub const F64: TypeId = TypeId(16);

	// internal
	pub const NOTHING: TypeId = TypeId(17);

	pub const LENGTH: usize = 18; // internal

	// methods
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
	pub fn is_known(&self) -> bool {
		self.0 < TypeId::LENGTH as u64
	}

	pub fn is_nothing(&self) -> bool {
		self.0 == TypeId::NOTHING.0
	}

	pub fn is_void(&self) -> bool {
		self.0 == TypeId::VOID.0
	}

	pub fn is_number(&self) -> bool {
		self.0 >= TypeId::I8.0 && self.0 <= TypeId::F64.0
	}

	pub fn is_float(&self) -> bool {
		self.0 >= TypeId::F32.0 && self.0 <= TypeId::F64.0
	}

	// pub fn is_infer(&self) -> bool {
	// 	self.0 >= TypeId::INFER.0
	// }
}

impl From<&Number> for TypeId {
	fn from(number: &Number) -> Self {
		match number {
			Number::I8 => TypeId::I8,
			Number::I16 => TypeId::I16,
			Number::I32 => TypeId::I32,
			Number::I64 => TypeId::I64,
			Number::Isize => TypeId::ISIZE,
			Number::Usize => TypeId::USIZE,
			Number::U8 => TypeId::U8,
			Number::U16 => TypeId::U16,
			Number::U32 => TypeId::U32,
			Number::U64 => TypeId::U64,
			Number::F32 => TypeId::F32,
			Number::F64 => TypeId::F64,
		}
	}
}
