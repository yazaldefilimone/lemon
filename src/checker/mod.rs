use crate::{ast, messages::Message};

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
) -> Result<()> {
	match synthesis::synthesise_block(ctx, &file.block) {
		Ok(typed) => {}
		Err(message) => return Err(message),
	}
	return Ok(());
}
