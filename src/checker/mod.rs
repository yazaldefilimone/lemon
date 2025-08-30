#![allow(dead_code, unused_variables)]
use crate::{
	ast,
	loader::{Loader, ModId},
	message::MessageResult,
	range::Range,
};

mod bidirectional;
mod borrow;
mod comptime;
pub mod context;
mod error_suggestions;
pub mod events;
mod generics;
pub mod types;
use context::Context;
use diags::SyntaxErr;
use typed_value::TypedValue;
use types::{BorrowType, Type, TypeId};
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
mod event;
mod infer;
mod synthesis;
mod typed_value;

type CheckResult = MessageResult<Option<TypedValue>>;
pub trait ExpectSome<T> {
	fn expect_some(self, range: Range) -> MessageResult<T>;
	fn some(self, range: Range) -> MessageResult<T>;
}

impl ExpectSome<TypedValue> for MessageResult<Option<TypedValue>> {
	fn expect_some(self, range: Range) -> MessageResult<TypedValue> {
		match self? {
			Some(value) => Ok(value),
			None => Err(SyntaxErr::cannot_infer_type(range)),
		}
	}
	fn some(self, range: Range) -> MessageResult<TypedValue> {
		self.expect_some(range)
	}
}

#[macro_export]
macro_rules! expect_some {
	($opt_result:expr, $range:expr) => {
		match $opt_result {
			Ok(Some(value)) => Ok(value),
			Ok(None) => Err($crate::checker::diags::SyntaxErr::cannot_infer_type($range)),
			Err(e) => Err(e),
		}
	};
}

pub struct Checker<'ckr> {
	ctx: &'ckr mut Context,
	loader: &'ckr mut Loader,
}

impl<'ckr> Checker<'ckr> {
	pub fn new(ctx: &'ckr mut Context, loader: &'ckr mut Loader) -> Self {
		Self { ctx, loader }
	}

	pub fn check(&mut self, mod_id: ModId) {
		if let Err(message) = self.check_program(mod_id) {
			let mod_id = self.ctx.mod_id;
			message.mod_id(mod_id).report(self.loader);
		}
	}

	pub fn check_program(&mut self, mod_id: ModId) -> CheckResult {
		self.ctx.add_entry_mod(mod_id);
		let mut ast = self.loader.lookup_mod_result(mod_id).cloned().unwrap_or_else(|message| {
			message.report(self.loader);
		});
		for stmt in ast.stmts.iter_mut() {
			self.check_stmt(stmt)?;
		}
		Ok(None)
	}

	pub(crate) fn check_stmt(&mut self, stmt: &mut ast::Stmt) -> CheckResult {
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

	fn lookup_stored_type(&self, type_id: TypeId) -> &Type {
		match self.ctx.type_store.lookup_type(type_id) {
			Some(type_value) => type_value,
			None => panic!("error: type not found"), // TODO: error handling
		}
	}

	pub fn lookup_stored_borrow(&self, type_id: TypeId) -> Option<&BorrowType> {
		match self.ctx.type_store.lookup_type(type_id) {
			Some(Type::Borrow(ref borrow)) => Some(borrow),
			_ => None,
		}
	}

	pub fn lookup_stored_type_without_borrow(&self, type_id: TypeId) -> &Type {
		match self.ctx.type_store.lookup_type(type_id) {
			Some(type_value) => {
				if let Type::Borrow(borrow) = type_value {
					return self.lookup_stored_type_without_borrow(borrow.value);
				}
				type_value
			}
			None => panic!("error: type not found"), // TODO: error handling
		}
	}

	pub fn lookup_stored_mut_type(&mut self, type_id: TypeId) -> &mut Type {
		match self.ctx.type_store.lookup_mut_type(type_id) {
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

	pub fn _display_type_value(&self, type_value: Type) -> String {
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
	pub fn fn_signature(
		&mut self,
		f: TypeId,
		range: Range,
	) -> MessageResult<(Vec<TypeId>, TypeId, bool)> {
		match self.lookup_stored_type(f).clone() {
			Type::Fn(info) => Ok((info.args, info.ret, false)),
			Type::ExternFn(info) => Ok((info.args, info.ret, info.var_packed)),
			_ => Err(SyntaxErr::not_a_fn(self.display_type(f), range)),
		}
	}

	pub fn call_args_match(
		&mut self,
		params: &[TypeId],
		args: &[TypedValue],
		range: Range,
		var_packed: bool,
	) -> MessageResult<()> {
		if !var_packed && params.len() != args.len() {
			return Err(SyntaxErr::wrong_arg_count(params.len(), args.len(), range));
		}
		for (i, (p, a)) in params.iter().zip(args).enumerate() {
			if *p != a.type_id {
				let exp = self.display_type(*p);
				let got = self.display_type(a.type_id);
				// return Err(SyntaxErr::arg_type_mismatch(i, exp, got));
				return Err(SyntaxErr::type_mismatch(exp, got, range));
			}
		}
		Ok(())
	}

	pub fn unwrap_typed_value(&mut self, value: Option<TypedValue>, range: Range) -> TypedValue {
		if let Some(value) = value {
			return value;
		}
		let message = SyntaxErr::cannot_infer_type(range);
		message.report(self.loader);
	}
}
