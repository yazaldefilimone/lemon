pub mod context;
use std::mem;

use crate::ir::IR;
use crate::source::Source;
use crate::{ast, ir};
use context::Context;

mod build_assign_expr;
mod build_binary_expr;
mod build_borrow_expr;
mod build_call_expr;

mod build_deref_expr;
mod build_expr;

mod build_fn_stmt;
mod build_ident_expr;
mod build_if_expr;
mod build_let_stmt;
mod build_literal;
mod build_ret_stmt;
mod build_type;

// mod build_block_stmt;
// mod build_const_del_stmt;
// mod build_const_fn_stmt;
// mod build_extern_fn;
// mod build_for_stmt;
// mod build_impl_stmt;
// mod build_member_expr;
// mod build_struct_init_expr;
// mod build_type_def_stmt;
// mod build_while_stmt;

pub struct Builder<'br> {
	ctx: Context,
	ir: IR,
	source: &'br Source,
}

impl<'br> Builder<'br> {
	pub fn new(source: &'br Source) -> Self {
		Self { ctx: Context::new(), ir: IR::new(), source }
	}

	pub fn build(&mut self, program: &mut ast::Program) -> IR {
		for stmt in program.stmts.iter_mut() {
			self.build_stmt(stmt);
		}
		mem::take(&mut self.ir)
	}

	pub fn push_function_with_blocks(&mut self, mut func: ir::Function) {
		let blocks = self.ctx.block.take_blocks();
		func.extend_blocks(blocks);
		self.ir.add_func(func);
	}

	fn build_stmt(&mut self, stmt: &mut ast::Stmt) {
		match stmt {
			ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.build_fn_stmt(fn_stmt),
			// ast::Stmt::Block(block_stmt) => self.build_block_stmt(block_stmt),
			// ast::Stmt::While(while_stmt) => self.build_while_stmt(while_stmt),
			// ast::Stmt::For(for_stmt) => self.build_for_stmt(for_stmt),
			// ast::Stmt::ConstDel(const_del) => self.build_const_del_stmt(const_del),
			// ast::Stmt::ConstFn(const_fn) => self.build_const_fn_stmt(const_fn),
			// ast::Stmt::Ret(ret_stmt) => self.build_ret_stmt(ret_stmt),
			// ast::Stmt::ExternFn(extern_fn) => self.build_extern_fn(extern_fn),
			// ast::Stmt::TypeDef(type_def) => self.build_type_def_stmt(type_def),
			// ast::Stmt::Impl(impl_stmt) => self.build_impl_stmt(impl_stmt),
			_ => todo!(),
		}
	}
}
