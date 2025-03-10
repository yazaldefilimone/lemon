use super::{
	BorrowType, ConstType, ExternFnType, FieldType, FnType, InferType, MethodType, ModType, NumRange,
	Number, StructType, Type, TypeId, TypeStore,
};

impl Type {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		match self {
			Type::Void => *text += "void",
			Type::Bool => *text += "bool",
			Type::Str => *text += "str",
			Type::String => *text += "string",
			Type::Char => *text += "char",
			Type::Unit => *text += "unit",
			Type::Any => *text += "any",
			Type::Number(number) => number.display_type(text),
			Type::NumRange(num_range) => num_range.display_type(text),
			Type::Infer(infer) => infer.display_type(text, type_store, omit),
			Type::Borrow(borrow) => borrow.display_type(text, type_store, omit),
			Type::Const(const_type) => const_type.display_type(text, type_store, omit),
			Type::Fn(fn_type) => fn_type.display_type(text, type_store, omit),
			Type::ExternFn(extern_fn_type) => extern_fn_type.display_type(text, type_store, omit),
			Type::Struct(struct_type) => struct_type.display_type(text, type_store, omit),
			Type::Mod(mod_type) => mod_type.display_type(text, type_store, omit),
		}
	}
	pub fn display_ir_type(&self, text: &mut String, type_store: &TypeStore) {
		self.display_type(text, type_store, true);
	}
}

impl Number {
	fn display_type(&self, text: &mut String) {
		match self {
			Number::I8 => *text += "i8",
			Number::I16 => *text += "i16",
			Number::I32 => *text += "i32",
			Number::I64 => *text += "i64",
			Number::Isize => *text += "isize",
			Number::Usize => *text += "usize",
			Number::U8 => *text += "u8",
			Number::U16 => *text += "u16",
			Number::U32 => *text += "u32",
			Number::U64 => *text += "u64",
			Number::F32 => *text += "f32",
			Number::F64 => *text += "f64",
		}
	}
}

impl StructType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		if omit {
			*text += &self.name;
			return;
		}
		*text += "struct ";
		*text += &self.name;
		// if !self.generics.is_empty() {
		// 	*text += "<";
		// 	for (i, generic) in self.generics.iter().enumerate() {
		// 		if i > 0 {
		// 			*text += ", ";
		// 		}
		// 		generic.display_type(text, type_store);
		// 	}
		// 	*text += ">";
		// }
		*text += " { ";
		for (i, (_, field)) in self.fields.iter().enumerate() {
			// display 2 fields
			if i > 1 {
				*text += ", ..";
				*text += (self.fields.len() - i).to_string().as_str();
				break;
			}

			// ----------
			if i > 0 {
				*text += ", ";
			}
			*text += &field.name;
			*text += ": ";
			*text += &type_store.get_display_type(field.type_id);
		}
		*text += " }";
	}
}

impl FieldType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore) {
		*text += &self.name;
		*text += ": ";
		*text += &type_store.get_display_type(self.type_id);
	}
}

impl MethodType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore) {
		*text += &self.name;
		*text += "(";
		for (i, param) in self.args.iter().enumerate() {
			if i > 0 {
				*text += ", ";
			}
			*text += &type_store.get_display_type(*param);
		}
		*text += ")";
		*text += " -> ";
		*text += &type_store.get_display_type(self.ret);
	}
}

impl BorrowType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		*text += "&";
		if self.mutable {
			*text += "mut ";
		}
		let value = type_store.get_type(self.value).unwrap();
		value.display_type(text, type_store, omit);
	}
}
impl ConstType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		// match self.kind {
		// 	ConstKind::Fn => *text += "fn",
		// 	ConstKind::Del => *text += "del",
		// }
		let type_value = type_store.get_type(self.value).unwrap();
		type_value.display_type(text, type_store, omit);
	}
}

impl FnType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		*text += "fn(";
		for (i, arg) in self.args.iter().enumerate() {
			if i > 0 {
				*text += ", ";
			}
			let type_value = type_store.get_type(*arg).unwrap();
			type_value.display_type(text, type_store, omit);
		}
		*text += ") -> ";
		let type_value = type_store.get_type(self.ret).unwrap();
		type_value.display_type(text, type_store, omit);
	}
}

impl ExternFnType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		*text += "extern fn(";
		for (i, arg) in self.args.iter().enumerate() {
			if i > 0 {
				*text += ", ";
			}
			let type_value = type_store.get_type(*arg).unwrap();
			type_value.display_type(text, type_store, omit);
		}

		if self.var_packed {
			*text += ", ...";
		}
		*text += ") -> ";
		let type_value = type_store.get_type(self.ret).unwrap();
		type_value.display_type(text, type_store, omit);
	}
}

impl NumRange {
	pub fn display_type(&self, text: &mut String) {
		// if self.is_float {
		// 	*text += "f";
		// } else {
		// 	*text += "i";
		// }
		// *text += "?";
		self.as_number().display_type(text);
	}
}

impl InferType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		*text += &self.id;
		if let Some(extend) = &self.extend {
			*text += ": ";
			extend.display_type(text, type_store, omit);
		}
	}
}

impl TypeId {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		match *self {
			TypeId::VOID => *text += "void",
			TypeId::BOOL => *text += "bool",
			TypeId::STR => *text += "str",
			TypeId::STRING => *text += "string",
			TypeId::CHAR => *text += "char",
			TypeId::I8 => *text += "i8",
			TypeId::I16 => *text += "i16",
			TypeId::I32 => *text += "i32",
			TypeId::I64 => *text += "i64",
			TypeId::ISIZE => *text += "isize",
			TypeId::U8 => *text += "u8",
			TypeId::U16 => *text += "u16",
			TypeId::U32 => *text += "u32",
			TypeId::U64 => *text += "u64",
			TypeId::USIZE => *text += "usize",
			TypeId::F32 => *text += "f32",
			TypeId::F64 => *text += "f64",
			_ => {
				let type_value = type_store.get_type(*self).unwrap();
				type_value.display_type(text, type_store, omit);
			}
		}
	}
}

impl ModType {
	pub fn display_type(&self, text: &mut String, type_store: &TypeStore, omit: bool) {
		*text += "mod ";
		*text += &self.name;
		// *text += " { ";
		// for (i, item) in self.items.iter().enumerate() {
		// 	if i > 0 {
		// 		*text += ", ";
		// 	}
		// 	*text += &type_store.get_display_type(*item);
		// }
		// *text += " }";
	}
}
