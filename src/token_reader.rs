use crate::token::Token;

pub struct TokenReader<'a> {
	position: usize,
	tokens: Vec<Token<'a>>,
	source: &'a str,
}

impl<'a> TokenReader<'a> {}
