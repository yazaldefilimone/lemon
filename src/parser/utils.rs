use std::borrow::Cow;

use crate::{
	ast::{Attributes, BinaryOperator, Node},
	error,
	messages::Messages,
	note,
	span::Span,
	token::{Token, TokenKind},
	token_reader::TokenReader,
};

pub fn disallow_all_attributes(
	messages: &mut Messages,
	attributes: Attributes,
	span: Span,
	label: &str,
) {
	let mut buffer = [Span::unusable(); Attributes::FIELD_COUNT];
	let spans = attributes.attribute_spans(&mut buffer);

	if !spans.is_empty() {
		let mut message = error!("{label} does not allow attributes").with_span(span);
		for &span in spans {
			message = message.with_note(note!(span, "attribute here"));
		}
		messages.message(message);
	}
}

pub fn is_word_reserved(word: &str) -> bool {
	matches!(
		word,
		"const"
			| "fn"
			| "let"
			| "mut"
			| "return"
			| "struct"
			| "enum"
			| "union"
			| "opaque"
			| "import"
			| "generic"
			| "extern"
			| "export"
			| "static"
			| "self"
			| "if"
			| "else"
			| "while"
			| "for"
			| "match"
			| "or"
			| "and"
			| "is"
			| "in"
			| "of"
			| "defer"
			| "break"
			| "continue"
			| "yield"
			| "true"
			| "false"
	)
}

pub fn check_not_reserved(messages: &mut Messages, token: Token, use_as: &str) -> Result<(), ()> {
	if is_word_reserved(token.text) {
		let error = error!("cannot use reserved word {:?} as {use_as}", token.text);
		messages.message(error.with_span(token.span));
		Err(())
	} else {
		Ok(())
	}
}

pub fn consume_error_syntax<'a>(reader: &mut TokenReader<'a>, messages: &mut Messages) {
	let mut brackets = 0;
	let mut parens = 0;
	let mut braces = 0;

	let mut span = None;

	while let Ok(token) = reader.peek() {
		span = Some(token.span);
		let all_zero = brackets == 0 && parens == 0 && braces == 0;
		match token.kind {
			TokenKind::Newline if all_zero => break,

			TokenKind::OpenBracket => brackets += 1,
			TokenKind::CloseBracket => brackets -= 1,

			TokenKind::OpenParen => parens += 1,
			TokenKind::CloseParen => parens -= 1,

			TokenKind::OpenBrace => braces += 1,
			TokenKind::CloseBrace => braces -= 1,

			_ => {}
		}

		reader.next(messages).expect("this should never fail");
	}

	// reached end of file while unbalanced
	if brackets != 0 {
		messages.message(error!("unbalanced brackets").with_span_if_some(span));
	}
	if parens != 0 {
		messages.message(error!("unbalanced parentheses").with_span_if_some(span));
	}
	if braces != 0 {
		messages.message(error!("unbalanced braces").with_span_if_some(span));
	}
}

pub fn token_to_operator(token: Token) -> Option<Node<BinaryOperator>> {
	let operator = match token.kind {
		TokenKind::Equal => BinaryOperator::Assign,

		TokenKind::Add => BinaryOperator::Add,
		TokenKind::AddAssign => BinaryOperator::AddAssign,
		TokenKind::Sub => BinaryOperator::Sub,
		TokenKind::SubAssign => BinaryOperator::SubAssign,
		TokenKind::Mul => BinaryOperator::Mul,
		TokenKind::MulAssign => BinaryOperator::MulAssign,
		TokenKind::Div => BinaryOperator::Div,
		TokenKind::DivAssign => BinaryOperator::DivAssign,
		TokenKind::Modulo => BinaryOperator::Modulo,
		TokenKind::ModuloAssign => BinaryOperator::ModuloAssign,

		TokenKind::DoubleDot => BinaryOperator::Range,

		TokenKind::BitshiftLeft => BinaryOperator::BitshiftLeft,
		TokenKind::BitshiftLeftAssign => BinaryOperator::BitshiftLeftAssign,
		TokenKind::BitshiftRight => BinaryOperator::BitshiftRight,
		TokenKind::BitshiftRightAssign => BinaryOperator::BitshiftRightAssign,

		TokenKind::Ampersand => BinaryOperator::BitwiseAnd,
		TokenKind::AmpersandAssign => BinaryOperator::BitwiseAndAssign,
		TokenKind::Pipe => BinaryOperator::BitwiseOr,
		TokenKind::PipeAssign => BinaryOperator::BitwiseOrAssign,
		TokenKind::Caret => BinaryOperator::BitwiseXor,
		TokenKind::CaretAssign => BinaryOperator::BitwiseXorAssign,

		TokenKind::CompEqual => BinaryOperator::Equals,
		TokenKind::CompNotEqual => BinaryOperator::NotEquals,

		TokenKind::CompGreater => BinaryOperator::GreaterThan,
		TokenKind::CompGreaterEqual => BinaryOperator::GreaterThanEquals,

		TokenKind::CompLess => BinaryOperator::LessThan,
		TokenKind::CompLessEqual => BinaryOperator::LessThanEquals,

		TokenKind::Word if token.text == "and" => BinaryOperator::LogicalAnd,
		TokenKind::Word if token.text == "or" => BinaryOperator::LogicalOr,

		_ => return None,
	};

	Some(Node::new(operator, token.span))
}

pub fn parse_escape(bytes: &[u8]) -> Option<(&'static str, usize)> {
	match bytes {
		[b'\\', b'n', ..] => Some(("\n", 2)),
		[b'\\', b'r', ..] => Some(("\r", 2)),
		[b'\\', b't', ..] => Some(("\t", 2)),
		[b'\\', b'\\', ..] => Some(("\\", 2)),
		[b'\\', b'"', ..] => Some(("\"", 2)),
		[b'\\', b'0', ..] => Some(("\0", 2)),
		[b'\\', b'{', ..] => Some(("{", 2)),
		_ => None,
	}
}

pub fn parse_string_contents<'a>(string: &'a str) -> Cow<'a, str> {
	let mut allocated = String::new();
	let mut index = 0;
	let mut last = 0;

	let bytes = string.as_bytes();

	while index < bytes.len() {
		if let Some((escaped, consumed)) = parse_escape(&bytes[index..]) {
			allocated.push_str(&string[last..index]);
			allocated.push_str(escaped);
			index += consumed;
			last = index;
		} else {
			index += 1;
		}
	}

	if allocated.is_empty() {
		Cow::Borrowed(string)
	} else {
		allocated.push_str(&string[last..]);
		Cow::Owned(allocated)
	}
}
