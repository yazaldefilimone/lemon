use crate::ast::{Block, Expression, Node, Span, Type};

#[derive(Debug)]
pub enum Statement<'a> {
	Expression(Node<Expression<'a>>),
	Block(Node<Block<'a>>),
	Let(Node<Let<'a>>),
	Assignment(Node<Assignment<'a>>),
}

impl<'a> Statement<'a> {
	pub fn span(&self) -> Span {
		use Statement::*;
		//TODO: Struct and Function could be improved
		match self {
			Expression(statement) => statement.span,
			Block(statement) => statement.span,
			Let(statement) => statement.span,
			Assignment(statement) => statement.span,
			// IfElseChain(statement) => statement.span,
			// Match(statement) => statement.span,
			// WhenElseChain(statement) => statement.span,
			// While(statement) => statement.span,
			// For(statement) => statement.span,
			// Import(statement) => statement.span,
			// Struct(statement) => statement.name.span,
			// Enum(statement) => statement.name.span,
			// Union(statement) => statement.name.span,
			// Trait(statement) => statement.name.span,
			// Function(statement) => statement.name.span,
			// Const(statement) => statement.span,
			// Static(statement) => statement.span,
			// Binding(statement) => statement.span,
			// Defer(statement) => statement.span,
			// Break(statement) => statement.span,
			// Continue(statement) => statement.span,
			// Yield(statement) => statement.span,
			// Return(statement) => statement.span,
		}
	}

	pub fn name_and_article(&self) -> &'static str {
		use Statement::*;
		match self {
			Expression(..) => "An expression",
			Block(..) => "A block",
			Let(..) => "A let statement",
			Assignment(..) => "An assignment",
			// IfElseChain(..) => "An if-else statement",
			// Match(..) => "A match statement",
			// While(..) => "A while loop",
			// For(..) => "A for loop",
			// WhenElseChain(..) => "A when statement",
			// Import(..) => "An import statement",
			// Struct(..) => "A struct definition",
			// Enum(..) => "An enum definition",
			// Union(..) => "A union definition",
			// Trait(..) => "A trait definition",
			// Function(..) => "A function definition",
			// Const(..) => "A const definition",
			// Static(..) => "A static definition",
			// Binding(..) => "A binding definition",
			// Defer(..) => "A defer statement",
			// Break(..) => "A break statement",
			// Continue(..) => "A continue statement",
			// Yield(..) => "A yield statement",
			// Return(..) => "A return statement",
		}
	}
}

#[derive(Debug)]
pub struct Let<'a> {
	pub name: Node<&'a str>,
	pub parsed_type: Option<Node<Type<'a>>>,
	pub expression: Node<Expression<'a>>,
	pub mutable: bool,
	pub span: Span,
}

#[derive(Debug)]
pub struct Assignment<'a> {
	pub name: Node<&'a str>,
	pub parsed_type: Option<&'a Node<Type<'a>>>,
	pub expression: Node<Expression<'a>>,
	pub mutable: bool,
	pub span: Span,
}
