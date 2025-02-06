use crate::{checker::types::TypeStore, ir};
mod block;
mod function;
mod instr;
mod value;

pub struct Disassembler<'ir> {
	pub type_store: &'ir TypeStore,
}

impl<'ir> Disassembler<'ir> {
	pub fn new(type_store: &'ir TypeStore) -> Self {
		Self { type_store }
	}

	pub fn disassemble_program(&self, program: &'ir ir::IR, output: &mut String) {
		for func in &program.funcs {
			self.disassemble_function(func, output);
		}
	}
}
