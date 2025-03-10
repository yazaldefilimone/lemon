use crate::checker::types::TypeStore;
use crate::ir::{self};
use crate::source::Source;
use env::Env;
use inkwell::{builder::Builder, context::Context, module::Module};

// cmp
mod llvm_cmp_utils;
mod llvm_compile_cmp_eq;
mod llvm_compile_cmp_ge;
mod llvm_compile_cmp_gt;
mod llvm_compile_cmp_le;
mod llvm_compile_cmp_lt;
mod llvm_compile_string;
// logic
// mod llvm_compile_neg;
// mod llvm_compile_not;
mod llvm_compile_and;
mod llvm_compile_or;
mod llvm_compile_shl;
mod llvm_compile_shr;

// math
mod llvm_compile_add;
mod llvm_compile_div;
mod llvm_compile_mod;
mod llvm_compile_mul;
mod llvm_compile_sub;

// fn
mod llvm_compile_block;
mod llvm_compile_call;
mod llvm_compile_function;
mod llvm_compile_instr;
// control
mod llvm_compile_jmp;
mod llvm_compile_jmp_if;
// mem
mod llvm_compile_drop;
mod llvm_compile_getptr;
mod llvm_compile_halloc;
mod llvm_compile_load;
mod llvm_compile_ret;
mod llvm_compile_salloc;
mod llvm_compile_set;
mod llvm_memory;

// structs
mod llvm_compile_struct;

// other
mod env;
mod llvm_compile_type;
mod llvm_compile_value;

pub fn create_module_from_source<'ll>(ctx: &'ll Context, source: &Source) -> Module<'ll> {
	let module = ctx.create_module(source.name());
	module
}

pub struct Llvm<'ll> {
	pub ctx: &'ll Context,
	pub module: Module<'ll>,
	pub builder: Builder<'ll>,
	pub env: Env<'ll>,
	pub type_store: &'ll TypeStore,
}

impl<'ll> Llvm<'ll> {
	pub fn new(ctx: &'ll Context, module: Module<'ll>, type_store: &'ll TypeStore) -> Self {
		let builder = ctx.create_builder();
		let env = Env::new();
		Self { ctx, module, builder, type_store, env }
	}

	pub fn compile_ir(&mut self, root: &ir::IR) {
		// structs
		//

		root.structs.iter().for_each(|struct_def| {
			self.llvm_compile_struct(struct_def);
		});

		// functions
		//
		root.functions.iter().for_each(|function| {
			self.llvm_compile_function(function);
		});
	}
}
