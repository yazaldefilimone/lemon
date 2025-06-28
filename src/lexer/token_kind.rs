#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenKind {
	Newline,

	Word,
	Number,
	String,
	FormatString,
	Codepoint,
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
	Period,
	DoublePeriod,
	TriplePeriod,
	Comma,
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
			TokenKind::Codepoint => "codepoint literal",
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
			TokenKind::Period => "'.'",
			TokenKind::DoublePeriod => "'..'",
			TokenKind::TriplePeriod => "'...'",
			TokenKind::Comma => "','",
			TokenKind::PoundSign => "'#'",
			TokenKind::FatArrow => "'=>'",

			TokenKind::Exclamation => "'!'",
		};

		write!(f, "{}", text)
	}
}
