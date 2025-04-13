use crate::ast;
use crate::message::MessageResult;

use super::{Checker, TypedValue};
impl Checker<'_> {
	pub fn check_expr(&mut self, expr: &mut ast::Expr) -> MessageResult<TypedValue> {
		match expr {
			ast::Expr::Binary(binary_expr) => self.check_binary_expr(binary_expr),
			ast::Expr::Literal(literal) => self.check_literal(literal),
			ast::Expr::Deref(deref_expr) => self.check_deref_expr(deref_expr),
			ast::Expr::Borrow(borrow_expr) => self.check_borrow_expr(borrow_expr),
			ast::Expr::Assign(assign_expr) => self.check_assign_expr(assign_expr),
			ast::Expr::Ident(ident_expr) => self.check_ident_expr(ident_expr),
			ast::Expr::Call(call_expr) => self.check_call_expr(call_expr),
			ast::Expr::If(if_expr) => self.check_if_expr(if_expr),
			ast::Expr::StructInit(stuct_init_expr) => self.check_struct_init_expr(stuct_init_expr),
			ast::Expr::Import(import_expr) => self.check_import_expr(import_expr),
			ast::Expr::Associate(associate_expr) => self.check_associate_expr(associate_expr),
			ast::Expr::Member(member_expr) => self.check_member_expr(member_expr),
			_ => todo!("code {:?}", expr),
		}
	}
}
