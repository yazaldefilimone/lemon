#![allow(dead_code)]
use std::borrow::Cow;

use crate::{resolver::file::SourceFile, span::Span, token::Token};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Node<T> {
	pub item: T,
	pub span: Span,
}

impl<T> Node<T> {
	pub fn new(item: T, span: Span) -> Self {
		Self { item, span }
	}

	pub fn from_token(node: T, token: Token) -> Node<T> {
		Node { item: node, span: token.span }
	}

	pub fn from_span(node: T, span: Span) -> Node<T> {
		Node { item: node, span }
	}
}

#[derive(Debug)]
pub struct File<'a> {
	pub file: &'a SourceFile,
	pub block: Block<'a>,
}

#[derive(Debug)]
pub struct Block<'a> {
	pub statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub enum Statement<'a> {
	Struct(Node<Struct<'a>>),
	Enum(Node<Enum<'a>>),
	Union(Node<Union<'a>>),
	Function(Node<Function<'a>>),
	Const(Node<Constant<'a>>),
	Let(Node<Let<'a>>),
	Command(Node<Command<'a>>),
	Expression(Node<Expression<'a>>),
	Block(Node<Block<'a>>),
	IfElseChain(Node<IfElseChain<'a>>),
	Match(Node<Match<'a>>),
	While(Node<While<'a>>),
	For(Node<For<'a>>),
	WhenElseChain(Node<WhenElseChain<'a>>),
	Return(Node<Expression<'a>>),
}

#[derive(Debug)]
pub struct Let<'a> {
	pub pattern: Pattern<'a>,
	pub parsed_type: Option<Node<Type<'a>>>,
	pub expression: Node<Expression<'a>>,
	pub mutable: bool,
}

#[derive(Debug)]
pub enum Pattern<'a> {
	Identifier(Node<&'a str>),
	Tuple(Vec<Node<Pattern<'a>>>),
	Array(Vec<Node<Pattern<'a>>>),
	Struct { name: Node<&'a str>, fields: Vec<Node<Pattern<'a>>> },
	Wildcard,
}

#[derive(Debug)]
pub enum Command<'a> {
	Break,
	Continue,
	Defer(Box<Statement<'a>>),
	Return(Option<Box<Expression<'a>>>),
	Yield(Box<Expression<'a>>),
}

#[derive(Debug)]
pub enum Expression<'a> {
	Block(Block<'a>),
	IfElseChain(Box<IfElseChain<'a>>),
	Match(Box<Match<'a>>),

	NumberLiteral(NumberLiteral),
	BooleanLiteral(bool),
	StringLiteral(StringLiteral<'a>),
	FormatStringLiteral(FormatStringLiteral<'a>),

	ArrayLiteral(ArrayLiteral<'a>),
	SliceLiteral(SliceLiteral<'a>),
	StructLiteral(StructLiteral<'a>),

	Call(Call<'a>),
	Read(Read<'a>),
	DotAccess(DotAccess<'a>),

	UnaryOperation(UnaryOperation<'a>),
	BinaryOperation(BinaryOperation<'a>),
	CheckIs(CheckIs<'a>),
}

#[derive(Debug)]
pub struct IfElseChain<'a> {
	pub entries: Vec<IfElseEntry<'a>>,
	pub else_body: Option<Block<'a>>,
}

impl<'a> IfElseChain<'a> {
	pub fn new(entries: Vec<IfElseEntry<'a>>, else_body: Option<Block<'a>>) -> Self {
		Self { entries, else_body }
	}
}

#[derive(Debug)]
pub struct IfElseEntry<'a> {
	pub condition: Expression<'a>,
	pub body: Block<'a>,
}

#[derive(Debug)]
pub struct WhenElseChain<'a> {
	pub entries: Vec<WhenElseEntry<'a>>,
	pub else_body: Option<Block<'a>>,
}

#[derive(Debug)]
pub struct WhenElseEntry<'a> {
	pub condition: Node<&'a str>,
	pub body: Block<'a>,
}

#[derive(Debug)]
pub struct Match<'a> {
	pub expression: Expression<'a>,
	pub arms: Vec<MatchArm<'a>>,
	pub else_arm: Option<Block<'a>>,
}

#[derive(Debug)]
pub struct MatchArm<'a> {
	pub binding_name: Option<Node<&'a str>>,
	pub variant_names: Vec<Node<&'a str>>,
	pub block: Block<'a>,
}

#[derive(Debug)]
pub struct While<'a> {
	pub condition: Expression<'a>,
	pub body: Block<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationKind {
	In,
	Of,
}

#[derive(Debug)]
pub struct For<'a> {
	pub item: Node<&'a str>,
	pub index: Option<Node<&'a str>>,
	pub is_last: Option<Node<&'a str>>,
	pub iteration_kind: IterationKind,
	pub initializer: Expression<'a>,
	pub body: Block<'a>,
}

#[derive(Debug)]
pub struct Struct<'a> {
	pub name: Node<&'a str>,
	pub generics: Vec<GenericName<'a>>,
	pub is_opaque: bool,
	pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Enum<'a> {
	pub name: Node<&'a str>,
	pub generics: Vec<GenericName<'a>>,
	pub is_bitflags: bool,
	pub tag_type: Option<String>,
	pub shared_fields: Vec<Field<'a>>,
	pub variants: Vec<Variant<'a>>,
}

#[derive(Debug)]
pub struct Union<'a> {
	pub name: Node<&'a str>,
	pub generics: Vec<GenericName<'a>>,
	pub variants: Vec<Variant<'a>>,
}

#[derive(Debug)]
pub struct Function<'a> {
	pub name: Node<&'a str>,
	pub generics: Vec<GenericName<'a>>,
	pub parameters: Vec<Parameter<'a>>,
	pub return_type: Option<Node<Type<'a>>>,
	pub body: Option<Node<Block<'a>>>,
	pub extern_attribute: Option<&'a Node<ExternAttribute<'a>>>,
	pub pub_attribute: Option<&'a Node<PubAttribute<'a>>>,
	pub method_kind: Option<MethodKind>,
	pub lang_attribute: Option<String>,
}

impl<'a> Function<'a> {
	pub fn new(
		name: Node<&'a str>,
		parameters: Vec<Parameter<'a>>,
		return_type: Option<Node<Type<'a>>>,
		body: Option<Node<Block<'a>>>,
	) -> Self {
		Self {
			name,
			parameters,
			generics: Vec::new(),
			return_type,
			body,
			extern_attribute: None,
			pub_attribute: None,
			method_kind: None,
			lang_attribute: None,
		}
	}
}

#[derive(Debug)]
pub struct Constant<'a> {
	pub name: Node<&'a str>,
	pub ty: Option<Type<'a>>,
	pub expression: Node<Expression<'a>>,
}

#[derive(Debug)]
pub struct PathSegments<'a> {
	pub segments: &'a [Node<&'a str>],
}
#[derive(Debug, Clone, Copy)]
pub struct ExternAttribute<'a> {
	pub name: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct PubAttribute<'a> {
	pub name: &'a str,
}

#[derive(Debug)]
pub struct GenericAttribute<'a> {
	pub names: &'a [GenericName<'a>],
}

#[derive(Debug)]
pub struct GenericName<'a> {
	pub name: Node<&'a str>,
	pub constraints: Vec<GenericConstraint<'a>>,
}

#[derive(Debug)]
pub struct GenericConstraint<'a> {
	pub path: PathSegments<'a>,
	pub type_arguments: Vec<Type<'a>>,
}

#[derive(Debug)]
pub enum Type<'a> {
	Void,

	Pointer {
		pointee: &'a Node<Type<'a>>,
		mutable: bool,
	},

	#[allow(unused)]
	FunctionPointer {
		parameters: &'a [Node<Type<'a>>],
		return_type: Option<&'a Node<Type<'a>>>,
	},

	Array {
		item: &'a Node<Type<'a>>,
		length: u64, // TODO: Allow array length to be a proper expression
	},

	Slice {
		pointee: &'a Node<Type<'a>>,
		mutable: bool,
	},

	Path {
		path_segments: Node<PathSegments<'a>>,
		type_arguments: &'a [Node<Type<'a>>],
		dot_access_chain: &'a [Node<&'a str>],
	},
}

#[derive(Debug)]
pub struct Field<'a> {
	pub name: Node<&'a str>,
	pub ty: Type<'a>,
	pub is_read_only: bool,
	pub attributes: Vec<FieldAttribute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldAttribute {
	Internal,
	Readable,
}

#[derive(Debug)]
pub struct Variant<'a> {
	pub kind: VariantKind<'a>,
	pub tag_value: Option<Expression<'a>>,
}

#[derive(Debug)]
pub enum VariantKind<'a> {
	StructLike(StructLikeVariant<'a>),
	Transparent(TransparentVariant<'a>),
}

#[derive(Debug)]
pub struct StructLikeVariant<'a> {
	pub name: Node<&'a str>,
	pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct TransparentVariant<'a> {
	pub name: Node<&'a str>,
	pub ty: Type<'a>,
}

#[derive(Debug)]
pub struct Parameter<'a> {
	pub name: Node<&'a str>,
	pub param_type: Node<Type<'a>>,
	pub mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
	Static,
	ImmutableSelf,
	MutableSelf,
}

impl MethodKind {
	pub fn as_str(&self) -> &'static str {
		match self {
			MethodKind::Static => "static",
			MethodKind::ImmutableSelf => "immutable",
			MethodKind::MutableSelf => "mutable",
		}
	}
}

#[derive(Debug)]
pub struct NumberLiteral {
	pub value: Node<Decimal>,
}

impl NumberLiteral {
	pub fn new(value: Node<Decimal>) -> Self {
		Self { value }
	}
}

#[derive(Debug, Clone)]
pub struct StringLiteral<'a> {
	pub value: Cow<'a, str>,
}

impl<'a> StringLiteral<'a> {
	pub fn new(value: Cow<'a, str>) -> Self {
		Self { value }
	}
}

#[derive(Debug)]
pub struct FormatStringLiteral<'a> {
	pub items: Vec<Node<FormatStringItem<'a>>>,
}

impl<'a> FormatStringLiteral<'a> {
	pub fn new(items: Vec<Node<FormatStringItem<'a>>>) -> Self {
		Self { items }
	}
}

#[derive(Debug)]
pub enum FormatStringItem<'a> {
	Text(Cow<'a, str>),
	Expression(Expression<'a>),
}

impl<'a> FormatStringItem<'a> {
	pub fn new_text(text: Cow<'a, str>) -> Self {
		Self::Text(text)
	}

	pub fn new_expression(expression: Expression<'a>) -> Self {
		Self::Expression(expression)
	}
}

#[derive(Debug)]
pub struct ArrayLiteral<'a> {
	pub ty: Option<Type<'a>>,
	pub elements: Vec<Node<Expression<'a>>>,
}

impl<'a> ArrayLiteral<'a> {
	pub fn new(ty: Option<Type<'a>>, elements: Vec<Node<Expression<'a>>>) -> Self {
		Self { ty, elements }
	}
}

#[derive(Debug)]
pub struct SliceLiteral<'a> {
	pub ty: Option<Type<'a>>,
	pub elements: Vec<Expression<'a>>,
	pub mutable: bool,
}

impl<'a> SliceLiteral<'a> {
	pub fn new(ty: Option<Type<'a>>, elements: Vec<Expression<'a>>, mutable: bool) -> Self {
		Self { ty, elements, mutable }
	}
}

#[derive(Debug)]
pub struct StructLiteral<'a> {
	pub base: Box<Node<Expression<'a>>>,
	pub initializer: Node<StructInitializer<'a>>,
}

impl<'a> StructLiteral<'a> {
	pub fn new(base: Node<Expression<'a>>, initializer: Node<StructInitializer<'a>>) -> Self {
		Self { base: Box::new(base), initializer }
	}
}

#[derive(Debug)]
pub struct StructInitializer<'a> {
	pub field_initializers: Vec<Node<FieldInitializer<'a>>>,
}

