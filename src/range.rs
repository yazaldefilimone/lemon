#![allow(dead_code)]
use core::fmt;
use logos::Span;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Range {
	// set as u32?
	pub start: usize,
	pub end: usize,
}

impl Range {
	pub fn new(start: usize, end: usize) -> Range {
		Range { start, end }
	}

	pub fn merge(&mut self, range: &Range) {
		self.start = min(self.start, range.start);
		self.end = max(self.end, range.end);
	}

	pub fn merged_with(&self, range: &Range) -> Range {
		Range::new(min(self.start, range.start), max(self.end, range.end))
	}

	pub fn from_span(span: Span) -> Range {
		Range::new(span.start, span.end)
	}
	pub fn is_empty(&self) -> bool {
		self.start == self.end
	}

	pub fn is_valid(&self) -> bool {
		if self.start > self.end {
			return false;
		}
		if self.start == self.end && self.start == 0 {
			return false;
		}
		true
	}
}

impl fmt::Display for Range {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}

impl Default for Range {
	fn default() -> Self {
		Self::new(0, 0)
	}
}
