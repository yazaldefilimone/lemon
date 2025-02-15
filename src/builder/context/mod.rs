use scope::Scope;

use crate::{
	checker::types::TypeId,
	ir::{BasicValue, IrBasicValue},
	report::throw_ir_build_error,
};
mod block;
mod label;
mod scope;

pub struct Context {
	pub scopes: Vec<Scope>,
	pub block: block::Block,
	pub register_count: usize,
}
impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

impl Context {
	pub fn new() -> Context {
		Context { scopes: Vec::new(), block: block::Block::new(), register_count: 1 }
	}

	pub fn push_scope(&mut self, ret_type: Option<TypeId>) {
		self.scopes.push(Scope::new(ret_type));
	}

	pub fn get_ret_type(&self) -> Option<&TypeId> {
		self.get_current_scope().get_ret_type()
	}

	pub fn new_register(&mut self, type_id: TypeId) -> IrBasicValue {
		let register = format!("r{}", self.register_count);
		self.register_count += 1;
		let register_value = BasicValue::Register(register);

		IrBasicValue::new(register_value, type_id)
	}

	pub fn pop_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn get_current_scope(&self) -> &Scope {
		match self.scopes.last() {
			Some(scope) => scope,
			None => throw_ir_build_error("scope not found"),
		}
	}

	pub fn get_current_scope_mut(&mut self) -> &mut Scope {
		match self.scopes.last_mut() {
			Some(scope) => scope,
			None => throw_ir_build_error("scope not found"),
		}
	}

	pub fn add_local(&mut self, key: String, basic_value: IrBasicValue) {
		self.get_current_scope_mut().add_local(key, basic_value);
	}

	pub fn get_local(&self, name: &str) -> Option<&IrBasicValue> {
		self.get_current_scope().get_local(name)
	}

	pub fn add_dont_load(&mut self, key: impl Into<String>) {
		self.get_current_scope_mut().add_dont_load(key);
	}

	pub fn is_dont_load(&self, key: &str) -> bool {
		self.get_current_scope().is_dont_load(key)
	}
}
