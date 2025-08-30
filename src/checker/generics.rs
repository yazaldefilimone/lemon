use rustc_hash::FxHashMap;
use crate::{
	message::MessageResult,
	range::Range,
};

use super::{
	diags::SyntaxErr,
	types::{Type, TypeId},
	Checker,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
	pub name: String,
	pub constraint: Option<TypeId>,
	pub range: Range,
}

#[derive(Debug, Clone)]
pub struct GenericContext {
	type_params: Vec<TypeParameter>,
	instantiations: FxHashMap<String, TypeId>,
}

impl GenericContext {
	pub fn new() -> Self {
		Self {
			type_params: Vec::new(),
			instantiations: FxHashMap::default(),
		}
	}

	pub fn add_type_param(&mut self, param: TypeParameter) {
		self.type_params.push(param);
	}

	pub fn instantiate(&mut self, param_name: &str, concrete_type: TypeId) {
		self.instantiations.insert(param_name.to_string(), concrete_type);
	}

	pub fn lookup_instantiation(&self, param_name: &str) -> Option<TypeId> {
		self.instantiations.get(param_name).copied()
	}

	pub fn is_type_param(&self, name: &str) -> bool {
		self.type_params.iter().any(|p| p.name == name)
	}
}

impl<'ckr> Checker<'ckr> {
	pub fn instantiate_generic_type(
		&mut self,
		generic_type: TypeId,
		type_args: &[TypeId],
		range: Range,
	) -> MessageResult<TypeId> {
		let base_type = self.lookup_stored_type(generic_type).clone();
		
		match base_type {
			Type::Fn(fn_type) => {
				// For generic functions, we need to substitute type parameters
				// This is a simplified version - full implementation would track generic params
				Ok(generic_type)
			}
			Type::Struct(struct_type) => {
				// For generic structs, create a new instantiated version
				Ok(generic_type)
			}
			_ => {
				Err(SyntaxErr::not_generic_type(
					self.display_type(generic_type),
					range,
				))
			}
		}
	}

	pub fn infer_generic_args(
		&mut self,
		expected_type: TypeId,
		actual_args: &[TypeId],
	) -> MessageResult<Vec<TypeId>> {
		// This is a simplified type inference algorithm
		// A full implementation would use unification
		let mut inferred = Vec::new();
		
		for arg in actual_args {
			inferred.push(*arg);
		}
		
		Ok(inferred)
	}

	pub fn unify_generic_types(
		&mut self,
		left: TypeId,
		right: TypeId,
		substitutions: &mut FxHashMap<String, TypeId>,
	) -> MessageResult<bool> {
		if left == right {
			return Ok(true);
		}

		let left_type = self.lookup_stored_type(left).clone();
		let right_type = self.lookup_stored_type(right).clone();

		match (&left_type, &right_type) {
			(Type::Infer(left_infer), _) => {
				// If left is a type variable, try to bind it
				if let Some(existing) = substitutions.get(&left_infer.id) {
					return self.unify_generic_types(*existing, right, substitutions);
				}
				substitutions.insert(left_infer.id.clone(), right);
				Ok(true)
			}
			(_, Type::Infer(right_infer)) => {
				// If right is a type variable, try to bind it
				if let Some(existing) = substitutions.get(&right_infer.id) {
					return self.unify_generic_types(left, *existing, substitutions);
				}
				substitutions.insert(right_infer.id.clone(), left);
				Ok(true)
			}
			(Type::Fn(left_fn), Type::Fn(right_fn)) => {
				// Unify function types
				if left_fn.args.len() != right_fn.args.len() {
					return Ok(false);
				}
				
				for (l_arg, r_arg) in left_fn.args.iter().zip(right_fn.args.iter()) {
					if !self.unify_generic_types(*l_arg, *r_arg, substitutions)? {
						return Ok(false);
					}
				}
				
				self.unify_generic_types(left_fn.ret, right_fn.ret, substitutions)
			}
			_ => Ok(false),
		}
	}

	pub fn apply_substitutions(
		&mut self,
		type_id: TypeId,
		substitutions: &FxHashMap<String, TypeId>,
	) -> TypeId {
		let type_value = self.lookup_stored_type(type_id).clone();
		
		match type_value {
			Type::Infer(infer) => {
				substitutions.get(&infer.id).copied().unwrap_or(type_id)
			}
			Type::Fn(fn_type) => {
				let mut args = Vec::new();
				for arg in fn_type.args.iter() {
					args.push(self.apply_substitutions(*arg, substitutions));
				}
				let ret = self.apply_substitutions(fn_type.ret, substitutions);
				
				if args == fn_type.args && ret == fn_type.ret {
					type_id
				} else {
					let mut new_fn_type = fn_type.clone();
					new_fn_type.args = args;
					new_fn_type.ret = ret;
					let new_fn = Type::Fn(new_fn_type);
					self.ctx.type_store.add_type(new_fn)
				}
			}
			_ => type_id,
		}
	}
}