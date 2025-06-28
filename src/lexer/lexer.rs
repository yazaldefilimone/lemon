use crate::{
	ast::Span,
	error,
	lexer::{
		token::{Token, TokenStream},
		LexResult, TokenKind,
	},
	messages::Messages,
};

#[derive(Debug)]
pub struct Lexer<'a> {
	pub file: u32,
	source: &'a str,
	chars: &'a [u8],
	pos: usize,
	line: u32,
}

impl<'a> Lexer<'a> {
	pub fn new(file: u32, source: &'a str) -> Self {
		Self { file, source, chars: source.as_bytes(), pos: 0, line: 0 }
	}

	pub fn new_substring(
		file: u32,
		source: &'a str,
		start_pos: usize,
		length: usize,
		start_line: u32,
	) -> Self {
		let source = &source[..start_pos + length];

		Self { file, source, chars: source.as_bytes(), pos: start_pos, line: start_line }
	}

	pub fn tokenize(&mut self, messages: &mut Messages) -> TokenStream<'a> {
		let mut tokens = Vec::new();
		while let Ok(token) = self.next_token(messages) {
			tokens.push(token);
		}
		TokenStream::new(0, tokens, self.source)
	}

	fn at_end(&self) -> bool {
		self.pos >= self.source.len()
	}

	fn current(&self) -> Option<u8> {
		if self.at_end() {
			None
		} else {
			Some(self.chars[self.pos])
		}
	}

	fn peek(&self) -> Option<u8> {
		if self.pos + 1 >= self.source.len() {
			None
		} else {
			Some(self.chars[self.pos + 1])
		}
	}

	fn consume(&mut self) -> Option<u8> {
		if self.at_end() {
			None
		} else {
			let ch = self.chars[self.pos];
			self.pos += 1;
			Some(ch)
		}
	}

	fn make_token(&self, text: &'a str, kind: TokenKind, start: usize) -> Token<'a> {
		Token::new(text, kind, start, self.pos, self.file, self.line)
	}

	fn next_token(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let before_whitespace = self.pos;

		if let Some(newline_token) = self.skip_whitespace()? {
			return Ok(newline_token);
		}

		if self.at_end() {
			return Err(());
		}

		match self.current().unwrap() {
			b'.' => self.scan_period(),
			b'(' => Ok(self.single_char(TokenKind::OpenParen)),
			b')' => Ok(self.single_char(TokenKind::CloseParen)),
			b':' => self.scan_colon(),
			b'{' => Ok(self.single_char(TokenKind::OpenBrace)),
			b'}' => Ok(self.single_char(TokenKind::CloseBrace)),
			b',' => Ok(self.single_char(TokenKind::Comma)),
			b'=' => self.scan_equals(),
			b'[' => Ok(self.single_char(TokenKind::OpenBracket)),
			b']' => Ok(self.single_char(TokenKind::CloseBracket)),
			b'+' => self.scan_plus(),
			b'-' => self.scan_minus(),
			b'*' => self.scan_star(),
			b'/' => self.scan_slash(messages),
			b'%' => self.scan_percent(),
			b'<' => self.scan_less(before_whitespace),
			b'>' => self.scan_greater(before_whitespace),
			b'!' => self.scan_bang(),
			b'&' => self.scan_ampersand(),
			b'|' => self.scan_pipe(),
			b'^' => self.scan_caret(),
			b'~' => Ok(self.single_char(TokenKind::Tilde)),
			b'#' => Ok(self.single_char(TokenKind::PoundSign)),
			b'\'' => self.scan_char_literal(messages),
			b'"' => self.scan_string_literal(messages),
			b'b' if self.peek() == Some(b'\'') => self.scan_byte_char_literal(messages),
			b'f' if self.peek() == Some(b'"') => self.scan_format_string(messages),
			_ => self.scan_identifier_or_number(),
		}
	}

	fn single_char(&mut self, kind: TokenKind) -> Token<'a> {
		let start = self.pos;
		self.consume();
		self.make_token(&self.source[start..self.pos], kind, start)
	}

	fn double_char(&mut self, kind: TokenKind) -> Token<'a> {
		let start = self.pos;
		self.consume();
		self.consume();
		self.make_token(&self.source[start..self.pos], kind, start)
	}

	fn triple_char(&mut self, kind: TokenKind) -> Token<'a> {
		let start = self.pos;
		self.consume();
		self.consume();
		self.consume();
		self.make_token(&self.source[start..self.pos], kind, start)
	}

	fn scan_period(&mut self) -> LexResult<Token<'a>> {
		let start = self.pos;

		if self.chars[self.pos..].starts_with(b"...") {
			self.pos += 3;
			Ok(self.make_token("...", TokenKind::TriplePeriod, start))
		} else if self.chars[self.pos..].starts_with(b"..") {
			self.pos += 2;
			Ok(self.make_token("..", TokenKind::DoublePeriod, start))
		} else {
			Ok(self.single_char(TokenKind::Period))
		}
	}

	fn scan_colon(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"::") {
			Ok(self.double_char(TokenKind::DoubleColon))
		} else {
			Ok(self.single_char(TokenKind::Colon))
		}
	}

	fn scan_equals(&mut self) -> LexResult<Token<'a>> {
		match self.chars[self.pos..] {
			[b'=', b'=', ..] => Ok(self.double_char(TokenKind::CompEqual)),
			[b'=', b'>', ..] => Ok(self.double_char(TokenKind::FatArrow)),
			_ => Ok(self.single_char(TokenKind::Equal)),
		}
	}

	fn scan_plus(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"+=") {
			Ok(self.double_char(TokenKind::AddAssign))
		} else {
			Ok(self.single_char(TokenKind::Add))
		}
	}

	fn scan_minus(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"-=") {
			Ok(self.double_char(TokenKind::SubAssign))
		} else if self.is_negative_number() {
			self.scan_identifier_or_number()
		} else {
			Ok(self.single_char(TokenKind::Sub))
		}
	}

	fn is_negative_number(&self) -> bool {
		self.pos + 1 < self.chars.len() && self.chars[self.pos + 1].is_ascii_digit()
	}

	fn scan_star(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"*=") {
			Ok(self.double_char(TokenKind::MulAssign))
		} else {
			Ok(self.single_char(TokenKind::Mul))
		}
	}

	fn scan_slash(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		match self.chars[self.pos..] {
			[b'/', b'/', ..] => {
				self.skip_line_comment();
				self.next_token(messages)
			}
			[b'/', b'*', ..] => {
				self.skip_block_comment(messages)?;
				self.next_token(messages)
			}
			[b'/', b'=', ..] => Ok(self.double_char(TokenKind::DivAssign)),
			_ => Ok(self.single_char(TokenKind::Div)),
		}
	}

	fn skip_line_comment(&mut self) {
		self.pos += 2;
		while !self.at_end() && self.chars[self.pos] != b'\n' {
			self.pos += 1;
		}
	}

	fn skip_block_comment(&mut self, messages: &mut Messages) -> LexResult<()> {
		let mut depth = 1;
		self.pos += 2;

		while depth > 0 {
			if self.at_end() {
				let error = error!("unterminated block comment");
				let span = Span {
					start: self.source.len().saturating_sub(1),
					end: self.source.len(),
					file: self.file,
					line: self.line,
				};
				messages.message(error.span(span));
				return Err(());
			}

			if self.chars[self.pos] == b'\n' {
				self.line += 1;
			} else if self.chars[self.pos..].starts_with(b"/*") {
				depth += 1;
				self.pos += 1; // skip extra to avoid double counting
			} else if self.chars[self.pos..].starts_with(b"*/") {
				depth -= 1;
				self.pos += 1; // skip extra to avoid double counting
			}

			self.pos += 1;
		}

		Ok(())
	}

	fn scan_percent(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"%=") {
			Ok(self.double_char(TokenKind::ModuloAssign))
		} else {
			Ok(self.single_char(TokenKind::Modulo))
		}
	}

	fn scan_less(&mut self, before_whitespace: usize) -> LexResult<Token<'a>> {
		match self.chars[self.pos..] {
			[b'<', b'<', b'=', ..] => Ok(self.triple_char(TokenKind::BitshiftLeftAssign)),
			[b'<', b'<', ..] => Ok(self.double_char(TokenKind::BitshiftLeft)),
			[b'<', b'=', ..] => Ok(self.double_char(TokenKind::CompLessEqual)),
			_ => {
				if before_whitespace < self.pos {
					Ok(self.single_char(TokenKind::CompLess))
				} else {
					Ok(self.single_char(TokenKind::OpenGeneric))
				}
			}
		}
	}

	fn scan_greater(&mut self, before_whitespace: usize) -> LexResult<Token<'a>> {
		match self.chars[self.pos..] {
			[b'>', b'>', b'=', ..] => Ok(self.triple_char(TokenKind::BitshiftRightAssign)),
			[b'>', b'>', ..] if before_whitespace < self.pos => {
				Ok(self.double_char(TokenKind::BitshiftRight))
			}
			[b'>', b'=', ..] => Ok(self.double_char(TokenKind::CompGreaterEqual)),
			_ => Ok(self.single_char(TokenKind::CompGreater)),
		}
	}

	fn scan_bang(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"!=") {
			Ok(self.double_char(TokenKind::CompNotEqual))
		} else {
			Ok(self.single_char(TokenKind::Exclamation))
		}
	}

	fn scan_ampersand(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"&=") {
			Ok(self.double_char(TokenKind::AmpersandAssign))
		} else {
			Ok(self.single_char(TokenKind::Ampersand))
		}
	}

	fn scan_pipe(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"|=") {
			Ok(self.double_char(TokenKind::PipeAssign))
		} else {
			Ok(self.single_char(TokenKind::Pipe))
		}
	}

	fn scan_caret(&mut self) -> LexResult<Token<'a>> {
		if self.chars[self.pos..].starts_with(b"^=") {
			Ok(self.double_char(TokenKind::CaretAssign))
		} else {
			Ok(self.single_char(TokenKind::Caret))
		}
	}

	fn scan_char_literal(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.pos;
		self.consume(); // skip opening quote

		if self.at_end() {
			return self.eof_error(messages);
		}

		if self.current() == Some(b'\\') {
			self.consume(); // skip backslash
			self.consume(); // skip escaped character
		} else {
			self.consume_codepoint(messages)?;
		}

		self.expect_char(messages, b'\'')?;
		self.consume(); // skip closing quote

		let content = &self.source[start + 1..self.pos - 1];
		Ok(self.make_token(content, TokenKind::Codepoint, start))
	}

	fn scan_byte_char_literal(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.pos;
		self.consume(); // skip 'b'
		self.consume(); // skip opening quote

		if self.at_end() {
			return self.eof_error(messages);
		}

		if self.current() == Some(b'\\') {
			self.consume(); // skip backslash
			self.consume(); // skip escaped character
		} else {
			let before_advance = self.pos;
			self.consume_codepoint(messages)?;
			if self.pos - before_advance > 1 {
				let error = error!("byte codepoint literal may not contain a multi-byte codepoint");
				let span =
					Span { start: before_advance, end: before_advance + 1, file: self.file, line: self.line };
				messages.message(error.span(span));
			}
		}

		self.expect_char(messages, b'\'')?;
		self.consume(); // skip closing quote

		let content = &self.source[start + 2..self.pos - 1];
		Ok(self.make_token(content, TokenKind::ByteCodepoint, start))
	}

	fn scan_string_literal(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.pos;
		let start_line = self.line;
		self.consume(); // skip opening quote

		while !self.at_end() {
			match self.current().unwrap() {
				b'"' => break,
				b'\n' => {
					self.line += 1;
					self.consume();
				}
				b'\\' => {
					self.consume(); // skip backslash
					if !self.at_end() {
						self.consume(); // skip escaped character
					}
				}
				_ => {
					self.consume();
				}
			}
		}

		if self.at_end() {
			return self.eof_error(messages);
		}

		let content = &self.source[start + 1..self.pos];
		self.consume(); // skip closing quote

		Ok(Token::new(content, TokenKind::String, start, self.pos, self.file, start_line))
	}

	fn scan_format_string(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		let start = self.pos;
		let start_line = self.line;
		self.consume(); // skip 'f'
		self.consume(); // skip opening quote

		while !self.at_end() {
			match self.current().unwrap() {
				b'"' => break,
				b'\n' => {
					self.line += 1;
					self.consume();
				}
				b'\\' => {
					self.consume(); // skip backslash
					if !self.at_end() {
						self.consume(); // skip escaped character
					}
				}
				_ => {
					self.consume();
				}
			}
		}

		if self.at_end() {
			return self.eof_error(messages);
		}

		let content = &self.source[start + 2..self.pos];
		self.consume(); // skip closing quote

		Ok(Token::new(content, TokenKind::FormatString, start, self.pos, self.file, start_line))
	}

	fn scan_identifier_or_number(&mut self) -> LexResult<Token<'a>> {
		let start = self.pos;
		let is_negative = self.current() == Some(b'-');

		if is_negative {
			self.consume();
		}

		let is_number = self.current().map_or(false, |ch| ch.is_ascii_digit());

		// Consume word/number characters
		while !self.at_end() {
			let ch = self.current().unwrap();
			if self.is_delimiter(ch) {
				// Special case for decimal numbers
				if is_number && ch == b'.' && !self.at_end() {
					if let Some(next_ch) = self.peek() {
						if next_ch.is_ascii_digit() {
							self.consume(); // skip the '.'

							// consume decimal digits
							while !self.at_end() && self.current().unwrap().is_ascii_digit() {
								self.consume();
							}
							break;
						}
					}
				}
				break;
			}
			self.consume();
		}

		let text = &self.source[start..self.pos];
		let kind = if is_number { TokenKind::Number } else { TokenKind::Word };

		Ok(self.make_token(text, kind, start))
	}

	fn is_delimiter(&self, ch: u8) -> bool {
		matches!(
			ch,
			b' '
				| b'\t'
				| b'\n'
				| b'\r'
				| b'('
				| b')'
				| b'{'
				| b'}'
				| b'['
				| b']'
				| b'+'
				| b'-'
				| b'*'
				| b'/'
				| b'='
				| b'>'
				| b'<'
				| b':'
				| b';'
				| b'.'
				| b','
				| b'\''
				| b'"'
				| b'!'
				| b'&'
				| b'|'
		)
	}

	fn consume_codepoint(&mut self, messages: &mut Messages) -> LexResult<()> {
		if self.at_end() {
			return self.eof_error(messages);
		}

		let mut chars = self.source[self.pos..].char_indices();
		chars.next().ok_or(())?;
		if let Some((next_offset, _)) = chars.next() {
			self.pos += next_offset;
		} else {
			self.pos = self.source.len();
		}

		Ok(())
	}

	fn expect_char(&self, messages: &mut Messages, expected: u8) -> LexResult<()> {
		if self.at_end() {
			return self.eof_error(messages);
		}

		let found = self.chars[self.pos];
		if found != expected {
			let err = error!("expected {:?} but found {:?}", expected as char, found as char);
			messages.message(err.span(Span {
				start: self.pos,
				end: self.pos + 1,
				file: self.file,
				line: self.line,
			}));
			return Err(());
		}

		Ok(())
	}

	fn eof_error<T>(&self, messages: &mut Messages) -> LexResult<T> {
		let error = error!("unexpected end of file");
		let span = Span {
			start: self.source.len().saturating_sub(1),
			end: self.source.len(),
			file: self.file,
			line: self.line,
		};
		messages.message(error.span(span));
		Err(())
	}

	fn skip_whitespace(&mut self) -> LexResult<Option<Token<'a>>> {
		while !self.at_end() {
			match self.current().unwrap() {
				b'\n' => {
					let start = self.pos;
					let current_line = self.line;
					self.consume();
					self.line += 1;
					return Ok(Some(Token::new(
						"\n",
						TokenKind::Newline,
						start,
						self.pos,
						self.file,
						current_line,
					)));
				}
				b' ' | b'\t' | b'\r' => {
					self.consume();
				}
				_ => break,
			}
		}
		Ok(None)
	}
}
