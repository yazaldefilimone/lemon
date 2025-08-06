use crate::{
	ast::{self, Declaration, Node},
	messages::Messages,
	parser::{
		check_not_reserved, disallow_all_attributes, parse_attributes, parse_expression, parse_type,
	},
	token::{Token, TokenKind},
	token_reader::TokenReader,
};

type Result<T> = std::result::Result<T, ()>;

pub(super) fn parse_statements<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Vec<super::ast::Statement<'a>> {
	let mut items = Vec::new();

	loop {
		while let Ok(token) = reader.peek() {
			if token.kind != TokenKind::Newline {
				break;
			}
			if reader.next(messages).is_err() {
				return items;
			}
		}

		let attributes = match parse_attributes(reader, messages) {
			Ok(attributes) => attributes,
			Err(_) => {
				super::consume_error_syntax(reader, messages);
				continue;
			}
		};

		reader.read_newlines();

		let token = match reader.peek() {
			Ok(token) => token,
			Err(_) => return items,
		};

		if token.kind == TokenKind::CloseBrace {
			break;
		}
		if let Some(statement) = parse_statement(reader, messages, attributes) {
			items.push(statement);
			reader.read_semicolon(messages).ok();
		}
	}

	items
}

pub fn parse_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
	attributes: ast::Attributes<'a>,
) -> Option<ast::Statement<'a>> {
	let peeked = reader.peek().ok()?;
	match peeked {
		Token { kind: TokenKind::Word, text: "let", .. } => {
			disallow_all_attributes(messages, attributes, peeked.span, "a let statement");
			if let Ok(declaration) = parse_let_statement(reader, messages) {
				return Some(ast::Statement::Declaration(declaration));
			}
			super::consume_error_syntax(reader, messages);
			return None;
		}
		Token { kind: TokenKind::Word, text: "const", .. } => {
			super::consume_error_syntax(reader, messages);
			return None;
		}
		Token { kind: TokenKind::Word, text: "fn", .. } => {
			super::consume_error_syntax(reader, messages);
			return None;
		}
		Token { kind: TokenKind::Word, text: "type", .. } => {
			super::consume_error_syntax(reader, messages);
			return None;
		}
		Token { kind: TokenKind::Word, text: "enum", .. } => {
			super::consume_error_syntax(reader, messages);
			return None;
		}
		_ => return None,
	}
}

pub fn parse_let_statement<'a>(
	reader: &mut TokenReader<'a>,
	messages: &mut Messages,
) -> Result<Node<ast::Declaration<'a>>> {
	let mut span = reader.expect_word("let", messages)?.span;
	let mutable = reader.read_if_word("mut", messages).is_ok();

	let name_token = reader.expect(TokenKind::Word, messages)?;
	check_not_reserved(messages, name_token, "variable name")?;
	let name = ast::Node::from_token(name_token.text, name_token);
	let parsed_type = if reader.peek_kind() == Ok(&TokenKind::Colon) {
		reader.next(messages)?;
		Some(parse_type(reader, messages)?)
	} else {
		None
	};

	reader.expect(TokenKind::Equal, messages)?;

	let expression = parse_expression(reader, messages, true)?;
	span += expression.span;
	let let_stmt = ast::Let { name, parsed_type, expression, mutable };
	Ok(ast::Node::from_span(Declaration::Let(let_stmt), span))
}
