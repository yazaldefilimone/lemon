#![allow(dead_code)]

use crate::error;
use crate::messages::Messages;
use crate::token::{Token, TokenKind};

type Result<T> = std::result::Result<T, ()>;

pub struct TokenReader<'a> {
	position: usize,
	tokens: Vec<Token<'a>>,
	source: &'a str,
}

impl<'a> TokenReader<'a> {
	pub fn new(tokens: Vec<Token<'a>>, source: &'a str) -> Self {
		Self { position: 0, tokens, source }
	}

	pub fn next(&mut self, messages: &mut Messages) -> Result<Token<'a>> {
		if self.position >= self.tokens.len() {
			let message = error!("unexpected end of file");
			messages.message(message);
			return Err(());
		}
		let token = self.tokens[self.position].clone();
		self.position += 1;
		Ok(token)
	}
	pub fn rewind(&mut self, n: usize) {
		self.position = self.position.saturating_sub(n);
	}

	pub fn next_ahead(&mut self, messages: &mut Messages) -> Result<&Token<'a>> {
		if let Ok(ahead_token) = self.next_n(1) {
			return Ok(ahead_token);
		}
		let message = error!("unexpected end of file");
		messages.message(message);
		return Err(());
	}

	pub fn peek(&self) -> Result<Token<'a>> {
		if self.position >= self.tokens.len() {
			return Err(());
		}
		Ok(self.tokens[self.position])
	}

	pub fn peek_kind(&self) -> Result<TokenKind> {
		self.peek().map(|token| token.kind)
	}

	pub fn peek_ahead(&self) -> Result<&Token<'a>> {
		let ahead_position = self.position + 1;
		if ahead_position >= self.tokens.len() {
			return Err(());
		}
		Ok(&self.tokens[ahead_position])
	}
	pub fn peek_two_ahead(&self) -> Result<&Token<'a>> {
		let two_position_ahead = self.position + 2;
		if two_position_ahead >= self.tokens.len() {
			return Err(());
		}
		Ok(&self.tokens[two_position_ahead])
	}

	pub fn expect(&mut self, kind: TokenKind, messages: &mut Messages) -> Result<Token<'a>> {
		let token = self.peek()?.clone();
		if token.kind != kind {
			let message = error!("expected '{}', found '{}'", kind, token.kind);
			messages.message(message);
			return Err(());
		}
		self.next(messages)?;
		Ok(token)
	}

	#[inline]
	pub fn expect_peek(&mut self, kind: TokenKind, messages: &mut Messages) -> Result<Token<'a>> {
		let token = self.peek()?;
		if token.kind != kind {
			let message = error!("expected '{}', found '{}'", kind, token.kind);
			messages.message(message);
			return Err(());
		}
		Ok(token)
	}
	pub fn expect_word(&mut self, word: &str, messages: &mut Messages) -> Result<Token<'a>> {
		let token = self.expect(TokenKind::Word, messages)?;
		if token.text != word {
			let message = error!("expected '{word}', found '{token}'");
			messages.message(message);
			return Err(());
		}
		Ok(token)
	}

	pub fn read_if_word(&mut self, word: &str, messages: &mut Messages) -> Result<Token<'a>> {
		match self.peek() {
			Ok(Token { kind: TokenKind::Word, text, .. }) if text == word => {
				self.next(messages)?;
				Ok(Token { kind: TokenKind::Word, text, span: self.peek()?.span })
			}
			_ => Err(()),
		}
	}

	pub fn read_if(&mut self, kind: TokenKind, messages: &mut Messages) -> Result<Token<'a>> {
		if self.peek_kind() == Ok(kind) {
			return Ok(self.next(messages)?);
		}
		Err(())
	}

	pub fn read_semicolon(&mut self, messages: &mut Messages) -> Result<Token<'a>> {
		let token = self.peek()?;
		match token {
			Token { kind: TokenKind::Semicolon, .. } => {
				self.next(messages)?;
				Ok(token)
			}
			_ => Err(()),
		}
	}

	#[inline]
	pub fn read_newlines(&mut self) {
		while self.peek().map(|token| token.kind) == Ok(TokenKind::Newline) {
			self.position += 1;
		}
	}

	pub fn source(&self) -> &'a str {
		self.source
	}

	fn peek_n(&self, n: usize) -> Result<&Token<'a>> {
		let ahead_position = self.position + n;
		if ahead_position >= self.tokens.len() {
			return Err(());
		}
		Ok(&self.tokens[ahead_position])
	}

	fn peek_n_kind(&self, n: usize) -> Result<&TokenKind> {
		self.peek_n(n).map(|token| &token.kind)
	}

	pub fn end(&self) -> bool {
		self.position >= self.tokens.len()
	}
	fn next_n(&mut self, n: usize) -> Result<&Token<'a>> {
		let ahead_position = self.position + n;
		if ahead_position >= self.tokens.len() {
			return Err(());
		}
		self.position += n;
		Ok(&self.tokens[ahead_position])
	}
}
