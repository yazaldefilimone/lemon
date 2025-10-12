use super::{layout::Layout, numeric::NumericKind};

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
	pub fn numeric(n: NumericKind) -> Self {
		Self::Numeric(n)
	}

	pub fn i8() -> Self {
		Self::Numeric(NumericKind::I8)
	}
	pub fn i16() -> Self {
		Self::Numeric(NumericKind::I16)
	}
	pub fn i32() -> Self {
		Self::Numeric(NumericKind::I32)
	}
	pub fn i64() -> Self {
		Self::Numeric(NumericKind::I64)
	}
	pub fn u8() -> Self {
		Self::Numeric(NumericKind::U8)
	}
	pub fn u16() -> Self {
		Self::Numeric(NumericKind::U16)
	}
	pub fn u32() -> Self {
		Self::Numeric(NumericKind::U32)
	}
	pub fn u64() -> Self {
		Self::Numeric(NumericKind::U64)
	}

	pub fn usize() -> Self {
		Self::Numeric(NumericKind::USize)
	}
	pub fn isize() -> Self {
		Self::Numeric(NumericKind::ISize)
	}
	pub fn f32() -> Self {
		Self::Numeric(NumericKind::F32)
	}
	pub fn f64() -> Self {
		Self::Numeric(NumericKind::F64)
	}
	pub fn name(self) -> &'static str {
		use PrimativeKind::*;
		match self {
			Bool => "bool",
			Numeric(n) => n.name(),
			String => "str",
			StringMut => "strmut",
			FormatString => "fstr",
			AnyCollapse => "any collapse",
			NoReturn => "noreturn",
			Void => "void",
			UntypedNumber => "untyped number",
		}
	}

	pub fn layout(self) -> Layout {
		use PrimativeKind::*;
		match self {
			AnyCollapse | NoReturn | Void => Layout { size: 0, alignment: 1 },
			UntypedNumber => unreachable!(),
			Bool => Layout { size: 1, alignment: 1 },
			Numeric(n) => n.layout(),
			String | StringMut | FormatString => Layout { size: 16, alignment: 8 },
		}
	}
}
