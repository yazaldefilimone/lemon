use std::borrow::Cow;

use crate::ast::{Block, Node, Type};

#[derive(Debug)]
pub enum Expression<'a> {
	Block(Block<'a>),
	// IfElseChain(&'a IfElseChain<'a>),
	// Match(&'a Match<'a>),

	// NumberLiteral(NumberLiteral),

	// BooleanLiteral(bool),
	// CodepointLiteral(CodepointLiteral),
	// ByteCodepointLiteral(ByteCodepointLiteral),
	StringLiteral(StringLiteral<'a>),
	FormatStringLiteral(FormatStringLiteral<'a>),
	ArrayLiteral(ArrayLiteral<'a>),
	// SliceLiteral(SliceLiteral<'a>),
	// StructLiteral(&'a StructLiteral<'a>),

	// Call(&'a Call<'a>),
	// Read(Read<'a>),
	// DotAccess(&'a DotAccess<'a>),
	// DotInfer(&'a DotInfer<'a>),
	// DotInferCall(&'a DotInferCall<'a>),

	// UnaryOperation(&'a UnaryOperation<'a>),
	// BinaryOperation(&'a BinaryOperation<'a>),
	// CheckIs(&'a CheckIs<'a>),
}

#[derive(Debug)]
pub struct StringLiteral<'a> {
	pub value: Cow<'a, str>,
}

#[derive(Debug)]
pub enum FormatStringItem<'a> {
	Text(Cow<'a, str>),
	Expression(Node<Expression<'a>>),
}

#[derive(Debug)]
pub struct FormatStringLiteral<'a> {
	pub items: &'a [FormatStringItem<'a>],
}

#[derive(Debug)]
pub struct ArrayLiteral<'a> {
	pub parsed_type: Option<Node<Type<'a>>>,
	pub expressions: &'a [Node<Expression<'a>>],
}
