use crate::types::{self, TypeEntry, TypeEntryKind};

#[derive(Debug)]
pub struct BuiltinTypes {
	pub any_collapse: types::TypeId,
	pub no_return: types::TypeId,
	pub module: types::TypeId,
	pub type_type: types::TypeId,
	pub void: types::TypeId,

	pub number: types::TypeId,
	pub i8: types::TypeId,
	pub i16: types::TypeId,
	pub i32: types::TypeId,
	pub i64: types::TypeId,
	pub u8: types::TypeId,
	pub u16: types::TypeId,
	pub u32: types::TypeId,
	pub u64: types::TypeId,

	pub isize: types::TypeId,
	pub usize: types::TypeId,

	pub f32: types::TypeId,
	pub f64: types::TypeId,

	pub bool: types::TypeId,
	pub string: types::TypeId,
	pub string_mutable: types::TypeId,
	pub format_string: types::TypeId,

	pub char: types::TypeId,
	pub pointer: types::TypeId,
	pub slice: types::TypeId,
	pub array: types::TypeId,
	pub tuple: types::TypeId,
	pub function: types::TypeId,
}

impl BuiltinTypes {
	pub fn new(entries: &mut types::TypeEntries) -> Self {
		let any_collapse = entries.push(TypeEntry::new(TypeEntryKind::builtin_any_collapse(0)));
		let no_return = entries.push(TypeEntry::new(TypeEntryKind::builtin_no_return(0)));
		let module = entries.push(TypeEntry::new(TypeEntryKind::Module));
		let type_type = entries.push(TypeEntry::new(TypeEntryKind::Type));
		let void = entries.push(TypeEntry::new(TypeEntryKind::builtin_void(0)));

		let number = entries.push(TypeEntry::new(TypeEntryKind::builtin_untyped_number(0)));

		let i8 = entries.push(TypeEntry::new(TypeEntryKind::builtin_i8(0)));
		let i16 = entries.push(TypeEntry::new(TypeEntryKind::builtin_i16(0)));
		let i32 = entries.push(TypeEntry::new(TypeEntryKind::builtin_i32(0)));
		let i64 = entries.push(TypeEntry::new(TypeEntryKind::builtin_i64(0)));
		let u8 = entries.push(TypeEntry::new(TypeEntryKind::builtin_u8(0)));
		let u16 = entries.push(TypeEntry::new(TypeEntryKind::builtin_u16(0)));
		let u32 = entries.push(TypeEntry::new(TypeEntryKind::builtin_u32(0)));
		let u64 = entries.push(TypeEntry::new(TypeEntryKind::builtin_u64(0)));

		let isize = entries.push(TypeEntry::new(TypeEntryKind::builtin_isize(0)));
		let usize = entries.push(TypeEntry::new(TypeEntryKind::builtin_usize(0)));
		let f32 = entries.push(TypeEntry::new(TypeEntryKind::builtin_f32(0)));
		let f64 = entries.push(TypeEntry::new(TypeEntryKind::builtin_f64(0)));

		let bool = entries.push(TypeEntry::new(TypeEntryKind::builtin_void(0)));
		let string = entries.push(TypeEntry::new(TypeEntryKind::builtin_string(0)));
		let string_mutable = entries.push(TypeEntry::new(TypeEntryKind::builtin_string_mut(0)));
		let format_string = entries.push(TypeEntry::new(TypeEntryKind::builtin_format_string(0)));

		let char = entries.push(TypeEntry::new(TypeEntryKind::builtin_i32(0))); // ou outro tipo de char
		let pointer = entries.push(TypeEntry::new(TypeEntryKind::pointer(number, false)));
		let slice = entries.push(TypeEntry::new(TypeEntryKind::slice(number, false)));
		let array = entries.push(TypeEntry::new(TypeEntryKind::array(number, 0, 0)));
		let tuple = entries.push(TypeEntry::new(TypeEntryKind::Type)); // placeholder
		let function = entries.push(TypeEntry::new(TypeEntryKind::Type)); // placeholder

		Self {
			any_collapse,
			void,
			bool,
			string,
			string_mutable,
			format_string,
			number,
			i8,
			i16,
			i32,
			i64,
			u8,
			u16,
			u32,
			u64,
			f32,
			f64,
			char,
			pointer,
			slice,
			array,
			tuple,
			function,
			no_return,
			module,
			type_type,
			isize,
			usize,
		}
	}
}
