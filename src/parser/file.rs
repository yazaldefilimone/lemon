use crate::{
	ast::{Block, File},
	lexer::token::TokenStream,
	loader::SourceFile,
	messages::Messages,
	parser::parse_statements,
};

pub fn parse_file<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
	file: &'a SourceFile,
) -> File<'a> {
	let block = parse_root_block(messages, tokens);
	File { source_file: file, block }
}

pub fn parse_root_block<'a>(messages: &mut Messages, tokens: &mut TokenStream<'a>) -> Block<'a> {
	let statements = parse_statements(messages, tokens);
	Block { statements }
}
