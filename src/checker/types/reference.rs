use super::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct Reference {
	pub target_type: TypeId,
	pub mutable: bool,

	pub borrow_tag: Option<u32>,

	pub has_ownership: bool,
}

impl Reference {
	pub fn immutable(target_type: TypeId) -> Self {
		Reference { target_type, mutable: false, borrow_tag: None, has_ownership: false }
	}

	pub fn mutable(target_type: TypeId) -> Self {
		Reference { target_type, mutable: true, borrow_tag: None, has_ownership: true }
	}

	pub fn transfer_ownership(&mut self, to: &mut Reference) {
		assert!(self.has_ownership, "cannot transfer ownership from non-owning reference");
		assert!(to.mutable, "cannot transfer ownership to immutable reference");
		self.has_ownership = false;
		to.has_ownership = true;
		to.borrow_tag = self.borrow_tag.take();
	}
}

#[derive(Debug, Clone)]
pub struct Prophecy {
	pub reference_id: usize,

	pub final_type: TypeId,

	pub resolved: bool,

	pub resolution_scope: usize,
}

impl Prophecy {
	pub fn new(reference_id: usize, final_type: TypeId, scope: usize) -> Self {
		Prophecy { reference_id, final_type, resolved: false, resolution_scope: scope }
	}

	pub fn resolve(&mut self) {
		assert!(!self.resolved, "prophecy already resolved");
		self.resolved = true;
		// here, the cache would be updated without memory read
	}
}

#[derive(Debug, Clone)]
pub struct BorrowStack {
	tags: Vec<u32>,

	next_tag: u32,
}

impl BorrowStack {
	pub fn new() -> Self {
		BorrowStack { tags: Vec::new(), next_tag: 1 }
	}
	pub fn push(&mut self) -> u32 {
		let tag = self.next_tag;
		self.next_tag += 1;
		self.tags.push(tag);
		tag
	}

	pub fn pop(&mut self) -> Option<u32> {
		self.tags.pop()
	}

	pub fn has_access(&self, tag: u32) -> bool {
		self.tags.last() == Some(&tag)
	}

	pub fn top(&self) -> Option<u32> {
		self.tags.last().copied()
	}
}

#[derive(Debug, Clone)]
pub struct ReferenceCache {
	cached_values: Vec<CachedValue>,
}

#[derive(Debug, Clone)]
pub struct CachedValue {
	pub reference_id: usize,

	pub type_id: TypeId,

	pub valid: bool,

	pub version: u32,
}

impl ReferenceCache {
	pub fn new() -> Self {
		ReferenceCache { cached_values: Vec::new() }
	}

	pub fn cache(&mut self, reference_id: usize, type_id: TypeId) -> usize {
		let index = self.cached_values.len();
		self.cached_values.push(CachedValue { reference_id, type_id, valid: true, version: 0 });
		index
	}

	pub fn invalidate(&mut self, reference_id: usize) {
		if let Some(cached) = self.cached_values.iter_mut().find(|c| c.reference_id == reference_id) {
			cached.valid = false;
		}
	}

	pub fn update_from_prophecy(&mut self, reference_id: usize, new_type: TypeId) {
		if let Some(cached) = self.cached_values.iter_mut().find(|c| c.reference_id == reference_id) {
			cached.type_id = new_type;
			cached.valid = true;
			cached.version += 1;
		}
	}

	pub fn is_valid(&self, reference_id: usize) -> bool {
		self
			.cached_values
			.iter()
			.find(|c| c.reference_id == reference_id)
			.map(|c| c.valid)
			.unwrap_or(false)
	}
}
