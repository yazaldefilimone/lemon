use super::layout::Layout;

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

impl NumericKind {
	pub fn name(self) -> &'static str {
		use NumericKind::*;
		match self {
			I8 => "i8",
			I16 => "i16",
			I32 => "i32",
			I64 => "i64",
			U8 => "u8",
			U16 => "u16",
			U32 => "u32",
			U64 => "u64",
			ISize => "isize",
			USize => "usize",
			F32 => "f32",
			F64 => "f64",
		}
	}

	pub fn layout(self) -> Layout {
		use NumericKind::*;
		match self {
			ISize | USize => Layout {
				size: std::mem::size_of::<usize>() as i64,
				alignment: std::mem::align_of::<usize>() as i64,
			},
			I8 | U8 => Layout { size: 1, alignment: 1 },
			I16 | U16 => Layout { size: 2, alignment: 2 },
			I32 | U32 | F32 => Layout { size: 4, alignment: 4 },
			I64 | U64 | F64 => Layout { size: 8, alignment: 8 },
		}
	}

	pub fn is_signed(self) -> bool {
		matches!(
			self,
			Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::ISize | Self::F32 | Self::F64
		)
	}
}
