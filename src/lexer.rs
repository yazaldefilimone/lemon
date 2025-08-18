#![allow(dead_code)]
use crate::{
	error,
	messages::Messages,
	span::Span,
	token::{Token, TokenKind},
	token_reader::TokenReader,
};

type LexResult<T> = std::result::Result<T, ()>;

pub struct Lexer<'a> {
	pub file: u32,
	pub source: &'a str,
	pub chars_bytes: &'a [u8],
	pub position: usize,
	pub line: u32,
}

impl<'a> Lexer<'a> {
	pub fn new(file: u32, source: &'a str) -> Self {
		Self { file, source, chars_bytes: source.as_bytes(), position: 0, line: 1 }
	}

	pub fn new_substring(
		file: u32,
		source: &'a str,
		position: usize,
		length: usize,
		line: u32,
	) -> Self {
		let chars_bytes = &source[..position + length].as_bytes();
		Self { file, source, chars_bytes, position, line }
	}

	pub fn reader(&mut self, messages: &mut Messages) -> TokenReader {
		let mut tokens = Vec::new();
		while let Ok(token) = self.read_token(messages) {
			if token.kind == TokenKind::Newline {
				// self.line_starts.push(token.span.end);
			}
			tokens.push(token);
		}
		TokenReader::new(tokens, self.source)
	}

	pub fn read_token(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		// let pre_whitespace = self.position;

		if let Some(newline_token) = self.read_whitespace()? {
			return Ok(newline_token);
		};

		if self.position >= self.chars_bytes.len() {
			return Err(());
		}

		match self.chars_bytes[self.position..] {
			// triple tokens
			[b'.', b'.', b'.', ..] => {
				let span = self.advance_and_make_span(3, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::TripleDot, span);
				return Ok(operator_token);
			}

			[b'<', b'<', b'=', ..] => {
				let span = self.advance_and_make_span(3, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::BitshiftLeftAssign, span);
				return Ok(operator_token);
			}

			[b'>', b'>', b'=', ..] => {
				let span = self.advance_and_make_span(3, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::BitshiftRightAssign, span);
				return Ok(operator_token);
			}

			// douple tokens
			//
			[b'.', b'.', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::DoubleDot, span);
				return Ok(operator_token);
			}
			[b'=', b'>', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::FatArrow, span);
				return Ok(operator_token);
			}

