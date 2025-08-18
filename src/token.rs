#![allow(dead_code)]
use std::fmt::Display;

use crate::span::Span;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenKind {
	Newline,

	Word,
	Number,
	String,
	FormatString,
	ByteCodepoint,

	OpenParen,
	CloseParen,

	OpenBrace,
	CloseBrace,

	OpenBracket,
	CloseBracket,

	OpenGeneric,

	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
	Modulo,
	ModuloAssign,

	BitshiftLeft,
	BitshiftLeftAssign,
	BitshiftRight,
	BitshiftRightAssign,

	Equal,
	CompEqual,
	CompNotEqual,

	CompGreater,
	CompGreaterEqual,
	CompLess,
	CompLessEqual,

	Ampersand,
	AmpersandAssign,
	Pipe,
	PipeAssign,
	Caret,
	CaretAssign,
	Tilde,

	Colon,
	DoubleColon,
	Dot,
	DoubleDot,
	TripleDot,
	Comma,
	Semicolon,
	PoundSign,
	FatArrow,

	Exclamation,
}

impl std::fmt::Display for TokenKind {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let text = match self {
			TokenKind::Newline => "newline",

			TokenKind::Word => "word",
			TokenKind::Number => "number",
			TokenKind::String => "string literal",
			TokenKind::FormatString => "format string literal",
			TokenKind::ByteCodepoint => "byte codepoint literal",

			TokenKind::OpenParen => "'('",
			TokenKind::CloseParen => "')'",

			TokenKind::OpenBrace => "'{'",
			TokenKind::CloseBrace => "'}'",

			TokenKind::OpenBracket => "'['",
			TokenKind::CloseBracket => "']'",

			TokenKind::OpenGeneric => "'<'",

			TokenKind::Add => "'+'",
			TokenKind::AddAssign => "'+='",
			TokenKind::Sub => "'-'",
			TokenKind::SubAssign => "'-='",
			TokenKind::Mul => "'*'",
			TokenKind::MulAssign => "'*='",
			TokenKind::Div => "'/'",
			TokenKind::DivAssign => "'/='",
			TokenKind::Modulo => "'%'",
			TokenKind::ModuloAssign => "'%='",

			TokenKind::BitshiftLeft => "'<<'",
			TokenKind::BitshiftLeftAssign => "'<<='",
			TokenKind::BitshiftRight => "'>>'",
			TokenKind::BitshiftRightAssign => "'>>=='",

			TokenKind::Equal => "'='",
			TokenKind::CompEqual => "'=='",
			TokenKind::CompNotEqual => "'!='",

			TokenKind::CompGreater => "'>'",
			TokenKind::CompGreaterEqual => "'>='",
			TokenKind::CompLess => "'<'",
			TokenKind::CompLessEqual => "'<='",

			TokenKind::Ampersand => "'&'",
			TokenKind::AmpersandAssign => "'&='",
			TokenKind::Pipe => "'|'",
			TokenKind::PipeAssign => "'|='",
			TokenKind::Caret => "'^'",
			TokenKind::CaretAssign => "'^='",
			TokenKind::Tilde => "'~'",

			TokenKind::Colon => "':'",
			TokenKind::DoubleColon => "'::'",
			TokenKind::Dot => "'.'",
			TokenKind::DoubleDot => "'..'",
			TokenKind::TripleDot => "'...'",
			TokenKind::Comma => "','",
			TokenKind::Semicolon => "';'",
			TokenKind::PoundSign => "'#'",
			TokenKind::FatArrow => "'=>'",

			TokenKind::Exclamation => "'!'",
		};

		write!(f, "{}", text)
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
	pub text: &'a str,
	pub kind: TokenKind,
	pub span: Span,
}

impl<'a> Token<'a> {
	pub fn new(text: &'a str, kind: TokenKind, span: Span) -> Self {
		Token { text, kind, span }
	}
}

impl Display for Token<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} - kind: {}, text: {}", self.span, self.kind, self.text)
	}
}
