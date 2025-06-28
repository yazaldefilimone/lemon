use crate::ast::{Expression, Node, Type};

#[derive(Debug)]
pub enum UnaryOperator<'a> {
	Negate,
	Invert,
	BitwiseNot,
	AddressOf,
	AddressOfMut,
	Dereference,
	Cast { parsed_type: Node<Type<'a>> },
	Index { index_expression: Node<Expression<'a>> },
}

#[derive(Debug)]
pub struct UnaryOperation<'a> {
	pub operator: Node<UnaryOperator<'a>>,
	pub expression: Node<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
	Left,
	Right,
}

// should this type live in `ir.rs` instead?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
	Assign,

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

	BitwiseAnd,
	BitwiseAndAssign,
	BitwiseOr,
	BitwiseOrAssign,
	BitwiseXor,
	BitwiseXorAssign,

	Equals,
	NotEquals,

	GreaterThan,
	GreaterThanEquals,

	LessThan,
	LessThanEquals,

	LogicalAnd,
	LogicalIsAnd,
	LogicalOr,

	Range,
}

impl BinaryOperator {
	pub fn name(self) -> &'static str {
		match self {
			BinaryOperator::Assign => "Assignment",

			BinaryOperator::Add => "Addition",
			BinaryOperator::AddAssign => "Addition assignment",
			BinaryOperator::Sub => "Subtraction",
			BinaryOperator::SubAssign => "Subtraction assignment",
			BinaryOperator::Mul => "Multiplication",
			BinaryOperator::MulAssign => "Multiplication assignment",
			BinaryOperator::Div => "Division",
			BinaryOperator::DivAssign => "Division assignment",
			BinaryOperator::Modulo => "Modulo",
			BinaryOperator::ModuloAssign => "Modulo assignment",

			BinaryOperator::BitshiftLeft => "Bitshift left",
			BinaryOperator::BitshiftLeftAssign => "Bitshift left assignment",
			BinaryOperator::BitshiftRight => "Bitshift right",
			BinaryOperator::BitshiftRightAssign => "Bitshift right assignment",

			BinaryOperator::BitwiseAnd => "Bitwise and",
			BinaryOperator::BitwiseAndAssign => "Bitwise and assignment",
			BinaryOperator::BitwiseOr => "Bitwise or",
			BinaryOperator::BitwiseOrAssign => "Bitwise or assignment",
			BinaryOperator::BitwiseXor => "Bitwise xor",
			BinaryOperator::BitwiseXorAssign => "Bitwise xor assignment",

			BinaryOperator::Equals => "Equals",
			BinaryOperator::NotEquals => "Not Equals",

			BinaryOperator::GreaterThan => "Greater Than",
			BinaryOperator::GreaterThanEquals => "Greater Than Equals",
			BinaryOperator::LessThan => "Less Than",
			BinaryOperator::LessThanEquals => "Less Than Equals",

			BinaryOperator::LogicalAnd | Self::LogicalIsAnd => "Logical And",
			BinaryOperator::LogicalOr => "Logical Or",

			BinaryOperator::Range => "Range",
		}
	}
}

impl BinaryOperator {
	pub fn precedence(self) -> u32 {
		use BinaryOperator::*;
		match self {
			Assign | AddAssign | SubAssign | MulAssign | DivAssign | ModuloAssign
			| BitshiftLeftAssign | BitshiftRightAssign | BitwiseAndAssign | BitwiseOrAssign
			| BitwiseXorAssign => 0,

			Range => 1,

			LogicalAnd | LogicalIsAnd | LogicalOr => 2,

			Equals | NotEquals | GreaterThan | GreaterThanEquals | LessThan | LessThanEquals => 3,

			BitwiseAnd | BitwiseOr | BitwiseXor => 4,

			BitshiftLeft | BitshiftRight => 5,

			Add | Sub => 6,

			Mul | Div | Modulo => 7,
		}
	}

	pub fn associativity(self) -> Associativity {
		use BinaryOperator::*;
		match self {
			Assign | AddAssign | SubAssign | MulAssign | DivAssign | ModuloAssign
			| BitshiftLeftAssign | BitshiftRightAssign | BitwiseAndAssign | BitwiseOrAssign
			| BitwiseXorAssign => Associativity::Right,

			Range => Associativity::Left,

			Add | Sub | Mul | Div | Modulo | BitshiftLeft | BitshiftRight | BitwiseAnd | BitwiseOr
			| BitwiseXor => Associativity::Left,

			Equals | NotEquals | GreaterThan | GreaterThanEquals | LessThan | LessThanEquals => {
				Associativity::Left
			}

			LogicalAnd | LogicalIsAnd | LogicalOr => Associativity::Left,
		}
	}
}

#[derive(Debug)]
pub struct BinaryOperation<'a> {
	pub operator: Node<BinaryOperator>,
	pub left: Node<Expression<'a>>,
	pub right: Node<Expression<'a>>,
}
