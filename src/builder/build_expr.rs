use crate::{ast, ir::IrBasicValue};

use super::Builder;

impl Builder<'_> {
	pub fn build_expr(&mut self, expr: &mut ast::Expr) -> IrBasicValue {
		match expr {
			ast::Expr::Assign(assign_expr) => self.build_assign_expr(assign_expr),
			ast::Expr::Binary(binary_expr) => self.build_binary_expr(binary_expr),
			ast::Expr::Borrow(borrow_expr) => self.build_borrow_expr(borrow_expr),
			ast::Expr::Call(call_expr) => self.build_call_expr(call_expr),
			ast::Expr::If(if_expr) => self.build_if_expr(if_expr),
			ast::Expr::Deref(deref_expr) => self.build_deref_expr(deref_expr),
			ast::Expr::Ident(ident_expr) => self.build_ident_expr(ident_expr),
			ast::Expr::Literal(literal) => self.build_literal(literal),
			ast::Expr::StructInit(struct_init_expr) => self.build_struct_init_expr(struct_init_expr),
			ast::Expr::Member(member_expr) => self.build_member_expr(member_expr),
			ast::Expr::Associate(associate_expr) => self.build_associate_expr(associate_expr),
			// ast::Expr::Ret(ret_expr) => self.build_ret_expr(ret_expr),
			// ast::Expr::StructInit(struct_init_expr) => self.build_struct_init_expr(struct_init_expr),
			// ast::Expr::TypeDef(type_def) => self.build_type_def_expr(type_def),
			// ast::Expr::While(while_expr) => self.build_while_expr(while_expr),
			_ => todo!("{:?}", expr),
		}
	}
}
