use super::synthesis;
use super::types::TypeId;
use super::{diags::SyntaxErr, Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_const_del_stmt(&mut self, const_del: &mut ast::ConstDelStmt) -> TyResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(const_del.range.clone()));
		}
		let found_id = self.check_expr(&mut const_del.expr)?;
		let lexeme = const_del.name.ident.text.clone();
		let expect_id = match const_del.name.ty.as_ref() {
			Some(ast_type) => synthesis::synthesise_ast_type(ast_type, false, self.ctx)?,
			None => {
				let found_id = self.infer_no_type_anotation(found_id)?;
				self.ctx.add_value(lexeme.as_str(), found_id, false);
				const_del.set_type_id(found_id);
				return Ok(TypeId::UNIT);
			}
		};
		const_del.set_type_id(expect_id);
		self.ctx.add_value(lexeme.as_str(), expect_id, false);
		Ok(TypeId::UNIT)
	}

	pub fn check_const_bind(&mut self, const_bind: &mut ast::Binding) -> TyResult<TypeId> {
		todo!()
	}
}
