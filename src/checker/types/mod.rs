mod display_type;
pub mod monomorphic;
mod store;
mod type_id;

use std::hash::{Hash, Hasher};

use rustc_hash::FxHashMap;
pub use store::*;
pub use type_id::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Void,
	Bool,
	Str,
	String,
	Char,
	Any,
	// Number
	NumRange(NumRange),
	Number(Number),
	Borrow(BorrowType),
	Const(ConstType),
	Fn(FnType),
	ExternFn(ExternFnType),

	// struct
	Struct(StructType),

	// module
	//
	Mod(ModType),

	// internal
	Unit,
	// e.g. T
	Infer(InferType),
}

impl Type {
	pub fn is_numeric(&self) -> bool {
		matches!(self, Type::Number(_) | Type::NumRange(_))
	}
	pub fn is_struct(&self) -> bool {
		matches!(self, Type::Struct(_))
	}

	pub fn can_implemented(&self) -> bool {
		// all of types that can be implemented
		self.is_struct()
	}

	pub fn is_impl(&self) -> bool {
		matches!(self, Type::Struct(StructType { implemeted: true, .. }))
	}

	pub fn set_impl(&mut self, is_impl: bool) {
		if let Type::Struct(struct_type) = self {
			struct_type.set_implemented(is_impl);
		}
	}

	pub fn is_float(&self) -> bool {
		matches!(self, Type::Number(Number::F32) | Type::Number(Number::F64))
	}
	pub fn is_infer(&self) -> bool {
		matches!(self, Type::NumRange(_))
	}
	pub fn is_borrow(&self) -> bool {
		matches!(self, Type::Borrow(_))
	}

	pub fn is_borrow_mut(&self) -> bool {
		matches!(self, Type::Borrow(BorrowType { mutable: true, .. }))
	}
	pub fn is_local_borrow(&self) -> bool {
		matches!(self, Type::Borrow(BorrowType { external: false, .. }))
	}
	pub fn is_external_borrow(&self) -> bool {
		matches!(self, Type::Borrow(BorrowType { external: true, .. }))
	}
	pub fn is_const(&self) -> bool {
		matches!(self, Type::Const(_))
	}
	// gen type_id
	pub fn get_type_id(&self) -> Option<TypeId> {
		match self {
			Type::Void => Some(TypeId::VOID),
			Type::Bool => Some(TypeId::BOOL),
			Type::Str => Some(TypeId::STR),
			Type::String => Some(TypeId::STRING),
			Type::Char => Some(TypeId::CHAR),
			Type::Number(number) => Some(TypeId::from(number)),
			_ => None,
		}
	}

	pub fn get_struct_type(&mut self) -> Option<&mut StructType> {
		if let Type::Struct(struct_type) = self {
			return Some(struct_type);
		}
		None
	}

