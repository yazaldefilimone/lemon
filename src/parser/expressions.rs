use crate::{
	ast::{Expression, Node},
	lexer::token::TokenStream,
	messages::{Messages, ParseResult},
};

pub fn parse_expression<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
) -> ParseResult<Node<Expression<'a>>> {
	todo!()
}
