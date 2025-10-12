use crate::types::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadableKind {
	Variable { mutable: bool },
}

#[derive(Debug, Clone, Copy)]
pub struct Readable<'a> {
	pub name: &'a str,
	pub type_id: TypeId,
	pub kind: ReadableKind,
	pub is_pointer_access_mutable: bool,
}

#[derive(Debug)]
pub struct Readables<'a> {
	pub starting_index: usize,
	pub readables: Vec<Readable<'a>>,
}

impl<'a> Readables<'a> {
	pub fn new() -> Self {
		Self { starting_index: 0, readables: Vec::new() }
	}

	pub fn overall_len(&self) -> usize {
		self.readables.len()
	}

	pub fn push(
		&mut self,
		name: &'a str,
		type_id: TypeId,
		kind: ReadableKind,
		is_pointer_access_mutable: bool,
	) -> usize {
		let index = self.readables.len() - self.starting_index;
		self.readables.push(Readable { name, type_id, kind, is_pointer_access_mutable });
		index
	}

	pub fn get(&self, index: usize) -> Option<Readable<'a>> {
		self.readables.get(index + self.starting_index).copied()
	}
}
