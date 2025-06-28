use std::{
	fmt::Display,
	ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Span {
	pub start: usize,
	pub end: usize,
	pub file: u32,
	pub line: u32,
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}
impl Add for Span {
	type Output = Span;
	fn add(self, other: Self) -> Span {
		assert_eq!(self.file, other.file);
		let start = self.start.min(other.start);
		let end = self.end.max(other.end);
		let line = self.line.min(other.line);
		Span::new(start, end, self.file, line)
	}
}

impl AddAssign for Span {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Span {
	pub fn new(start: usize, end: usize, file: u32, line: u32) -> Self {
		Span { start, end, file, line }
	}
	pub fn unusable() -> Span {
		Span { start: usize::MAX, end: usize::MAX, file: u32::MAX, line: u32::MAX }
	}
}
