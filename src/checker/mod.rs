#![allow(dead_code, unused_variables)]
use crate::{
	ast,
	diag::{Diag, DiagGroup},
};

pub mod context;
pub mod types;
use context::Context;
use types::{Type, TypeId};
mod synthesis;

mod check_assign_expr;
mod check_associate_expr;
mod check_binary_expr;
mod check_block_stmt;
mod check_borrow_expr;
mod check_call_expr;
mod check_const_del_stmt;
mod check_const_fn_stmt;
mod check_deref_expr;
mod check_expr;
mod check_extern_fn_stmt;
mod check_fn_stmt;
mod check_for_stmt;
mod check_ident_expr;
mod check_if_stmt;
mod check_impl_stmt;
mod check_import_expr;
mod check_let_stmt;
mod check_literal;
mod check_member_expr;
mod check_ret_stmt;
mod check_struct_init_expr;
mod check_type_def_stmt;
mod check_while_stmt;
mod diags;
mod equal_type;
mod infer;
mod infer_generic;

type TyResult<T> = Result<T, Diag>;

pub struct Checker<'ckr> {
	ctx: &'ckr mut Context,
	diag_group: &'ckr mut DiagGroup,
}

impl<'ckr> Checker<'ckr> {
	pub fn new(diag_group: &'ckr mut DiagGroup, ctx: &'ckr mut Context) -> Self {
		Self { ctx, diag_group }
	}

	pub fn check_program(&mut self, ast: &mut ast::Program) -> TyResult<TypeId> {
		for stmt in ast.stmts.iter_mut() {
			self.check_stmt(stmt)?;
		}
		Ok(TypeId::UNIT)
	}

	pub(crate) fn check_stmt(&mut self, stmt: &mut ast::Stmt) -> TyResult<TypeId> {
		match stmt {
			ast::Stmt::Expr(expr) => self.check_expr(expr),
			ast::Stmt::Let(let_stmt) => self.check_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.check_fn_stmt(fn_stmt),
			ast::Stmt::While(while_stmt) => self.check_while_stmt(while_stmt),
			ast::Stmt::For(for_stmt) => self.check_for_stmt(for_stmt),
			ast::Stmt::Block(block_stmt) => self.check_block_stmt(block_stmt),
			ast::Stmt::ConstDel(const_del) => self.check_const_del_stmt(const_del),
			ast::Stmt::ConstFn(const_fn) => self.check_const_fn_stmt(const_fn),
			ast::Stmt::Ret(ret_stmt) => self.check_ret_stmt(ret_stmt),
			ast::Stmt::If(if_stmt) => self.check_if_stmt(if_stmt),
			ast::Stmt::ExternFn(extern_fn_stmt) => self.check_extern_fn_stmt(extern_fn_stmt),
			ast::Stmt::TypeDef(type_def_stmt) => self.check_type_def_stmt(type_def_stmt),
			ast::Stmt::Impl(impl_stmt) => self.check_impl_stmt(impl_stmt),
		}
	}

	fn get_stored_type(&self, type_id: TypeId) -> &Type {
		match self.ctx.type_store.get_type(type_id) {
			Some(type_value) => type_value,
			None => panic!("error: type not found"), // TODO: error handling
		}
	}

	pub fn get_stored_type_without_borrow(&self, type_id: TypeId) -> &Type {
		match self.ctx.type_store.get_type(type_id) {
			Some(type_value) => {
				if let Type::Borrow(borrow) = type_value {
					return self.get_stored_type_without_borrow(borrow.value);
				}
				type_value
			}
			None => panic!("error: type not found"), // TODO: error handling
		}
	}

	pub fn get_stored_mut_type(&mut self, type_id: TypeId) -> &mut Type {
		match self.ctx.type_store.get_mut_type(type_id) {
			Some(type_value) => type_value,
			None => panic!("error: type not found"), // TODO: error handling
		}
	}

	pub fn display_type(&self, type_id: TypeId) -> String {
		let mut text = String::new();
		type_id.display_type(&mut text, &self.ctx.type_store, false);
		text
	}
	pub fn display_type_value(&self, type_value: &Type) -> String {
		if let Type::Struct(struct_type) = type_value {
			return format!("struct {}", struct_type.name);
		}
		let mut text = String::new();
		type_value.display_type(&mut text, &self.ctx.type_store, false);
		text
	}

	pub fn display_double_type(&self, left: TypeId, right: TypeId) -> (String, String) {
		let mut left_text = String::new();
		let mut right_text = String::new();
		left.display_type(&mut left_text, &self.ctx.type_store, false);
		right.display_type(&mut right_text, &self.ctx.type_store, false);
		(left_text, right_text)
	}
}
