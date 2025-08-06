use super::Result;
use crate::{
	ast::{self, Node},
	messages::Messages,
	token_reader::TokenReader,
};

pub fn parse_type<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Type<'a>>> {
	todo!()
}
