use crate::{ast, messages::Messages, resolver::file::SourceFile, token_reader::TokenReader};

mod attribute;
mod expression;
mod number;
mod statement;
mod types;
mod utils;
pub use attribute::*;
pub use expression::*;
pub use number::*;
pub use statement::*;
pub use types::*;
pub use utils::*;

pub type Result<T> = std::result::Result<T, ()>;

pub fn parse_file<'a>(
	file: &'a SourceFile,
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> ast::File<'a> {
	let block = parse_root_block(reader, messages);
	ast::File { file, block }
}

fn parse_root_block<'a>(reader: &mut TokenReader<'a>, messages: &mut Messages) -> ast::Block<'a> {
	let statements = parse_statements(reader, messages);
	ast::Block { statements }
}
