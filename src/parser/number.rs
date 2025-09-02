use rust_decimal::Decimal;

use super::Result;
use crate::{
	ast::{self, Expression, Node},
	error,
	messages::Messages,
	span::Span,
	token::TokenKind,
	token_reader::TokenReader,
};

pub fn parse_number<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<Expression<'a>>> {
	let token = reader.expect(TokenKind::Number, messages)?;
	let span = token.span;
	let text = token.text;
	let bytes = text.as_bytes();

	if text.len() >= 3 && bytes[0] == b'0' {
		match bytes[1] {
			b'x' => return parse_hex_literal(&text[2..], span, messages),
			b'b' => return parse_binary_literal(&text[2..], span, messages),
			_ => {}
		}
	}

	// Decimal or integer
	match Decimal::from_str_exact(text) {
		Ok(value) => Ok(make_number_node(value, span)),
		Err(_) => {
			let message = error!("invalid number literal");
			messages.message(message.with_span(span));
			Err(())
		}
	}
}

fn parse_hex_literal<'a>(
	text: &str,
	span: Span,
	messages: &mut Messages,
) -> Result<Node<Expression<'a>>> {
	match Decimal::from_str_radix(text, 16) {
		Ok(value) => Ok(make_number_node(value, span)),
		Err(_) => {
			let message = error!("invalid hexadecimal number literal").with_span(span);
			messages.message(message);
			Err(())
		}
	}
}

fn parse_binary_literal<'a>(
	text: &str,
	span: Span,
	messages: &mut Messages,
) -> Result<Node<Expression<'a>>> {
	let mut value = Decimal::ZERO;

	for (_, byte) in text.bytes().enumerate() {
		let bit = match byte {
			b'0' => Decimal::ZERO,
			b'1' => Decimal::ONE,
			_ => {
				let message = error!("Invalid binary digit").with_span(span);
				messages.message(message);
				return Err(());
			}
		};

		value = match value.checked_mul(Decimal::TWO) {
			Some(value) => value,
			None => {
				let message = error!("overflow while parsing binary literal").with_span(span);
				messages.message(message);
				return Err(());
			}
		};

		value = match value.checked_add(bit) {
			Some(v) => v,
			None => {
				let message = error!("overflow while parsing binary literal").with_span(span);
				messages.message(message);
				return Err(());
			}
		};
	}

	Ok(make_number_node(value, span))
}

fn make_number_node<'a>(value: Decimal, span: Span) -> Node<Expression<'a>> {
	let literal = ast::NumberLiteral { value: Node::new(value, span) };
	Node::new(Expression::NumberLiteral(literal), span)
}
