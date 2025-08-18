use super::Result;
use crate::{
	ast::{self, ExternAttribute, Node, PubAttribute},
	error,
	messages::Messages,
	note,
	span::Span,
	token::TokenKind,
	token_reader::TokenReader,
};

pub fn parse_attributes<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<ast::Attributes<'a>> {
	let attributes = ast::Attributes::blank();

	while let Ok(peeked) = reader.peek() {
		if peeked.kind == TokenKind::CloseBrace {
			break;
		}
		// todo: implement this
		break;
	}
	Ok(attributes)
}

fn check_duplicate_attribute<T>(
	messages: &mut Messages,
	attribute: Option<ast::Node<T>>,
	attribute_name: &str,
	duplicate_span: Span,
) -> Result<()> {
	if let Some(attribute) = attribute {
		let message = error!("duplicate attribute {attribute_name:?}")
			.with_span(duplicate_span)
			.with_note(note!(attribute.span, "original here"));
		messages.message(message);
		return Err(());
	}

	Ok(())
}
pub fn parse_pub_attribute<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::PubAttribute<'a>>> {
	// todo: remove clone here
	let pub_token = reader.expect_word("pub", messages)?.clone();
	let name_token = reader.expect(TokenKind::String, messages)?.clone();

	reader.expect(TokenKind::Newline, messages)?;
	let attribute = PubAttribute { name: name_token.text };
	let span = pub_token.span + name_token.span;
	Ok(Node::new(attribute, span))
}

pub fn parse_extern_attribute<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ExternAttribute<'a>>> {
	// todo: remove clone here
	let extern_token = reader.expect_word("extern", messages)?.clone();
	let name_token = reader.expect(TokenKind::String, messages)?.clone();

	reader.expect(TokenKind::Newline, messages)?;

	let attribute = ExternAttribute { name: name_token.text };
	let span = extern_token.span + name_token.span;
	Ok(Node::new(attribute, span))
}
