#![allow(dead_code)]
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(u64);

impl ModuleId {
	pub fn new(id: u64) -> Self {
		Self(id)
	}

	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

impl From<u64> for ModuleId {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl Display for ModuleId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "mod{}", self.0)
	}
}
