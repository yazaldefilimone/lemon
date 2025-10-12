use std::fmt;

use crate::{hir, store::type_store::TypeStore};

#[derive(Debug, Clone, Copy, Hash)]
pub struct TypeId {
	pub entry: u32,
}

impl TypeId {
	pub fn index(self) -> usize {
		self.entry as usize
	}
	pub fn unusable() -> TypeId {
		TypeId { entry: u32::MAX }
	}

	fn matches(self, _store: &TypeStore, _other: TypeId) -> bool {
		// store.direct_match(self, other)
		todo!()
	}

	pub fn is_any_collapse(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.any_collapse)
	}
	pub fn is_noreturn(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.no_return)
	}
	pub fn is_void(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.void)
	}
	pub fn is_untyped_number(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.number)
	}
	pub fn is_bool(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.bool)
	}
	pub fn is_string(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.string)
	}
	pub fn is_string_mutable(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.string_mutable)
	}
	pub fn is_format_string(self, store: &TypeStore) -> bool {
		self.matches(store, store.builtin.format_string)
	}

	pub fn is_numeric(self, store: &TypeStore) -> bool {
		let range = store.builtin.number.entry..=store.builtin.f64.entry;
		range.contains(&self.entry) || self.is_any_collapse(store)
	}

	pub fn is_integer(self, store: &TypeStore, expr: &hir::Expression) -> bool {
		let range = store.builtin.i8.entry..=store.builtin.usize.entry;
		if range.contains(&self.entry) || self.is_any_collapse(store) {
			return true;
		}
		matches!(&expr.kind, hir::ExpressionKind::NumberValue(v) if v.is_integer())
	}

	pub fn is_formattable(self, store: &TypeStore, expr: &hir::Expression) -> bool {
		let range = store.builtin.i8.entry..=store.builtin.format_string.entry;
		if range.contains(&self.entry) || self.is_any_collapse(store) {
			return true;
		}
		match &expr.kind {
			hir::ExpressionKind::NumberValue(v) => {
				v.collapsed().map_or(false, |c| range.contains(&c.entry))
			}
			_ => false,
		}
	}
}

impl fmt::Display for TypeId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "type_id: {}", self.index())
	}
}
