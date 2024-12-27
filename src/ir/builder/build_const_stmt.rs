use crate::{ast, ir::ir};

use super::Builder;

impl Builder {
	pub fn build_const_stmt(&mut self, const_stmt: &ast::ConstStmt) {
		self.ctx.enter_comptime();
		let name = const_stmt.name.lexeme();
		let value = self.build_expr(&const_stmt.expr);
		let type_id = const_stmt.type_id.unwrap();
		self.ctx.add_value(name, value.get_register().unwrap());
		let dest = self.ctx.get_register();
		let instr = ir::OwnInstr { type_id, value, dest };
		self.add_global(ir::Instr::Own(instr));
		self.ctx.exit_comptime();
	}
}
