use crate::{ast::Span, lexer::token::Token};
use std::hash::Hash;

#[derive(Debug)]
pub struct Node<T> {
	pub item: T,
	pub span: Span,
}

impl<T> Node<T> {
	pub fn new(node: T, span: Span) -> Node<T> {
		Node { item: node, span }
	}
	pub fn from_token(node: T, token: Token) -> Node<T> {
		Node { item: node, span: token.span }
	}
}

impl<T: Copy> Copy for Node<T> {}

impl<T: Clone> Clone for Node<T> {
	fn clone(&self) -> Self {
		Self { item: self.item.clone(), span: self.span }
	}
}

// This could be a potential footgun 😅
impl<T: Hash> Hash for Node<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.item.hash(state);
	}
}
