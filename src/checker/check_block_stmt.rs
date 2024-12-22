use crate::ast::{self};

use super::{context::scope::ScopeType, types::TypeId, Checker, TypeResult};

impl Checker<'_> {
	pub fn check_block_stmt(&mut self, block: &ast::BlockStmt) -> TypeResult<TypeId> {
		// todo: warn unreachable code
		self.ctx.enter_scope(ScopeType::new_block());
		for stmt in block.stmts.iter() {
			let ret_type = self.check_stmt(stmt)?;
			if ret_type != TypeId::NOTHING {
				return Ok(ret_type);
			}
		}
		self.ctx.exit_scope();
		Ok(TypeId::NOTHING)
	}
}
