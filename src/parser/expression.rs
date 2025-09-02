use super::Result;
use crate::{
	ast::{self, Node},
	error,
	messages::Messages,
	token::TokenKind,
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
	let mut left = parse_primary(reader, messages, allow_struct_literal)?;
	left = parse_postfix_chain(reader, messages, left, allow_struct_literal)?;

	while let Some(operator) = reader.peek().ok().and_then(super::token_to_operator) {
		let precedence = operator.item.precedence();
		if precedence < min_precedence {
			break;
		}

		reader.next(messages).expect("known peeked token");

		let associativity = operator.item.associativity();
		let next_min_precedence = match associativity {
			ast::Associativity::Left => precedence + 1,
			ast::Associativity::Right => precedence,
		};

		let right = parse_precedence(reader, messages, allow_struct_literal, next_min_precedence)?;
		let span = left.span + right.span;

		let binary_operation = ast::BinaryOperation::new(operator, left, right);
		let expression = ast::Expression::BinaryOperation(binary_operation);
		left = Node::new(expression, span);
	}

	Ok(left)
}

fn parse_primary<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	let peeked = reader.peek()?;

	match peeked.kind {
		TokenKind::Sub => {
			let token = reader.next(messages)?;
			let operator = Node::new(ast::UnaryOperator::Negate, token.span);
			let expression = parse_expression(reader, messages, allow_struct_literal)?;
			let span = token.span + expression.span;
			let negate = ast::UnaryOperation::new(operator, expression);
			Ok(Node::new(ast::Expression::UnaryOperation(negate), span))
		}

		TokenKind::Number => super::parse_number(reader, messages),

		TokenKind::String => {
			let string_token = reader.next(messages)?;
			let value = super::parse_string_contents(string_token.text);
			let string_literal = ast::StringLiteral { value };
			let expression = ast::Expression::StringLiteral(string_literal);
			Ok(Node::from_span(expression, string_token.span))
		}

		TokenKind::FormatString => todo!("format string parsing"),

		TokenKind::Word | TokenKind::DoubleColon => {
			parse_word_or_path_expression(reader, messages, allow_struct_literal)
		}
		TokenKind::OpenParen => {
			reader.next(messages)?;
			let expression = parse_expression(reader, messages, true)?;
			reader.expect(TokenKind::CloseParen, messages)?;
			Ok(expression)
		}

		TokenKind::OpenBrace => {
			let parsed_block = super::parse_block(reader, messages)?;
			let span = parsed_block.span;
			let block = parsed_block.item;
			Ok(Node::new(ast::Expression::Block(block), span))
		}
		TokenKind::Dot => parse_dot_infer(reader, messages, allow_struct_literal),
		TokenKind::OpenBracket => parse_primary_bracket(reader, messages),

		_ => {
			let message = error!("unexpected token {} in primary expression", reader.peek_kind()?);
			messages.message(message.with_span(reader.peek()?.span));
			Err(())
		}
	}
}

fn parse_word_or_path_expression<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	let peeked = reader.peek()?;

	match peeked.text {
		"true" | "false" => {
			let token = reader.next(messages)?;
			let bool_value = token.text == "true";
			let expression = ast::Expression::BooleanLiteral(bool_value);
			Ok(Node::new(expression, token.span))
		}
		"if" => {
			let node = parse_if_else_chain(reader, messages)?;
			let expression = ast::Expression::IfElseChain(Box::new(node.item));
			Ok(Node::new(expression, node.span))
		}
		"match" => {
			let node = parse_match(reader, messages)?;
			let expression = ast::Expression::Match(Box::new(node.item));
			Ok(Node::new(expression, node.span))
		}
		_ => parse_path_expression(reader, messages, None, allow_struct_literal),
	}
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
				expression = parse_following_dot(reader, messages, expression, allow_struct_literal)?;
			}
			Ok(TokenKind::OpenBracket) => {
				expression = parse_bracket_index(reader, messages, expression)?;
			}
			_ => break,
		}
	}
	Ok(expression)
}

