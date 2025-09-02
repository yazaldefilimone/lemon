use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	ops::{Add, AddAssign},
};

use crate::ast::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
	pub start: usize,
	pub end: usize,
	pub file: u32,
	pub line: u32,
}

impl Span {
	#[inline]
	pub fn new(start: usize, end: usize, file: u32, line: u32) -> Self {
		Self { start, end, file, line }
	}

	pub fn unusable() -> Span {
		Span { start: usize::MAX, end: usize::MAX, file: u32::MAX, line: u32::MAX }
	}

	pub fn debug_location(self, parsed_files: &[File]) -> Location {
		let file = &parsed_files[self.file as usize];
		// let line_start = file.line_starts[self.line as usize];
		// let offset_in_line = self.start - line_start + 1;

		Location { file: self.file, line: self.line + 1 }
	}
}
impl Display for Span {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}..{} | line {}", self.start, self.end, self.line)
	}
}

impl Add for Span {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		debug_assert_eq!(self.file, other.file);
		let start = self.start.min(other.start);
		let end = self.end.max(other.end);
		let line = self.line.min(other.line);
		Self::new(start, end, self.file, line)
	}
}

impl AddAssign for Span {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
	pub file: u32,
	pub line: u32,
	// pub offset_in_line: u32,
}

impl Location {
	pub fn unusable() -> Location {
		Location { file: u32::MAX, line: u32::MAX }
	}
}
