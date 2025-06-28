use std::fmt::Display;

use crate::{
	ast::Span,
	error,
	lexer::{LexResult, TokenKind},
	messages::{Messages, ParseResult},
};

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
	pub text: &'a str,
	pub kind: TokenKind,
	pub span: Span,
}

impl<'a> Token<'a> {
	pub fn new(
		text: &'a str,
		kind: TokenKind,
		start: usize,
		end: usize,
		file: u32,
		line: u32,
	) -> Self {
		Token { text, kind, span: Span { start, end, file: file, line: line } }
	}
}

impl Display for Token<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} - kind: {}, text: {}", self.span, self.kind, self.text)
	}
}

pub struct TokenStream<'a> {
	pos: usize,
	pub tokens: Vec<Token<'a>>,
	source: &'a str,
}

impl<'a> TokenStream<'a> {
	pub fn new(pos: usize, tokens: Vec<Token<'a>>, source: &'a str) -> Self {
		Self { pos, tokens, source }
	}

	pub fn into_tokens(self) -> Vec<Token<'a>> {
		self.tokens
	}

	pub fn source(&self) -> &'a str {
		self.source
	}

	pub fn is_at_end(&self) -> bool {
		self.pos >= self.tokens.len()
	}

	pub fn peek(&self) -> LexResult<Token<'a>> {
		if self.is_at_end() {
			return Err(());
		}
		Ok(self.tokens[self.pos])
	}

	pub fn peek_ahead(&self, offset: usize) -> LexResult<Token<'a>> {
		if self.pos + offset >= self.tokens.len() {
			return Err(());
		}
		Ok(self.tokens[self.pos + offset])
	}

	pub fn peek_kind(&self) -> LexResult<TokenKind> {
		self.peek().map(|token| token.kind)
	}

	pub fn peek_kind_ahead(&self, offset: usize) -> LexResult<TokenKind> {
		self.peek_ahead(offset).map(|token| token.kind)
	}

	pub fn peek_skip_newlines(&self) -> LexResult<Token<'a>> {
		if self.is_at_end() {
			return Err(());
		}

		if self.tokens[self.pos].kind == TokenKind::Newline {
			if self.pos + 1 >= self.tokens.len() {
				return Err(());
			}
			return Ok(self.tokens[self.pos + 1]);
		}

		Ok(self.tokens[self.pos])
	}

	pub fn advance(&mut self, messages: &mut Messages) -> LexResult<Token<'a>> {
		if self.is_at_end() {
			let error = error!("Ran out of tokens to parse");
			let span = self.tokens.last().map(|t| t.span);
			messages.message(error.span_if_some(span));
			return Err(());
		}

		let token = self.tokens[self.pos];
		self.pos += 1;
		Ok(token)
	}

	pub fn advance_expected(
		&mut self,
		messages: &mut Messages,
		expected: TokenKind,
	) -> LexResult<Token<'a>> {
		if self.is_at_end() {
			let error = error!("ran out of tokens to parse, expected {expected}");
			let span = self.tokens.last().map(|t| t.span);
			messages.message(error.span_if_some(span));
			return Err(());
		}

		let token = self.tokens[self.pos];
		self.pos += 1;
		Ok(token)
	}

	pub fn skip_newlines(&mut self) {
		while self.peek_kind() == Ok(TokenKind::Newline) {
			self.pos += 1;
		}
	}

	#[inline]
	pub fn next(&mut self, messages: &mut Messages) -> ParseResult<Token<'a>> {
		if self.pos >= self.tokens.len() {
			let error = error!("ran out of tokens to parse");
			let span = self.tokens.last().map(|t| t.span);
			messages.message(error.span_if_some(span));
			return Err(());
		}

		let pos = self.pos;
		self.pos += 1;
		Ok(self.tokens[pos])
	}

	pub fn expect_peek(&self, messages: &mut Messages, expected: TokenKind) -> LexResult<Token<'a>> {
		let token = self.peek()?;
		if token.kind == expected {
			return Ok(token);
		}

		let message = error!("expected {expected} but found {:?}", token.text);
		messages.message(message.span(token.span));
		Err(())
	}

	pub fn expect(&mut self, messages: &mut Messages, expected: TokenKind) -> LexResult<Token<'a>> {
		let token = self.advance_expected(messages, expected)?;
		if token.kind == expected {
			return Ok(token);
		}

		let message = error!("expected {expected} but found {:?}", token.text);
		messages.message(message.span(token.span));
		Err(())
	}

	pub fn expect_word(&mut self, messages: &mut Messages, expected: &str) -> LexResult<Token<'a>> {
		let token = self.expect(messages, TokenKind::Word)?;
		if token.text == expected {
			return Ok(token);
		}

		let message = error!("expected word {expected:?} but found word {:?}", token.text);
		messages.message(message.span(token.span));
		Err(())
	}
}
