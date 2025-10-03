use crate::{ast, hir, messages::Message};
pub mod context;
pub mod expressions;
pub mod functions;
pub mod statements;
pub mod synthesis;
pub mod types;
pub mod variables;

pub type Result<T> = std::result::Result<T, Message>;

pub fn check_file<'a, 'b>(
	ctx: &mut context::Context<'a, 'b>,
	file: &'a ast::File<'a>,
) -> Result<hir::File<'a>> {
	let block = check_block(ctx, &file.block)?;

	match synthesis::synthesise_block(ctx, &file.block) {
		Ok(typed) => {}
		Err(message) => return Err(message),
	}
	return Ok(hir::File::new(block, &file.file));
}

pub fn check_block<'a, 'b>(
	ctx: &mut context::Context<'a, 'b>,
	block: &'a ast::Block<'a>,
) -> Result<hir::Block<'a>> {
	let mut hir_block = hir::Block::new();
	for statement in &block.statements {
		// let statement = check_statement(ctx, statement)?;
	}
	return Ok(hir_block);
}
