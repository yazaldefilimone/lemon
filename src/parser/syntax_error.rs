use crate::{
	error,
	lexer::{
		token::{Token, TokenStream},
		TokenKind,
	},
	messages::{Messages, ParseResult},
};

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
			| "method"
			| "static"
			| "trait"
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

pub fn check_not_reserved(messages: &mut Messages, token: Token, use_as: &str) -> ParseResult<()> {
	if is_word_reserved(token.text) {
		let error = error!("cannot use reserved word {:?} as {use_as}", token.text);
		messages.message(error.span(token.span));
		Err(())
	} else {
		Ok(())
	}
}
pub fn consume_error_syntax(messages: &mut Messages, tokens: &mut TokenStream) {
	let mut brackets = 0;
	let mut parens = 0;
	let mut braces = 0;

	let mut span = None;

	while let Ok(token) = tokens.peek() {
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

		tokens.next(messages).expect("this should never fail");
	}

	//Reached end of file while unbalanced
	if brackets != 0 {
		messages.message(error!("unbalanced brackets").span_if_some(span));
	}
	if parens != 0 {
		messages.message(error!("unbalanced parentheses").span_if_some(span));
	}
	if braces != 0 {
		messages.message(error!("unbalanced braces").span_if_some(span));
	}
}