			[b'+', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::AddAssign, span);
				return Ok(operator_token);
			}

			[b'-', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::SubAssign, span);
				return Ok(operator_token);
			}

			[b'*', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::MulAssign, span);
				return Ok(operator_token);
			}

			[b'/', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::DivAssign, span);
				return Ok(operator_token);
			}

			[b'%', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::ModuloAssign, span);
				return Ok(operator_token);
			}

			[b'<', b'<', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::BitshiftLeft, span);
				return Ok(operator_token);
			}

			[b'>', b'>', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::BitshiftRight, span);
				return Ok(operator_token);
			}

			[b'<', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CompLessEqual, span);
				return Ok(operator_token);
			}

			[b'>', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CompGreaterEqual, span);
				return Ok(operator_token);
			}

			[b'&', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::AmpersandAssign, span);
				return Ok(operator_token);
			}

			[b'|', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::PipeAssign, span);
				return Ok(operator_token);
			}

			[b'^', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CaretAssign, span);
				return Ok(operator_token);
			}

			[b'=', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CompEqual, span);
				return Ok(operator_token);
			}
			[b'!', b'=', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CompNotEqual, span);
				return Ok(operator_token);
			}
			[b':', b':', ..] => {
				let span = self.advance_and_make_span(2, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::DoubleColon, span);
				return Ok(operator_token);
			}

			// comment
			[b'/', b'*', ..] => {
				let mut open_count = 1;
				self.advance(1); // consume '/' todo: review

				loop {
					self.advance(1); // first time consume '*'
					self.check_end_of_file(messages)?;

					if self.chars_bytes[self.position] == b'\n' {
						self.line += 1;
						// self.line_starts.push(self.position + 1);
						continue;
					}

					if matches!(self.chars_bytes[self.position..], [b'/', b'*', ..]) {
						open_count += 1;
						continue;
					}

					if matches!(self.chars_bytes[self.position..], [b'*', b'/', ..]) {
						if open_count <= 1 {
							self.advance(2);
							break;
						}
						open_count -= 1;
					}
				}
				return self.read_token(messages);
			}

			[b'.', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Dot, span);
				return Ok(operator_token);
			}
			[b':', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Colon, span);
				return Ok(operator_token);
			}
			[b';', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Semicolon, span);
				return Ok(operator_token);
			}
			[b',', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Comma, span);
				return Ok(operator_token);
			}
			[b'(', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::OpenParen, span);
				return Ok(operator_token);
			}
			[b')', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CloseParen, span);
				return Ok(operator_token);
			}
			[b'{', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::OpenBrace, span);
				return Ok(operator_token);
			}
			[b'}', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CloseBrace, span);
				return Ok(operator_token);
			}
			[b'[', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::OpenBracket, span);
				return Ok(operator_token);
			}
			[b']', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CloseBracket, span);
				return Ok(operator_token);
			}
			[b'\'', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let string_token = Token::new(text, TokenKind::String, span);
				return Ok(string_token);
			}
			[b'+', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Add, span);
				return Ok(operator_token);
			}
			[b'-', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Sub, span);
				return Ok(operator_token);
			}
			[b'*', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Mul, span);
				return Ok(operator_token);
			}
			[b'/', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Div, span);
				return Ok(operator_token);
			}
			[b'%', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Modulo, span);
				return Ok(operator_token);
			}
			[b'!', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Exclamation, span);
				return Ok(operator_token);
			}
			[b'=', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Equal, span);
				return Ok(operator_token);
			}
			[b'<', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CompLess, span);
				return Ok(operator_token);
			}
			[b'>', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::CompGreater, span);
				return Ok(operator_token);
			}
			[b'&', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Ampersand, span);
				return Ok(operator_token);
			}
			[b'|', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Pipe, span);
				return Ok(operator_token);
			}
			[b'^', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Caret, span);
				return Ok(operator_token);
			}
			[b'~', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::Tilde, span);
				return Ok(operator_token);
			}
			[b'#', ..] => {
				let span = self.advance_and_make_span(1, self.line);
				let text = &self.source[span.start..span.end];
				let operator_token = Token::new(text, TokenKind::PoundSign, span);
				return Ok(operator_token);
			}
			_ => self.read_token_generic(messages),
		}
	}

	// private functions
	// =================
	//
	fn read_token_generic(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		if self.starts_with(&[b'f', b'"']) {
			return self.read_format_string(messages);
		} else if self.starts_with(&[b'"']) {
			return self.read_string(messages);
		}
		return self.read_number_or_word(messages);
	}

	fn read_number_or_word(&mut self, _messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.position;

		let is_number = self.match_number_prefix();
		if is_number {
			self.read_number_body();
		} else {
			self.read_word_body();
		}

		let kind = if is_number { TokenKind::Number } else { TokenKind::Word };

		let span = self.make_span(start, self.position, self.line);
		let token = Token::new(&self.source[span.start..span.end], kind, span);
		Ok(token)
	}

	fn read_string(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.position;
		self.position += 1; // skip `"`

		self.read_until_closing_quote(messages)?;
		let span = self.make_span(start, self.position, self.line);
		let text = &self.source[span.start..span.end];
		Ok(Token::new(text, TokenKind::String, span))
	}
	fn read_format_string(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.position;
		self.position += 2; // skip `f"`

		self.read_until_closing_quote(messages)?;

		let span = self.make_span(start + 2, self.position, self.line);

		let text = &self.source[span.start..span.end];

		Ok(Token::new(text, TokenKind::FormatString, span))
	}

	fn read_number_body(&mut self) {
		while self.peek(0).map_or(false, |c| c.is_ascii_digit()) {
			self.position += 1;
		}

		if self.peek(0) == Some(b'.') && self.peek(1).map_or(false, |c| c.is_ascii_digit()) {
			self.position += 1; // consume '.'
			while self.peek(0).map_or(false, |c| c.is_ascii_digit()) {
				self.position += 1;
			}
		}
	}
	fn read_word_body(&mut self) {
		while self.peek(0).map_or(false, |byte| byte.is_ascii_alphanumeric() || byte == b'_') {
			self.position += 1;
		}
	}
	fn match_number_prefix(&mut self) -> bool {
		if self.chars_bytes[self.position] == b'-' {
			if self.peek(1).map_or(false, |c| c.is_ascii_digit()) {
				self.position += 1;
				return true;
			}
			return false;
		}
		self.chars_bytes[self.position].is_ascii_digit()
	}

	fn read_whitespace(&mut self) -> LexResult<Option<Token<'a>>> {
		while !self.is_end_of_file() {
			let byte = self.chars_bytes[self.position];
			if byte == b'\n' {
				let line = self.line;
				self.line += 1;
				let span = self.advance_and_make_span(1, line);
				let newline_token = Token::new("\n", TokenKind::Newline, span);
				return Ok(Some(newline_token));
			}

			if matches!(byte, b' ' | b'\t' | b'\r') {
				self.position += 1;
				continue;
			}

			break;
		}

		Ok(None)
	}
	fn read_until_closing_quote(&mut self, messages: &mut Messages) -> LexResult<()> {
		while self.position < self.chars_bytes.len() {
			self.position += 1;
			self.check_end_of_file(messages)?;

			match self.chars_bytes[self.position] {
				b'\n' => {
					self.line += 1;
					// self.line_starts.push(self.position + 1);
				}
				b'\"' => {
					self.position += 1; // skip `"`
					break;
				}
				b'\\' if self.peek(1) == Some(b'"') || self.peek(1) == Some(b'\\') => {
					self.position += 1; // skip escaped char
				}
				_ => {}
			}
		}
		Ok(())
	}

	fn check_end_of_file(&self, messages: &mut Messages) -> LexResult<()> {
		if self.position >= self.source.len() {
			let error = error!("unexpected end of file");

			let start = self.source.len().saturating_sub(1);
			let end = self.source.len().saturating_sub(1);
			let span = Span::new(start, end, self.file, self.line);

			messages.message(error.with_span(span));

			Err(())
		} else {
			Ok(())
		}
	}

	// utils functions
	// ===============
	//
	#[inline(always)]
	fn make_span(&self, start: usize, end: usize, line: u32) -> Span {
		Span::new(start, end, self.file, line)
	}

	#[inline(always)]
	fn advance_and_make_span(&mut self, steps: usize, line: u32) -> Span {
		let start = self.position;
		self.advance(steps);
		let end = self.position;
		Span::new(start, end, self.file, line)
	}

	#[inline(always)]
	fn advance(&mut self, steps: usize) {
		self.position += steps;
	}

	fn is_end_of_file(&self) -> bool {
		self.position >= self.source.len()
	}
	fn starts_with(&self, matcher: &[u8]) -> bool {
		let peeked = self.peek(0);
		matcher.iter().all(|byte| peeked == Some(*byte))
	}

	fn peek(&self, ahead: usize) -> Option<u8> {
		self.chars_bytes.get(self.position + ahead).copied()
	}
}