impl<'a> StructInitializer<'a> {
	pub fn new(field_initializers: Vec<Node<FieldInitializer<'a>>>) -> Self {
		Self { field_initializers }
	}
}

#[derive(Debug)]
pub struct FieldInitializer<'a> {
	pub name: Node<&'a str>,
	pub expression: Expression<'a>,
}

impl<'a> FieldInitializer<'a> {
	pub fn new(name: Node<&'a str>, expression: Expression<'a>) -> Self {
		Self { name, expression }
	}
}

#[derive(Debug)]
pub struct Call<'a> {
	pub base: Option<Box<Node<Expression<'a>>>>,
	pub name: Node<&'a str>,
	pub type_arguments: Vec<Type<'a>>,
	pub arguments: Vec<Argument<'a>>,
}

impl<'a> Call<'a> {
	pub fn new(
		base: Option<Node<Expression<'a>>>,
		name: Node<&'a str>,
		type_arguments: Vec<Type<'a>>,
		arguments: Vec<Argument<'a>>,
	) -> Self {
		Self { base: base.map(Box::new), name, type_arguments, arguments }
	}
}

#[derive(Debug)]
pub struct Argument<'a> {
	pub expression: Node<Expression<'a>>,
}

#[derive(Debug)]
pub struct Read<'a> {
	pub name: Node<&'a str>,
	pub type_arguments: Vec<Type<'a>>,
}

