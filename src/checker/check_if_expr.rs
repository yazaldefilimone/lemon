use super::types::TypeId;
use super::{Checker, TypedValue};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> MessageResult<TypedValue> {
		let cond_type = self.check_expr(&mut if_expr.cond)?;
		self.equal_type_expected(TypeId::BOOL, cond_type.type_id, if_expr.cond.get_range())?;
		let then = self.check_expr(&mut if_expr.then)?;
		let otherwise = self.check_expr(&mut if_expr.otherwise)?;
		let if_expr_typed = match self.unify_types(then.type_id, otherwise.type_id)? {
			Some(unified_type) => unified_type,
			None => {
				self.equal_type_expected(then.type_id, otherwise.type_id, if_expr.otherwise.get_range())?
			}
		};
		self.register_type(if_expr_typed, if_expr.get_range());
		Ok(self.owned_typed_value(if_expr_typed))
	}
}
