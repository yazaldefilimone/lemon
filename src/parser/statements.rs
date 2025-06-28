use crate::ast::*;
use crate::lexer::token::*;
use crate::lexer::*;
use crate::messages::*;
use crate::parser::expressions::*;
use crate::parser::*;

pub fn parse_block<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
) -> ParseResult<Node<Block<'a>>> {
	let open = tokens.expect(messages, TokenKind::OpenBrace)?;
	let statements = parse_statements(messages, tokens);
	let close = tokens.expect(messages, TokenKind::CloseBrace)?;

	let block = Block { statements };
	let span = open.span + close.span;
	Ok(Node::new(block, span))
}

pub fn parse_statements<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
) -> Vec<Statement<'a>> {
	let mut statements = Vec::new();

	while let Ok(token) = tokens.peek() {
		if token.kind == TokenKind::Newline {
			break;
		}
		tokens.skip_newlines();
		let token = match tokens.peek() {
			Ok(token) => token,
			Err(_) => return statements,
		};

		if token.kind == TokenKind::CloseBrace {
			break;
		}

		if let Some(statement) = parse_statement(messages, tokens) {
			statements.push(statement);
		}
	}
	statements
}

fn parse_statement<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
) -> Option<Statement<'a>> {
	let peeked = tokens.peek().ok()?;
	match peeked {
		Token { kind: TokenKind::Word, text: "let", .. } => {
			if let Ok(statement) = parse_let_statement(messages, tokens) {
				return Some(Statement::Let(statement));
			}
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "pub", .. } => {
			// if let Ok(statement) = parse_pub_statement(messages, tokens) {
			// 	return Some(Statement::Pub(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "fn", .. } => {
			// if let Ok(statement) = parse_fn_statement(messages, tokens) {
			// 	return Some(Statement::Fn(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "const", .. } => {
			// if let Ok(statement) = parse_const_statement(messages, tokens) {
			// 	return Some(Statement::Const(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "struct", .. } => {
			// if let Ok(statement) = parse_struct_statement(messages, tokens) {
			// 	return Some(Statement::Struct(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "enum", .. } => {
			// if let Ok(statement) = parse_enum_statement(messages, tokens) {
			// 	return Some(Statement::Enum(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "union", .. } => {
			// if let Ok(statement) = parse_union_statement(messages, tokens) {
			// 	return Some(Statement::Union(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "if", .. } => {
			// if let Ok(statement) = parse_if_statement(messages, tokens) {
			// 	return Some(Statement::If(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "match", .. } => {
			// if let Ok(statement) = parse_match_statement(messages, tokens) {
			// 	return Some(Statement::Match(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "while", .. } => {
			// if let Ok(statement) = parse_while_statement(messages, tokens) {
			// 	return Some(Statement::While(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "for", .. } => {
			// if let Ok(statement) = parse_for_statement(messages, tokens) {
			// 	return Some(Statement::For(statement));
			// }
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "break", .. } => {
			// let break_token = tokens.expect_word(messages, "break")?;
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "continue", .. } => {
			// let continue_token = tokens.expect_word(messages, "continue")?;
			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "yield", .. } => {
			// let yield_token = tokens.expect_word(messages, "yield")?;

			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::Word, text: "return", .. } => {
			// let return_token = tokens.expect_word(messages, "return")?;
			// let expression = parse_expression(messages, tokens)?;

			// tokens.expect_peek(messages, TokenKind::Newline)?;

			// let span = return_token.span + expression.span;
			// let item = Return { expression, span };

			// return Some(Statement::Return(Node { item, span }));

			consume_error_syntax(messages, tokens);
			return None;
		}

		Token { kind: TokenKind::OpenBrace, .. } => {
			// let block = parse_block(messages, tokens)?;
			// let span = block.span;
			// let item = BlockStatement { block, span };

			// return Some(Statement::Block(Node { item, span }));
			consume_error_syntax(messages, tokens);
			return None;
		}

		_ => {
			// expression_statement

			consume_error_syntax(messages, tokens);
			return None;
		}
	}
}

// let mut name: type = expression;
fn parse_let_statement<'a>(
	messages: &mut Messages,
	tokens: &mut TokenStream<'a>,
) -> ParseResult<Node<Let<'a>>> {
	let let_token = tokens.expect_word(messages, "let")?;

	let mut mutable = false;

	if let Ok(Token { kind: TokenKind::Word, text: "mut", .. }) = tokens.peek() {
		mutable = true;
		tokens.next(messages)?;
	}

	let name_token = tokens.expect(messages, TokenKind::Word)?;
	check_not_reserved(messages, name_token, "variable name")?;

	let name = Node::from_token(name_token.text, name_token);

	let parsed_type = if tokens.peek_kind() == Ok(TokenKind::Colon) {
		tokens.next(messages)?;
		Some(parse_type(messages, tokens)?)
	} else {
		None
	};

	tokens.expect(messages, TokenKind::Equal)?;

	let expression = parse_expression(messages, tokens)?;

	tokens.expect_peek(messages, TokenKind::Newline)?;

	let span = let_token.span + expression.span;
	let item = Let { name, parsed_type, expression, mutable, span };

	Ok(Node { item, span })
}