#[derive(Debug)]
pub struct DotAccess<'a> {
	pub base: Box<Node<Expression<'a>>>,
	pub name: Node<&'a str>,
	pub type_arguments: Vec<Type<'a>>,
}

impl<'a> DotAccess<'a> {
	pub fn new(
		base: Node<Expression<'a>>,
		name: Node<&'a str>,
		type_arguments: Vec<Type<'a>>,
	) -> Self {
		Self { base: Box::new(base), name, type_arguments }
	}
}

#[derive(Debug)]
pub struct UnaryOperation<'a> {
	pub operator: Box<Node<UnaryOperator<'a>>>,
	pub expression: Box<Node<Expression<'a>>>,
}

impl<'a> UnaryOperation<'a> {
	pub fn new(operator: Node<UnaryOperator<'a>>, expression: Node<Expression<'a>>) -> Self {
		Self { operator: Box::new(operator), expression: Box::new(expression) }
	}
}

#[derive(Debug)]
pub enum UnaryOperator<'a> {
	Negate,                                     // -expr
	Invert,                                     // !expr  (booleano, lógico)
	BitwiseNot,                                 // ~expr  (bit a bit)
	AddressOf { mutable: bool },                // &expr / &mut expr
	Dereference,                                // *expr
	Cast { parsed_type: Node<Type<'a>> },       // expr as <type>
	Index { expression: Node<Expression<'a>> }, // expr[index]
}

impl<'a> UnaryOperator<'a> {
	pub fn name(&self) -> &'static str {
		use UnaryOperator::*;
		match self {
			Negate => "Negate",
			Invert => "Invert",
			BitwiseNot => "Bitwise not",
			AddressOf { mutable } => {
				if *mutable {
					"Mutable address of"
				} else {
					"Address of"
				}
			}
			Dereference => "Dereference",
			Cast { .. } => "Cast",
			Index { .. } => "Index",
		}
	}
}

