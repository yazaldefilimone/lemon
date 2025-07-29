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
	pub chars: &'a [u8],
	pub position: usize,
	pub line: u32,
}

impl<'a> Lexer<'a> {
	pub fn new(file: u32, source: &'a str) -> Self {
		Self { file, source, chars: source.as_bytes(), position: 0, line: 0 }
	}

	pub fn new_substring(
		file: u32,
		source: &'a str,
		position: usize,
		length: usize,
		line: u32,
	) -> Self {
		let chars = &source[..position + length].as_bytes();
		Self { file, source, chars, position, line }
	}

	pub fn reader(&self, messages: &mut Messages) -> TokenReader {
		todo!("implement lexer")
		// TokenReader {}
	}

	pub fn read_token(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let pre_whitespace = self.position;

		if let Some(newline_token) = self.read_whitespace()? {
			return Ok(newline_token);
		};

		if self.position >= self.chars.len() {
			return Err(());
		}
		match self.chars[self.position..] {
			[b'.'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Dot, span);
				return Ok(operator_token);
			}
			[b':'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Colon, span);
				return Ok(operator_token);
			}
			[b';'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Semicolon, span);
				return Ok(operator_token);
			}
			[b','] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Comma, span);
				return Ok(operator_token);
			}
			[b'('] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::OpenParen, span);
				return Ok(operator_token);
			}
			[b')'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::CloseParen, span);
				return Ok(operator_token);
			}
			[b'{'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::OpenBrace, span);
				return Ok(operator_token);
			}
			[b'}'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::CloseBrace, span);
				return Ok(operator_token);
			}
			[b'['] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::OpenBracket, span);
				return Ok(operator_token);
			}
			[b']'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::CloseBracket, span);
				return Ok(operator_token);
			}
			[b'\''] => {
				let span = self.advance_and_make_span(1, self.line);
				let string_token = Token::new(self.source, TokenKind::String, span);
				return Ok(string_token);
			}
			[b'"'] => {
				let span = self.advance_and_make_span(1, self.line);
				let string_token = Token::new(self.source, TokenKind::String, span);
				return Ok(string_token);
			}
			[b'+'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Add, span);
				return Ok(operator_token);
			}
			[b'-'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Sub, span);
				return Ok(operator_token);
			}
			[b'*'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Mul, span);
				return Ok(operator_token);
			}
			[b'/'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Div, span);
				return Ok(operator_token);
			}
			[b'%'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Modulo, span);
				return Ok(operator_token);
			}
			[b'!'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Exclamation, span);
				return Ok(operator_token);
			}
			[b'='] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Equal, span);
				return Ok(operator_token);
			}
			[b'<'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::CompLess, span);
				return Ok(operator_token);
			}
			[b'>'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::CompGreater, span);
				return Ok(operator_token);
			}
			[b'&'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Ampersand, span);
				return Ok(operator_token);
			}
			[b'|'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Pipe, span);
				return Ok(operator_token);
			}
			[b'^'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Caret, span);
				return Ok(operator_token);
			}
			[b'~'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::Tilde, span);
				return Ok(operator_token);
			}
			[b'#'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::PoundSign, span);
				return Ok(operator_token);
			}

			// douple tokens

			// TokenKind::Equal => "'='",

			// TokenKind::CompGreater => "'>'",
			// TokenKind::CompLess => "'<'",

			// TokenKind::Ampersand => "'&'",
			// TokenKind::Pipe => "'|'",
			// TokenKind::Caret => "'^'",
			// TokenKind::Tilde => "'~'",

			// TokenKind::Colon => "':'",
			// TokenKind::Dot => "'.'",
			// TokenKind::DoubleDot => "'..'",
			// TokenKind::TripleDot => "'...'",
			// TokenKind::Comma => "','",
			// TokenKind::Semicolon => "';'",
			// TokenKind::PoundSign => "'#'",
			// TokenKind::FatArrow => "'=>'",

			// TokenKind::Exclamation => "'!'",

			//
			//
			//
			[b'.', b'.'] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::DoubleDot, span);
				return Ok(operator_token);
			}
			[b'=', b'>'] => {
				let span = self.advance_and_make_span(1, self.line);
				let operator_token = Token::new(self.source, TokenKind::FatArrow, span);
				return Ok(operator_token);
			}

			[b'+', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::AddAssign, span);
				return Ok(operator_token);
			}

			[b'-', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::SubAssign, span);
				return Ok(operator_token);
			}

			[b'*', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::MulAssign, span);
				return Ok(operator_token);
			}

			[b'/', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::DivAssign, span);
				return Ok(operator_token);
			}

			[b'%', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::ModuloAssign, span);
				return Ok(operator_token);
			}

			[b'<', b'<'] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::BitshiftLeft, span);
				return Ok(operator_token);
			}

			[b'>', b'>'] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::BitshiftRight, span);
				return Ok(operator_token);
			}

			[b'<', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::CompLessEqual, span);
				return Ok(operator_token);
			}

			[b'>', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::CompGreaterEqual, span);
				return Ok(operator_token);
			}

			[b'&', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::AmpersandAssign, span);
				return Ok(operator_token);
			}

			[b'|', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::PipeAssign, span);
				return Ok(operator_token);
			}

			[b'^', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::CaretAssign, span);
				return Ok(operator_token);
			}

			[b'=', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::CompEqual, span);
				return Ok(operator_token);
			}
			[b'!', b'='] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::CompNotEqual, span);
				return Ok(operator_token);
			}
			[b':', b':'] => {
				let span = self.advance_and_make_span(2, self.line);
				let operator_token = Token::new(self.source, TokenKind::DoubleColon, span);
				return Ok(operator_token);
			}

			// strings
			[b'\"', ..] => {
				let start = self.position;
				let line = self.line;

				loop {
					self.advance(1);
					self.check_end_of_file(messages)?;

					if self.chars[self.position] == b'\n' {
						self.line += 1;
					} else if matches!(self.chars[self.position..], [b'\\', b'\\', ..] | [b'\\', b'"', ..]) {
						self.advance(1);
						continue;
					}

					if self.chars[self.position] == b'\"' {
						let span = self.advance_and_make_span(self.position - start, line);
						let string_token = Token::new(self.source, TokenKind::String, span);
						return Ok(string_token);
					}
				}
			}

			// comment
			[b'/', b'*', ..] => {
				let mut open_count = 1;
				self.advance(1); // consume '/' todo: review

				loop {
					self.advance(1); // first time consume '*'
					self.check_end_of_file(messages)?;

					if self.chars[self.position] == b'\n' {
						self.line += 1;
						// self.line_starts.push(self.position + 1);
						continue;
					}

					if matches!(self.chars[self.position..], [b'/', b'*', ..]) {
						open_count += 1;
						continue;
					}

					if matches!(self.chars[self.position..], [b'*', b'/', ..]) {
						if open_count <= 1 {
							self.advance(2);
							break;
						}
						open_count -= 1;
					}
				}
				return self.read_token(messages);
			}

			// triple tokens
			[b'.', b'.', b'.', ..] => {
				let span = self.advance_and_make_span(3, self.line);
				let operator_token = Token::new(self.source, TokenKind::TripleDot, span);
				return Ok(operator_token);
			}

			[b'<', b'<', b'=', ..] => {
				let span = self.advance_and_make_span(3, self.line);
				let operator_token = Token::new(self.source, TokenKind::BitshiftLeftAssign, span);
				return Ok(operator_token);
			}

			[b'>', b'>', b'=', ..] => {
				let span = self.advance_and_make_span(3, self.line);
				let operator_token = Token::new(self.source, TokenKind::BitshiftRightAssign, span);
				return Ok(operator_token);
			}

			_ => {
				return Err(());
			}
		}
	}

	// private functions
	// =================
	//
	fn read_whitespace(&mut self) -> LexResult<Option<Token<'a>>> {
		while self.position < self.chars.len() {
			let byte = self.chars[self.position];
			if byte == b'\n' {
				let index = self.position;
				self.position += 1;

				let line = self.line;
				self.line += 1;

				let span = self.advance_and_make_span(index, line);
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
}
