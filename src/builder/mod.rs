pub mod context;
use std::mem;

use crate::checker::events::{Event, EventId};
use crate::checker::types::{TypeId, TypeStore};
use crate::ir::{Instr, IR};
use crate::loader::{Loader, ModId};
use crate::range::Range;
use crate::{ast, error_build, ir};
use context::Context;

mod build_assign_expr;
mod build_binary_expr;
mod build_borrow_expr;
mod build_call_expr;
mod build_if_expr;

mod build_deref_expr;
mod build_expr;

mod build_associate_expr;
mod build_block_stmt;
mod build_extern_fn_stmt;
mod build_fn_stmt;
mod build_ident_expr;
mod build_if_stmt;
mod build_impl_stmt;
mod build_let_stmt;
mod build_literal;
mod build_member_expr;
mod build_ret_stmt;
mod build_struct_def_stmt;
mod build_struct_init_expr;
mod build_type_def_stmt;
mod build_utils;
// mod build_const_del_stmt;
// mod build_const_fn_stmt;
// mod build_extern_fn;
// mod build_for_stmt;
// mod build_impl_stmt;
// mod build_member_expr;
// mod build_struct_init_expr;
// mod build_while_stmt;

pub struct Builder<'br> {
	ctx: Context,
	ir: IR,
	type_store: &'br TypeStore,
	event: &'br mut Event,
	loader: &'br mut Loader,
	mod_id: Option<ModId>,
}

impl<'br> Builder<'br> {
	pub fn new(type_store: &'br TypeStore, event: &'br mut Event, loader: &'br mut Loader) -> Self {
		Self { ctx: Context::new(), event, ir: IR::default(), type_store, loader, mod_id: None }
	}

	pub fn build(&mut self, mod_id: ModId) -> IR {
		self.mod_id = Some(mod_id);
		let mut program = self.loader.take_mod_result(mod_id).unwrap_or_else(|message| {
			message.report(self.loader);
		});

		for stmt in program.stmts.iter_mut() {
			self.build_stmt(stmt);
		}
		self.mod_id = None;
		mem::take(&mut self.ir)
	}

	#[inline(always)]
	pub fn mod_id_unchecked(&self) -> ModId {
		self.mod_id.unwrap_or_else(|| self.internal_error("could not resolve module", self.loader))
	}

	pub fn lookup_event_type(&self, range: Range) -> TypeId {
		let event_id = EventId::new(self.mod_id_unchecked(), range);
		self.event.lookup_type(event_id).unwrap_or_else(|| {
			self.internal_error_with_range("could not resolve event type", range, self.loader)
		})
	}

	pub fn lookup_multi_event_types(&self, range: Range) -> Vec<TypeId> {
		let event_id = EventId::new(self.mod_id_unchecked(), range);
		self.event.lookup_multi_types(event_id).cloned().unwrap_or_default()
	}

	// #[inline(always)]
	// pub fn lookup_event_drops(&self, _range: Range) -> Option<&FxHashSet<String>> {
	// 	todo!()
	// 	// self.event.lookup_drops(event_id)
	// }
	fn internal_error(&self, msg: &str, loader: &Loader) -> ! {
		let m = error_build!("{}", msg);
		m.note_internal().report(loader);
	}
	fn internal_error_with_range(&self, msg: &str, range: Range, loader: &Loader) -> ! {
		let m = error_build!("{}", msg).mod_id(self.mod_id_unchecked());
		m.note_internal().range(range).report(loader);
	}

	pub fn push_function_with_blocks(&mut self, mut function: ir::Function) {
		let blocks = self.ctx.current_block.extract_blocks();
		function.extend_blocks(blocks);
		self.ir.add_function(function);
	}

	pub fn drop_local_function_values(&mut self, ret_value: Option<&str>) {
		for value in self.ctx.collect_unbound_values() {
			if ret_value.map(|ret_value| value.value.as_str() != ret_value).unwrap_or(true) {
				let result = self.ctx.current_block.append_instr(Instr::Drop(value));
				result.unwrap_or_else(|message| {
					message.mod_id(self.mod_id_unchecked()).report(self.loader);
				});
			}
		}
	}

	fn build_stmt(&mut self, stmt: &mut ast::Stmt) {
		match stmt {
			ast::Stmt::Let(let_stmt) => self.build_let_stmt(let_stmt),
			ast::Stmt::Fn(fn_stmt) => self.build_fn_stmt(fn_stmt),
			ast::Stmt::ExternFn(extern_fn_stmt) => self.build_extern_fn_stmt(extern_fn_stmt),
			ast::Stmt::Block(block_stmt) => self.build_block_stmt(block_stmt),
			ast::Stmt::Ret(ret_stmt) => self.build_ret_stmt(ret_stmt),
			ast::Stmt::If(if_stmt) => self.build_if_stmt(if_stmt),
			ast::Stmt::TypeDef(type_def) => self.build_type_def_stmt(type_def),
			// ast::Stmt::While(while_stmt) => self.build_while_stmt(while_stmt),
			// ast::Stmt::For(for_stmt) => self.build_for_stmt(for_stmt),
			// ast::Stmt::ConstDel(const_del) => self.build_const_del_stmt(const_del),
			// ast::Stmt::ConstFn(const_fn) => self.build_const_fn_stmt(const_fn),
			// ast::Stmt::ExternFn(extern_fn) => self.build_extern_fn(extern_fn),
			ast::Stmt::Impl(impl_stmt) => self.build_impl_stmt(impl_stmt),
			ast::Stmt::Expr(expr) => {
				let _ = self.build_expr(expr);
			}
			_ => todo!("code {:#?}", stmt),
		}
	}
}