fn parse_bracket_index<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	left: Node<ast::Expression<'a>>,
) -> Result<Node<ast::Expression<'a>>> {
	reader.expect(TokenKind::OpenBracket, messages)?;
	let expression = parse_expression(reader, messages, true)?;
	let close_token = reader.expect(TokenKind::CloseBracket, messages)?;

	let span = left.span + close_token.span;
	let operator = Node::new(ast::UnaryOperator::Index { expression }, span);
	let operation = ast::UnaryOperation::new(operator, left);
	let expression = ast::Expression::UnaryOperation(operation);
	Ok(Node::new(expression, span))
}

fn parse_following_dot<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	left: Node<ast::Expression<'a>>,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	todo!("dot access parsing")
}

fn parse_arguments<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<Vec<ast::Argument<'a>>>> {
	let open_paren = reader.expect(TokenKind::OpenParen, messages)?;
	let mut arguments = vec![];
	while reader.peek_kind()? != TokenKind::CloseParen {
		let expression = super::parse_expression(reader, messages, true)?;
		arguments.push(ast::Argument { expression });
	}
	let close_paren = reader.expect(TokenKind::CloseParen, messages)?;
	Ok(Node::new(arguments, open_paren.span + close_paren.span))
}

fn parse_struct_initializer<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::StructInitializer<'a>>> {
	todo!("struct initializer parsing")
}

fn parse_primary_bracket<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Expression<'a>>> {
	todo!("primary bracket parsing")
}

fn parse_path_expression<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	previous: Option<Node<ast::Expression<'a>>>,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	let word_token = reader.expect(TokenKind::Word, messages)?;
	let name = Node::from_token(word_token.text, word_token);

	match reader.peek_kind() {
		Ok(TokenKind::OpenParen) => {
			let arguments = parse_arguments(reader, messages)?;
			let span = word_token.span + arguments.span;
			let call = ast::Call::new(previous, name, vec![], arguments.item);
			return Ok(Node::new(ast::Expression::Call(call), span));
		}
		Ok(TokenKind::OpenBrace) if allow_struct_literal => {
			todo!("struct literal parsing")
			// let base_expr = if let Some(prev) = previous {
			// 	let access = bump.alloc(ast::DotAccess { base: prev, name, type_arguments });
			// 	Node::new(ast::Expression::DotAccess(access), total_span)
			// } else {
			// 	let read = ast::Read { name, type_arguments };
			// 	Node::new(ast::Expression::Read(read), total_span)
			// };
			// let init = parse_struct_initializer(reader, messages)?;
			// let span = total_span + init.span;
			// let literal = bump.alloc(ast::StructLiteral { base: base_expr, initializer: init });
			// return Ok(Node::new(Expression::StructLiteral(literal), span));
		}
		_ => {}
	}
	todo!("path expression parsing")

	// let expression = if let Some(has_previous) = previous {
	// 	let access = bump.alloc(DotAccess { base: prev, name, type_arguments });
	// 	Node::new(Expression::DotAccess(access), total_span)
	// } else {
	// 	let read = Read { name, type_arguments };
	// 	Node::new(Expression::Read(read), total_span)
	// };

	// if tokens.peek_kind() == Ok(TokenKind::Period) {
	// 	tokens.next(messages)?;
	// 	if tokens.peek_kind() != Ok(TokenKind::Word) {
	// 		return parse_following_period(bump, messages, tokens, expression, allow_struct_literal);
	// 	}
	// 	return parse_path_expression(bump, messages, tokens, Some(expression), allow_struct_literal);
	// }

	// Ok(expression)
}

fn parse_dot_infer<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	allow_struct_literal: bool,
) -> Result<Node<ast::Expression<'a>>> {
	todo!("dot infer parsing")
}

fn parse_type_arguments<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Vec<Node<ast::Type<'a>>>> {
	todo!("type arguments parsing")
}

fn parse_if_else_chain<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::IfElseChain<'a>>> {
	todo!("if-else chain parsing")
}

fn parse_match<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Match<'a>>> {
	todo!("match parsing")
}
