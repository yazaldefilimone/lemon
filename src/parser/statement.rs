use crate::{
	ast::{self, Node, Parameters},
	error,
	messages::Messages,
	parser::check_not_reserved,
	token::{Token, TokenKind},
	token_reader::TokenReader,
};

type Result<T> = std::result::Result<T, ()>;

pub fn parse_statements<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Vec<super::ast::Statement<'a>> {
	let mut statements = Vec::new();
	reader.read_newlines();
	while let Ok(token) = reader.peek() {
		if token.kind == TokenKind::CloseBrace {
			break;
		}

		let attributes = match super::parse_attributes(reader, messages) {
			Ok(attributes) => attributes,
			Err(_) => {
				super::consume_error_syntax(reader, messages);
				continue;
			}
		};

		reader.read_newlines();

		if let Some(statement) = parse_statement(reader, messages, attributes) {
			statements.push(statement);

			if let Err(_) = reader.read_semicolon(messages) {
				super::consume_error_syntax(reader, messages);
			}

			reader.read_newlines();
		}
	}
	statements
}

pub fn parse_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	attributes: ast::Attributes<'a>,
) -> Option<ast::Statement<'a>> {
	let peeked = reader.peek().ok()?;

	match peeked {
		Token { kind: TokenKind::Word, text: "let", .. } => {
			super::disallow_all_attributes(messages, attributes, peeked.span, "a let statement");

			if let Ok(declaration) = parse_let_statement(reader, messages) {
				return Some(ast::Statement::Let(declaration));
			}

			super::consume_error_syntax(reader, messages);
			None
		}
		Token { kind: TokenKind::Word, text: "fn", .. } => {
			super::disallow_all_attributes(messages, attributes, peeked.span, "a function declaration");
			if let Ok(function) = parse_function_statement(reader, messages) {
				return Some(ast::Statement::Function(function));
			}
			super::consume_error_syntax(reader, messages);
			None
		}

		Token { kind: TokenKind::Word, text: "return", .. } => {
			super::disallow_all_attributes(messages, attributes, peeked.span, "a return statement");

			if let Ok(expression) = parse_return_statement(reader, messages) {
				return Some(ast::Statement::Return(expression));
			}
			super::consume_error_syntax(reader, messages);
			None
		}

		// Token { kind: TokenKind::Word, .. } => unsupported_statement(reader, messages, &peeked),
		_ => {
			super::disallow_all_attributes(messages, attributes, peeked.span, "a expression");
			if let Ok(expression) = super::parse_expression(reader, messages, true) {
				let expression = ast::Statement::Expression(expression);
				if reader.expect_peek(TokenKind::Semicolon, messages).is_err() {
					super::consume_error_syntax(reader, messages);
				}
				return Some(expression);
			} else {
				super::consume_error_syntax(reader, messages);
			}
			None
		}
	}
}

pub fn parse_let_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Let<'a>>> {
	let mut span = reader.expect_word("let", messages)?.span;
	let mutable = reader.read_if_word("mut", messages).is_ok();

	let name_token = reader.expect(TokenKind::Word, messages)?;
	super::check_not_reserved(messages, name_token, "variable name")?;
	let name = ast::Node::from_token(name_token.text, name_token);

	let parsed_type = if reader.read_if(TokenKind::Colon, messages).is_ok() {
		Some(super::parse_type(reader, messages)?)
	} else {
		None
	};

	reader.expect(TokenKind::Equal, messages)?;
	let expression = super::parse_expression(reader, messages, true)?;
	span += expression.span;

	let pattern = ast::Pattern::Identifier(name);

	let declaration = ast::Let { pattern, parsed_type, expression, mutable };
	Ok(ast::Node::from_span(declaration, span))
}

fn parse_function_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Function<'a>>> {
	let span = reader.expect_word("fn", messages)?.span;

	let name = reader.expect(TokenKind::Word, messages)?;
	super::check_not_reserved(messages, name, "function name")?;
	let name_node = ast::Node::from_token(name.text, name);

	let parameters = parse_function_parameters(reader, messages)?;

	let return_type = if reader.read_if(TokenKind::Colon, messages).is_ok() {
		Some(super::parse_type(reader, messages)?)
	} else {
		None
	};

	reader.expect(TokenKind::Equal, messages)?;

	let body = parse_block(reader, messages)?;

	let final_span = span + body.span;
	let function = ast::Function::new(name_node, parameters, return_type, Some(body));
	Ok(ast::Node::from_span(function, final_span))
}

fn parse_function_parameters<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<Parameters<'a>>> {
	let open_paren = reader.expect(TokenKind::OpenParen, messages)?;
	let mut parameters = Vec::new();
	let mut c_varargs = None;
	reader.read_newlines();

	while let Ok(token) = reader.peek() {
		if token.kind == TokenKind::CloseParen {
			break;
		}
		if reader.peek_kind() == Ok(TokenKind::TripleDot) {
			let triple_dot = reader.next(messages)?;
			c_varargs = Some(triple_dot.span);
			break;
		}
		let parameter = parse_function_parameter(reader, messages)?;
		parameters.push(parameter);

		if let Err(_) = reader.read_if(TokenKind::Comma, messages) {
			super::consume_error_syntax(reader, messages);
			break;
		}
	}
	let close_paren = reader.expect(TokenKind::CloseParen, messages)?;

	let parameters = ast::Parameters { parameters, c_varargs };

	Ok(ast::Node::new(parameters, open_paren.span + close_paren.span))
}

fn parse_function_parameter<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<ast::Node<ast::Parameter<'a>>> {
	let name = reader.expect(TokenKind::Word, messages)?;
	reader.expect(TokenKind::Colon, messages)?;
	let param_type = super::parse_type(reader, messages)?;

	check_not_reserved(messages, name, "parameter label")?;

	let name_node = ast::Node::from_token(name.text, name);

	let span = name.span + param_type.span;
	Ok(Node::new(ast::Parameter { name: name_node, param_type, mutable: false }, span))
}

fn parse_return_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Expression<'a>>> {
	let span = reader.expect_word("return", messages)?.span;
	let expression = super::parse_expression(reader, messages, true)?;
	Ok(ast::Node::from_span(expression.item, span + expression.span))
}

pub fn parse_block<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Block<'a>>> {
	let open_brace = reader.expect(TokenKind::OpenBrace, messages)?;

	let statements = parse_statements(reader, messages);
	let close_brace = reader.expect(TokenKind::CloseBrace, messages)?;

	let span = open_brace.span + close_brace.span;

	Ok(ast::Node::from_span(ast::Block { statements }, span))
}

//==== helpers
//

fn skip_newlines(reader: &mut TokenReader, messages: &mut Messages) {
	while let Ok(token) = reader.peek() {
		if token.kind != TokenKind::Newline {
			break;
		}
		if reader.next(messages).is_err() {
			break;
		}
	}
}

fn unsupported_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	token: &Token<'a>,
) -> Option<ast::Statement<'a>> {
	let message = error!("unsupported statement '{}'", token.text);
	messages.message(message.with_span(token.span));
	super::consume_error_syntax(reader, messages);
	None
}