#[derive(Debug)]
pub struct BinaryOperation<'a> {
	pub operator: Node<BinaryOperator>,
	pub left: Box<Node<Expression<'a>>>,
	pub right: Box<Node<Expression<'a>>>,
}

impl<'a> BinaryOperation<'a> {
	pub fn new(
		operator: Node<BinaryOperator>,
		left: Node<Expression<'a>>,
		right: Node<Expression<'a>>,
	) -> Self {
		Self { operator, left: Box::new(left), right: Box::new(right) }
	}
}

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
	pub fn name(&self) -> &'static str {
		use BinaryOperator::*;
		match self {
			Assign => "Assignment",
			Add => "Addition",
			AddAssign => "Addition assignment",
			Sub => "Subtraction",
			SubAssign => "Subtraction assignment",
			Mul => "Multiplication",
			MulAssign => "Multiplication assignment",
			Div => "Division",
			DivAssign => "Division assignment",
			Modulo => "Modulo",
			ModuloAssign => "Modulo assignment",
			BitshiftLeft => "Bitshift left",
			BitshiftLeftAssign => "Bitshift left assignment",
			BitshiftRight => "Bitshift right",
			BitshiftRightAssign => "Bitshift right assignment",
			BitwiseAnd => "Bitwise and",
			BitwiseAndAssign => "Bitwise and assignment",
			BitwiseOr => "Bitwise or",
			BitwiseOrAssign => "Bitwise or assignment",
			BitwiseXor => "Bitwise xor",
			BitwiseXorAssign => "Bitwise xor assignment",
			Equals => "Equals",
			NotEquals => "Not equals",
			GreaterThan => "Greater than",
			GreaterThanEquals => "Greater than equals",
			LessThan => "Less than",
			LessThanEquals => "Less than equals",
			LogicalAnd | LogicalIsAnd => "Logical and",
			LogicalOr => "Logical or",
			Range => "Range",
		}
	}
}

