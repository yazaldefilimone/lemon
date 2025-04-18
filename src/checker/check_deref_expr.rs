use crate::ast::{self};

use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::{Checker, TyResult};
impl Checker<'_> {
	pub fn check_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> TyResult<TypeId> {
		let ref_id = self.check_expr(&mut deref_expr.expr)?;
		if ref_id.is_known() {
			let ref_type = self.display_type(ref_id);
			return Err(SyntaxErr::cannot_dereference(ref_type, deref_expr.get_range()));
		}
		let ret_type = self.get_stored_type(ref_id);
		if let Type::Borrow(borrow) = ret_type {
			deref_expr.set_type_id(borrow.value);
			return Ok(borrow.value);
		}
		let ref_type = self.display_type(ref_id);
		Err(SyntaxErr::cannot_dereference(ref_type, deref_expr.get_range()))
	}
}
