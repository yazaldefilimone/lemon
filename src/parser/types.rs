use crate::{
	ast::{Node, Type},
	lexer::token::TokenStream,
	messages::{Messages, ParseResult},
};

pub fn parse_type<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
) -> ParseResult<Node<Type<'a>>> {
	todo!()
}