#[rustfmt::skip]
impl BinaryOperator {
	pub fn precedence(&self) -> u32 {
		use BinaryOperator::*;
		match self {
			Assign | AddAssign | SubAssign
			| MulAssign | DivAssign | ModuloAssign
			| BitshiftLeftAssign | BitshiftRightAssign
			| BitwiseAndAssign | BitwiseOrAssign
			| BitwiseXorAssign => 0,

			Range => 1,

			LogicalAnd | LogicalIsAnd | LogicalOr => 2,

			Equals | NotEquals | GreaterThan |
			GreaterThanEquals | LessThan | LessThanEquals => 3,

			BitwiseAnd | BitwiseOr | BitwiseXor => 4,

			BitshiftLeft | BitshiftRight => 5,

			Add | Sub => 6,

			Mul | Div | Modulo => 7,
		}
	}

	pub fn associativity(&self) -> Associativity {
		use BinaryOperator::*;
		match self {
			Assign | AddAssign | SubAssign
			| MulAssign | DivAssign | ModuloAssign
			| BitshiftLeftAssign | BitshiftRightAssign
			| BitwiseAndAssign | BitwiseOrAssign
			| BitwiseXorAssign => Associativity::Right,

			Range => Associativity::Left,

			Add | Sub | Mul | Div | Modulo
			| BitshiftLeft | BitshiftRight
			| BitwiseAnd | BitwiseOr| BitwiseXor => Associativity::Left,

			Equals | NotEquals | GreaterThan
		 | GreaterThanEquals | LessThan
			| LessThanEquals => Associativity::Left,

			LogicalAnd | LogicalIsAnd | LogicalOr => Associativity::Left,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
	Left,
	Right,
}

#[derive(Debug)]
pub struct CheckIs<'a> {
	pub left: Box<Node<Expression<'a>>>,
	pub binding_name: Option<Node<&'a str>>,
	pub variant_names: Vec<Node<&'a str>>,
}

impl<'a> CheckIs<'a> {
	pub fn new(
		left: Node<Expression<'a>>,
		binding_name: Option<Node<&'a str>>,
		variant_names: Vec<Node<&'a str>>,
	) -> Self {
		Self { left: Box::new(left), binding_name, variant_names }
	}
}

pub struct AllowedAttributes {
	pub extern_attribute: bool,
	pub pub_attribute: bool,
}

#[derive(Debug)]
pub struct Attributes<'a> {
	pub extern_attribute: Option<Node<ExternAttribute<'a>>>,
	pub pub_attribute: Option<Node<PubAttribute<'a>>>,
}

impl<'a> Attributes<'a> {
	pub const FIELD_COUNT: usize = 2;

	pub fn blank() -> Self {
		Attributes { extern_attribute: None, pub_attribute: None }
	}

	pub fn attribute_spans<'b>(&self, buffer: &'b mut [Span]) -> &'b [Span] {
		fn push_potential_span<T>(attribute: Option<&Node<T>>, buffer: &mut [Span], index: &mut usize) {
			if let Some(attribute) = attribute {
				buffer[*index] = attribute.span;
				*index += 1;
			}
		}

		let mut index = 0;
		push_potential_span(self.extern_attribute.as_ref(), buffer, &mut index);
		push_potential_span(self.pub_attribute.as_ref(), buffer, &mut index);
		&buffer[0..index]
	}
}
