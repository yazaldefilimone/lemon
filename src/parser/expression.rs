use super::Result;
use crate::{
	ast::{self, Node},
	messages::Messages,
	parser::{check_not_reserved, parse_number, parse_string_contents},
	token::{Token, TokenKind},
	token_reader::TokenReader,
};

pub fn parse_expression<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	parse_precedence(reader, messages, allow_struct_literal, 0)
}

fn parse_precedence<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	allow_struct_literal: bool,
	min_precedence: u32,
) -> Result<Node<ast::Expression<'a>>> {
	let mut left = parse_primary(reader, messages)?;

	left = parse_postfix_chain(reader, messages, left, allow_struct_literal)?;

	loop {
		if let Some(operator) = reader.peek().ok().and_then(super::token_to_operator) {
			let precedence = operator.item.precedence();
			if precedence < min_precedence {
				break;
			}

			reader.next(messages).expect("known peeked token");

			let associativity = operator.item.associativity();
			let next_min = match associativity {
				ast::Associativity::Left => precedence + 1,
				ast::Associativity::Right => precedence,
			};

			let right = parse_precedence(reader, messages, allow_struct_literal, next_min)?;
			let span = left.span + right.span;

			let binary_operation = ast::BinaryOperation::new(operator, left, right);
			let expression = ast::Expression::BinaryOperation(binary_operation);
			left = Node::new(expression, span);
			continue;
		}

		if let Ok(Token { text: "is", .. }) = reader.peek() {
			left = parse_is_expression(reader, messages, left)?;
			continue;
		}

		break;
	}
	Ok(left)
}

fn parse_postfix_chain<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	mut expression: Node<ast::Expression<'a>>,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	loop {
		match reader.peek_kind() {
			Ok(TokenKind::Dot) => {
				reader.next(messages)?;
				expression = parse_postfix(reader, messages, expression, allow_struct_literal)?;
			}
			Ok(TokenKind::OpenBracket) => {
				expression = parse_bracket_index(reader, messages, expression)?;
			}
			_ => break,
		}
	}
	Ok(expression)
}

fn parse_is_expression<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	left: Node<ast::Expression<'a>>,
) -> Result<Node<ast::Expression<'a>>> {
	reader.next(messages)?;

	let (binding_name, variant_names) = {
		let first = reader.expect(TokenKind::Word, messages)?.clone();

		if reader.peek_kind() == Ok(&TokenKind::Colon) {
			reader.next(messages)?;
			let variant_token = reader.expect(TokenKind::Word, messages)?.clone();
			check_not_reserved(messages, first, "`is` operator binding name")?;

			let binding = Some(Node::from_token(first.text, first));
			let variant = Node::from_token(variant_token.text, variant_token);
			(binding, vec![variant])
		} else {
			let mut variants = vec![Node::from_token(first.text, first)];
			while reader.peek_kind() == Ok(&TokenKind::Comma) {
				reader.next(messages)?;
				let token = reader.expect(TokenKind::Word, messages)?.clone();
				variants.push(Node::from_token(token.text, token));
			}
			(None, variants)
		}
	};

	let right_span = variant_names.last().unwrap().span;
	let span = left.span + right_span;
	let check_is = ast::CheckIs::new(left, binding_name, variant_names);
	let expression = ast::Expression::CheckIs(check_is);
	Ok(Node::new(expression, span))
}

fn parse_primary<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Expression<'a>>> {
	let peeked = reader.peek()?;

	match peeked.kind {
		TokenKind::Number => {
			return parse_number(reader, messages);
		}

		TokenKind::String => {
			let string_token = reader.next(messages)?;
			let value = parse_string_contents(string_token.text);

			let string_literal = ast::StringLiteral { value };
			let expression = ast::Expression::StringLiteral(string_literal);

			return Ok(Node::from_span(expression, string_token.span));
		}
		TokenKind::FormatString => {
			todo!()
		}
		TokenKind::OpenParen => {
			todo!()
			// tokens.next(messages)?;
			// // Regardless of if parent parsing context disallowed struct literals, we override that within parenthesis
			// let expression = parse_expression(bump, messages, tokens, true)?;
			// tokens.expect(messages, TokenKind::CloseParen)?;
			// Ok(expression)
		}

		TokenKind::OpenBrace => {
			todo!()
			// let parsed_block = parse_block(bump, messages, tokens, false)?;

			// let span = parsed_block.span;
			// let block = parsed_block.item;

			// Ok(Node::new(Expression::Block(block), span))
		}

		// TokenKind::Dot => parse_dot_infer(bump, messages, tokens, allow_struct_literal),
		TokenKind::Word | TokenKind::DoubleColon => {
			match peeked.text {
				"if" => {
					todo!();
					// let node = parse_if_else_chain(bump, messages, tokens)?;
					// let expression = Expression::IfElseChain(bump.alloc(node.item));
					// return Ok(Node::new(expression, node.span));
				}

				"match" => {
					todo!();
					// let node = parse_match(bump, messages, tokens)?;
					// let expression = Expression::Match(bump.alloc(node.item));
					// return Ok(Node::new(expression, node.span));
				}

				"true" => {
					todo!();
					// tokens.next(messages)?;
					// return Ok(Node::new(Expression::BooleanLiteral(true), peeked.span));
				}

				"false" => {
					todo!();
					// tokens.next(messages)?;
					// return Ok(Node::new(Expression::BooleanLiteral(false), peeked.span));
				}

				_ => {
					todo!()
				}
			}
			// parse_path_expression(bump, messages, tokens, None, allow_struct_literal)
		}
		_ => {
			todo!()
		}
	};
}

fn parse_postfix<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	left: Node<ast::Expression<'a>>,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	todo!()
}

fn parse_bracket_index<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	left: Node<ast::Expression<'a>>,
) -> Result<Node<ast::Expression<'a>>> {
	todo!()
}

// fn parse_infix_expression<'a>(
// 	reader: &mut TokenReader<'a>,
// 	messages: &mut Messages,
// ) -> Result<Node<ast::Expression<'a>>> {
// 	todo!()
// }
