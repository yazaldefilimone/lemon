use crate::{
	ast,
	message::MessageResult,
	range::Range,
};

use super::{
	borrow::ptr::RefId,
	context::{scope::ScopeKind, value::Value},
	diags::SyntaxErr,
	typed_value::TypedValue,
	types::{Type, TypeId},
	Checker,
};

impl<'ckr> Checker<'ckr> {
	pub fn check_expr_against_type(
		&mut self,
		expr: &mut ast::Expr,
		expected_type: TypeId,
	) -> MessageResult<TypedValue> {
		let typed_value = self.check_expr(expr)?.unwrap_or_else(|| {
			// Create a dummy RefId for now - in a real implementation, this would be properly tracked
			TypedValue::new(expected_type, RefId(0))
		});

		if typed_value.type_id == expected_type {
			return Ok(typed_value);
		}

		if self.can_coerce(typed_value.type_id, expected_type)? {
			return Ok(TypedValue::new(expected_type, RefId(0)));
		}

		let expected = self.display_type(expected_type);
		let found = self.display_type(typed_value.type_id);
		Err(SyntaxErr::type_mismatch(expected, found, expr.get_range()))
	}

	pub fn synthesize_expr_type(&mut self, expr: &mut ast::Expr) -> MessageResult<TypedValue> {
		let result = self.check_expr(expr)?;
		result.ok_or_else(|| SyntaxErr::cannot_infer_type(expr.get_range()))
	}

	pub fn can_coerce(&self, from: TypeId, to: TypeId) -> MessageResult<bool> {
		if from == to {
			return Ok(true);
		}

		let from_type = self.lookup_stored_type(from);
		let to_type = self.lookup_stored_type(to);

		match (from_type, to_type) {
			(Type::NumRange(from_range), Type::Number(to_num)) => {
				// Check if the range can be resolved to the target number type
				Ok(from_range.try_resolve_with_number(to_num).is_some())
			}
			(Type::Number(from_num), Type::Number(to_num)) => {
				Ok(self.can_coerce_numbers(from_num, to_num))
			}
			(Type::Borrow(from_borrow), Type::Borrow(to_borrow)) => {
				if !from_borrow.mutable && to_borrow.mutable {
					return Ok(false);
				}
				self.can_coerce(from_borrow.value, to_borrow.value)
			}
			_ => Ok(false),
		}
	}

	fn can_coerce_numbers(&self, from: &super::types::Number, to: &super::types::Number) -> bool {
		use super::types::Number::*;
		
		match (from, to) {
			(U8, U16) | (U8, U32) | (U8, U64) => true,
			(U16, U32) | (U16, U64) => true,
			(U32, U64) => true,
			(I8, I16) | (I8, I32) | (I8, I64) => true,
			(I16, I32) | (I16, I64) => true,
			(I32, I64) => true,
			(F32, F64) => true,
			_ => false,
		}
	}

	pub fn check_function_body(
		&mut self,
		body: &mut ast::BlockStmt,
		expected_return: TypeId,
	) -> MessageResult<()> {
		self.ctx.enter_scope(ScopeKind::function(expected_return));

		for stmt in body.stmts.iter_mut() {
			self.check_stmt(stmt)?;
		}

		// Check if the function has a return statement when needed
		let needs_return = expected_return != TypeId::VOID;
		let has_explicit_return = body.stmts.iter().any(|stmt| matches!(stmt, ast::Stmt::Ret(_)));

		self.ctx.exit_scope();

		if needs_return && !has_explicit_return {
			// Check if the last statement is an expression that can serve as implicit return
			if let Some(ast::Stmt::Expr(expr)) = body.stmts.last_mut() {
				let expr_type = self.check_expr(expr)?;
				if let Some(typed_value) = expr_type {
					if typed_value.type_id != expected_return {
						return Err(SyntaxErr::type_mismatch(
							self.display_type(expected_return),
							self.display_type(typed_value.type_id),
							expr.get_range(),
						));
					}
				}
			} else {
				return Err(SyntaxErr::missing_return(
					self.display_type(expected_return),
					body.get_range(),
				));
			}
		}

		Ok(())
	}

	pub fn check_ident_pattern_against_type(
		&mut self,
		ident: &str,
		expected_type: TypeId,
		range: Range,
	) -> MessageResult<()> {
		let typed_value = TypedValue::new(expected_type, RefId(0));
		let value = Value::new(typed_value, false);
		self.ctx.get_scope_mut().add_variable(ident.to_string(), value);
		Ok(())
	}
}