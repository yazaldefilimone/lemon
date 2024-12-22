use super::{Checker, TypeResult};

use super::types::{Type, TypeId, UsizeType};
use crate::checker::types::IntType;

impl Checker<'_> {
	pub fn infer_type(&self, expected: TypeId, found: TypeId) -> TypeResult<TypeId> {
		match self.get_stored_type(found)? {
			Type::InferInt { bits } => self.infer_int_type(*bits, expected, found),
			// Type::InferFloat { bits } => self.infer_float_type(bits, lt_type_id),
			_ => Ok(found),
		}
	}

	fn infer_int_type(&self, bits: u8, expected: TypeId, found_id: TypeId) -> TypeResult<TypeId> {
		match self.get_stored_type(expected)? {
			Type::Int(i) => match i {
				IntType::I8 if bits == 8 => Ok(TypeId::I8),
				IntType::I16 if bits <= 16 => Ok(TypeId::I16),
				IntType::I32 if bits <= 32 => Ok(TypeId::I32),
				IntType::I32 if bits <= 64 => Ok(TypeId::I32),
				IntType::Int if bits <= 64 => Ok(TypeId::INT), // todo we need to check 64????
				_ => Ok(found_id),
			},
			Type::Usize(u) => match u {
				UsizeType::U8 if bits == 8 => Ok(TypeId::U8),
				UsizeType::U16 if bits <= 16 => Ok(TypeId::U16),
				UsizeType::U32 if bits <= 32 => Ok(TypeId::U32),
				UsizeType::U32 if bits <= 64 => Ok(TypeId::U32),
				UsizeType::U32 if bits <= 64 => Ok(TypeId::U32),
				UsizeType::Usize if bits <= 64 => Ok(TypeId::USIZE), // todo
				_ => Ok(found_id),
			},
			_ => Ok(found_id),
		}
	}

	pub fn infer_no_type_anotation(&self, type_id: TypeId) -> TypeResult<TypeId> {
		match self.get_stored_type(type_id)? {
			Type::InferInt { bits } => Ok(TypeId::I32), // default is i32
			_ => Ok(type_id),
		}
	}
}