	pub fn needs_free(&self) -> bool {
		matches!(self, Type::Struct(_))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModType {
	pub name: String,
	pub items: Vec<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumRange {
	pub bits: u8, // bits of the number
	pub is_float: bool,
}

impl NumRange {
	pub fn new(bits: u8, is_float: bool) -> Self {
		assert!(bits <= 64); // don't support more than 64 bits
		Self { bits, is_float }
	}

	pub fn as_float(&self) -> Number {
		match self.bits {
			32 => Number::F32,
			64 => Number::F64,
			_ => unreachable!(),
		}
	}

	pub fn as_number(&self) -> Number {
		if self.is_float {
			return self.as_float();
		}
		match self.bits {
			0..=32 => Number::I32,
			64 => Number::I64,
			_ => unreachable!(),
		}
	}
	pub fn infer_with_type_id(&self, expected: TypeId) -> Option<TypeId> {
		if self.is_float != expected.is_float() {
			return None;
		};
		let number = match expected {
			TypeId::I8 if self.bits <= 8 => TypeId::I8,
			TypeId::I16 if self.bits <= 16 => TypeId::I16,
			TypeId::I32 if self.bits <= 32 => TypeId::I32,
			TypeId::I64 if self.bits <= 64 => TypeId::I64,
			TypeId::F32 if self.bits == 32 => TypeId::F32,
			TypeId::F64 if self.bits == 64 => TypeId::F64,
			_ => return None,
		};
		Some(number)
	}
	pub fn as_infer_number(&self, expected: &Number) -> Option<Number> {
		if self.is_float != expected.is_float() {
			return None;
		};
		let number = match expected {
			Number::I8 if self.bits <= 8 => Number::I8,
			Number::I16 if self.bits <= 16 => Number::I16,
			Number::I32 if self.bits <= 32 => Number::I32,
			Number::I64 if self.bits <= 64 => Number::I64,
			Number::F32 if self.bits == 32 => Number::F32,
			Number::F64 if self.bits == 64 => Number::F64,
			_ => return None,
		};
		Some(number)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Number {
	// isize
	I8,
	I16,
	I32,
	I64,
	Isize,
	// usize
	Usize,
	U8,
	U16,
	U32,
	U64,
	// float
	F32,
	F64,
}

impl Number {
	pub fn is_isize(&self) -> bool {
		matches!(self, Number::I8 | Number::I16 | Number::I32 | Number::I64)
	}
	pub fn is_usize(&self) -> bool {
		matches!(self, Number::Usize | Number::U8 | Number::U16 | Number::U32 | Number::U64)
	}
	pub fn is_float(&self) -> bool {
		matches!(self, Number::F32 | Number::F64)
	}

	pub fn as_type(&self) -> Type {
		match self {
			Number::I8 => Type::Number(Number::I8),
			Number::I16 => Type::Number(Number::I16),
			Number::I32 => Type::Number(Number::I32),
			Number::I64 => Type::Number(Number::I64),
			Number::Isize => Type::Number(Number::Isize),
			Number::Usize => Type::Number(Number::Usize),
			Number::U8 => Type::Number(Number::U8),
			Number::U16 => Type::Number(Number::U16),
			Number::U32 => Type::Number(Number::U32),
			Number::U64 => Type::Number(Number::U64),
			Number::F32 => Type::Number(Number::F32),
			Number::F64 => Type::Number(Number::F64),
		}
	}
}

#[derive(Debug, Clone, Eq, Hash)]
#[allow(renamed_and_removed_lints)]
#[allow(clippy::derive_hash_xor_eq)]
pub struct BorrowType {
	pub value: TypeId,
	pub mutable: bool,
	pub external: bool,
}
impl PartialEq for BorrowType {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value && self.mutable == other.mutable
	}
}

impl BorrowType {
	pub fn new(value: TypeId, mutable: bool, external: bool) -> Self {
		Self { value, mutable, external }
	}

	pub fn change_value(&mut self, value: TypeId) {
		self.value = value;
	}

	pub fn new_external(value: TypeId, mutable: bool) -> Self {
		Self::new(value, mutable, true)
	}

	pub fn new_internal(value: TypeId, mutable: bool) -> Self {
		Self::new(value, mutable, false)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstType {
	pub value: TypeId,
	pub kind: ConstKind,
}

impl ConstType {
	pub fn new(value: TypeId, kind: ConstKind) -> Self {
		Self { value, kind }
	}

	pub fn new_fn(value: TypeId) -> Self {
		Self::new(value, ConstKind::Fn)
	}

	pub fn new_del(value: TypeId) -> Self {
		Self::new(value, ConstKind::Del)
	}

	pub fn is_fn(&self) -> bool {
		matches!(self.kind, ConstKind::Fn)
	}
	pub fn is_del(&self) -> bool {
		matches!(self.kind, ConstKind::Del)
	}
}

// === struct ===
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
	pub name: String,
	// hashmap? name -> FieldType
	pub fields: FxHashMap<String, FieldType>,
	// hasmap? name -> MethodType
	pub fns: FxHashMap<String, TypeId>,
	pub associated: FxHashMap<String, TypeId>,
	pub implemeted: bool,
	pub mutable: bool,
}

impl Hash for StructType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		// ignore internal hash... is it ok?
		for (key, value) in &self.fields {
			key.hash(state);
			value.hash(state);
		}
	}
}

impl StructType {
	pub fn new(name: String) -> Self {
		let associated = FxHashMap::default();
		let fields = FxHashMap::default();
		let fns = FxHashMap::default();
		Self { name, fields, fns, associated, implemeted: false, mutable: false }
	}

	pub fn has_implemented(&self) -> bool {
		self.implemeted
	}

	pub fn is_mutable(&self) -> bool {
		self.mutable
	}

	pub fn set_mutable(&mut self, is_mut: bool) {
		self.mutable = is_mut;
	}

	pub fn set_implemented(&mut self, is_impl: bool) {
		self.implemeted = is_impl;
	}

	pub fn add_field(&mut self, field: FieldType) {
		self.fields.insert(field.name.clone(), field);
	}
	pub fn with_fields(&mut self, fields: Vec<FieldType>) {
		self.fields = fields.into_iter().map(|field| (field.name.clone(), field)).collect();
	}
	pub fn add_fn(&mut self, name: String, fn_id: TypeId) {
		self.fns.insert(name, fn_id);
	}

	pub fn add_associate(&mut self, name: String, type_id: TypeId) {
		self.associated.insert(name, type_id);
	}

	// ceck methods
	//

	pub fn has_fn(&self, name: &str) -> bool {
		self.fns.contains_key(name)
	}

	pub fn has_field(&self, name: &str) -> bool {
		self.fields.contains_key(name)
	}

	// get methods
	pub fn get_field(&self, name: &str) -> Option<&FieldType> {
		self.fields.get(name)
	}

	pub fn get_associate(&self, name: &str) -> Option<&TypeId> {
		self.associated.get(name)
	}

	pub fn get_fn(&self, name: &str) -> Option<&TypeId> {
		self.fns.get(name)
	}

	#[rustfmt::skip]
	pub fn get_type(&self, name: &str) -> Option<TypeId> {
		self.get_field(name).map(|field| field.type_id)
			.or_else(|| self.get_fn(name).copied())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldType {
	pub name: String,
	pub type_id: TypeId,
	pub is_mut: bool,
	pub is_pub: bool,
}

impl FieldType {
	pub fn new(name: String, type_id: TypeId) -> Self {
		Self { name, type_id, is_mut: false, is_pub: false }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodType {
	pub name: String,
	pub args: Vec<TypeId>,
	pub ret: TypeId,
	pub is_pub: bool,
	pub self_id: Option<TypeId>,
}

impl MethodType {
	pub fn new(name: String, args: Vec<TypeId>, ret: TypeId, is_pub: bool) -> Self {
		Self { name, args, ret, is_pub, self_id: None }
	}

	pub fn set_self_id(&mut self, self_id: TypeId) {
		self.self_id = Some(self_id);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstKind {
	Fn,
	Del,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternFnType {
	pub args: Vec<TypeId>,
	pub ret: TypeId,
	pub var_packed: bool,
}

impl ExternFnType {
	pub fn new(args: Vec<TypeId>, ret: TypeId, var_packed: bool) -> Self {
		Self { args, ret, var_packed }
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnType {
	pub args: Vec<TypeId>,
	pub ret: TypeId,
	pub generics: Vec<TypeId>,
	pub is_pub: bool,
}

impl FnType {
	pub fn new(args: Vec<TypeId>, ret: TypeId) -> Self {
		Self { args, ret, generics: vec![], is_pub: false }
	}
	pub fn extend_generics(&mut self, generics: Vec<TypeId>) {
		self.generics.extend(generics);
	}
	pub fn set_generics(&mut self, generics: Vec<TypeId>) {
		self.generics = generics;
	}
	pub fn set_ret(&mut self, ret: TypeId) {
		self.ret = ret;
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InferType {
	pub id: String,
	pub extend: Option<TypeId>,
}

impl From<FnType> for Type {
	fn from(value: FnType) -> Self {
		Type::Fn(value)
	}
}

impl From<NumRange> for Type {
	fn from(value: NumRange) -> Self {
		Type::NumRange(value)
	}
}
impl From<Number> for Type {
	fn from(value: Number) -> Self {
		Type::Number(value)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefType {
	pub mutable: bool,
	pub value: TypeId,
}

impl RefType {
	pub fn new(mutable: bool, value: TypeId) -> Self {
		Self { mutable, value }
	}
}

impl From<BorrowType> for Type {
	fn from(value: BorrowType) -> Self {
		Type::Borrow(value)
	}
}

impl From<ExternFnType> for Type {
	fn from(value: ExternFnType) -> Self {
		Type::ExternFn(value)
	}
}

impl From<InferType> for Type {
	fn from(value: InferType) -> Self {
		Type::Infer(value)
	}
}

impl From<StructType> for Type {
	fn from(value: StructType) -> Self {
		Type::Struct(value)
	}
}
