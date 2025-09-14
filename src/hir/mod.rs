mod ir_number;

use ir_number::*;
use std::borrow::Cow;

use crate::{
	ast::{BinaryOperator, Node},
	checker::{
		context::store::{FunctionStore, TypeStore},
		types::TypeId,
	},
	hir::NumberValue,
	messages::Messages,
	reference::{Ref, SliceRef},
	root_layers::RootLayer,
	span::{Location, Span},
};

#[derive(Debug)]
pub struct Statement<'a> {
	pub kind: StatementKind<'a>,
	pub location: Location,
}

#[derive(Debug)]
pub enum StatementKind<'a> {
	Expression(Expression<'a>),

	When(Block<'a>),
	While(While<'a>),
	For(For<'a>),

	Let(Let<'a>),

	Defer(Box<Defer<'a>>),

	Break(Break),
	Continue(Continue),
	Return(Return<'a>),
}
#[derive(Debug)]
pub struct Let<'a> {
	pub name: &'a str,
	pub type_id: TypeId,
	pub expression: Option<Expression<'a>>,
	pub readable_index: usize,
}

#[derive(Debug)]
pub struct Defer<'a> {
	pub statement: Statement<'a>,
}

#[derive(Debug)]
pub struct Break {
	pub loop_index: usize,
}

#[derive(Debug)]
pub struct Continue {
	pub loop_index: usize,
}

#[derive(Debug)]
pub struct Yield<'a> {
	pub yield_target_index: usize,
	pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct Return<'a> {
	pub expression: Option<Expression<'a>>,
}

#[derive(Debug)]
pub struct Block<'a> {
	pub type_id: TypeId,
	pub returns: bool,
	pub statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct While<'a> {
	pub condition: Expression<'a>,
	pub body: Block<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForKind {
	InArray,
	OfArray,
	InSlice,
	OfSlice,
	Range,
	AnyCollapse,
}

#[derive(Debug)]
pub struct For<'a> {
	pub kind: ForKind,
	pub item: ResultBinding,
	pub index: Option<ResultBinding>,
	pub is_last: Option<ResultBinding>,
	pub initializer: Expression<'a>,
	pub body: Block<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct ResultBinding {
	pub type_id: TypeId,
	pub readable_index: usize,
}

#[derive(Debug)]
pub struct Expression<'a> {
	pub span: Span,
	pub type_id: TypeId,
	pub is_itself_mutable: bool,
	pub is_pointer_access_mutable: bool,
	pub returns: bool,
	pub kind: ExpressionKind<'a>,
	pub location: Location,
}

#[derive(Debug)]
pub enum ExpressionKind<'a> {
	Void,

	ModuleLayer(Ref<RootLayer<'a>>),
	Type { type_id: TypeId },

	Block(Block<'a>),

	NumberValue(NumberValue),

	BooleanLiteral(bool),
	StringLiteral(StringLiteral<'a>),
	FormatStringLiteral(FormatStringLiteral<'a>),

	ArrayLiteral(ArrayLiteral<'a>),
	SliceLiteral(SliceLiteral<'a>),
	StructLiteral(StructLiteral<'a>),

	Call(Call<'a>),
	MethodCall(Box<MethodCall<'a>>),
	Read(Read<'a>),
	StaticRead(StaticRead),
	FieldRead(Box<FieldRead<'a>>),

	UnaryOperation(Box<UnaryOperation<'a>>),
	BinaryOperation(Box<BinaryOperation<'a>>),
}
#[derive(Debug, Clone, Hash)]
pub struct TypeArguments {
	pub explicit_len: usize,
	pub implicit_len: usize,
	pub method_base_len: usize,
	pub ids: Vec<Node<TypeId>>,
}

impl std::cmp::Eq for TypeArguments {}

impl std::cmp::PartialEq for TypeArguments {
	fn eq(&self, other: &Self) -> bool {
		!self.ne(other)
	}

	fn ne(&self, other: &Self) -> bool {
		let len_mismatch = self.explicit_len != other.explicit_len
			|| self.implicit_len != other.implicit_len
			|| self.method_base_len != other.method_base_len;

		if len_mismatch {
			return true;
		}

		for (a, b) in self.ids.iter().copied().zip(other.ids.iter().copied()) {
			if a.item.index() != b.item.index() {
				return true;
			}
		}

		false
	}
}

impl TypeArguments {
	pub fn new_from_explicit(explicit: Vec<Node<TypeId>>) -> TypeArguments {
		let explicit_len = explicit.len();
		TypeArguments { ids: explicit, explicit_len, implicit_len: 0, method_base_len: 0 }
	}

	pub fn push_implicit(&mut self, implict: Node<TypeId>) {
		assert_eq!(self.method_base_len, 0);
		self.ids.push(implict);
		self.implicit_len += 1;
	}

	pub fn push_method_base(&mut self, type_id: Node<TypeId>) {
		self.ids.push(type_id);
		self.method_base_len += 1;
	}

	pub fn is_empty(&self) -> bool {
		self.ids.is_empty()
	}

	pub fn explicit_ids(&self) -> &[Node<TypeId>] {
		&self.ids[0..self.explicit_len]
	}

	pub fn specialize_with_generics<'a>(
		&mut self,
		messages: &mut Messages<'a>,
		type_store: &mut TypeStore<'a>,
		function_store: &FunctionStore<'a>,
		module_path: &'a [String],
		// generic_usages: &mut Vec<GenericUsage>,
		// enclosing_generic_parameters: &GenericParameters<'a>,
		// situation: TypeIdSpecializationSituation,
		type_arguments: &TypeArguments,
	) {
		// for original_id in &mut self.ids {
		todo!()
		// 	original_id.item = type_store.specialize_type_id_with_generics(
		// 		messages,
		// 		function_store,
		// 		module_path,
		// 		// generic_usages,
		// 		// enclosing_generic_parameters,
		// 		original_id.item,
		// 		type_arguments,
		// 		situation,
		// 	);
		// }
	}
}
#[derive(Debug, Clone)]
pub struct Function {
	pub type_arguments: Ref<TypeArguments>,
	pub generic_poisoned: bool,
	pub parameters: SliceRef<Parameter>,
	pub return_type: TypeId,
}

#[derive(Debug, Clone, Copy)]
pub struct Parameter {
	pub type_id: TypeId,
	pub is_mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
	pub function_shape_index: usize,
	pub specialization_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId {
	pub file_index: u32,
	pub scope_index: usize,
}

#[derive(Debug, Clone)]
pub struct StringLiteral<'a> {
	pub value: Cow<'a, str>,
}

#[derive(Debug)]
pub enum FormatStringItem<'a> {
	Text(Cow<'a, str>),
	Expression(Expression<'a>),
}

#[derive(Debug)]
pub struct FormatStringLiteral<'a> {
	pub items: Vec<FormatStringItem<'a>>,
}

#[derive(Debug)]
pub struct ArrayLiteral<'a> {
	pub type_id: TypeId,
	pub item_type_id: TypeId,
	pub expressions: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct SliceLiteral<'a> {
	pub type_id: TypeId,
	pub item_type_id: TypeId,
	pub expressions: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct StructLiteral<'a> {
	pub type_id: TypeId,
	pub field_initializers: Vec<FieldInitializer<'a>>,
}

#[derive(Debug)]
pub struct FieldInitializer<'a> {
	pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct Call<'a> {
	pub span: Span,
	pub name: &'a str,
	pub function_id: FunctionId,
	pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct MethodCall<'a> {
	pub base: Expression<'a>,
	pub function_id: FunctionId,
	pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct Read<'a> {
	pub name: &'a str,
	pub readable_index: usize,
}

#[derive(Debug, Clone)]
pub struct StaticRead {
	pub static_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldReadImmutableReason {
	Readable,
	ReadOnly,
}

#[derive(Debug)]
pub struct FieldRead<'a> {
	pub base: Expression<'a>,
	pub name: &'a str,
	pub field_index: usize,
	pub field_type_id: TypeId,
	pub immutable_reason: Option<FieldReadImmutableReason>,
}

#[derive(Debug)]
pub enum UnaryOperator<'a> {
	Negate,
	Invert,
	BitwiseNot,
	AddressOf,
	AddressOfMut,
	Dereference,
	Cast { type_id: TypeId },
	Index { index_expression: Expression<'a> },
	RangeIndex { index_expression: Expression<'a> },
}

#[derive(Debug)]
pub struct UnaryOperation<'a> {
	pub operator: UnaryOperator<'a>,
	pub type_id: TypeId,
	pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct BinaryOperation<'a> {
	pub operator: BinaryOperator,
	pub left: Expression<'a>,
	pub right: Expression<'a>,
	pub type_id: TypeId,
}

#[derive(Debug)]
pub struct EnumVariantToEnum<'a> {
	pub type_id: TypeId,
	pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct UnionVariantToUnion<'a> {
	pub type_id: TypeId,
	pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct SliceMutableToImmutable<'a> {
	pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct StringToFormatString<'a> {
	pub expression: Expression<'a>,
}

impl<'a> ExpressionKind<'a> {
	pub fn name(&self) -> &'static str {
		match self {
			ExpressionKind::Void => "void value",
			ExpressionKind::ModuleLayer(_) => "module",
			ExpressionKind::Type { .. } => "type",
			ExpressionKind::Block(_) => "block",
			// ExpressionKind::IfElseChain(_) => "if expression",
			// ExpressionKind::Match(_) => "match expression",
			ExpressionKind::NumberValue(_) => "untyped number",
			ExpressionKind::BooleanLiteral(_) => "boolean literal",
			ExpressionKind::StringLiteral(_) => "string literal",
			ExpressionKind::FormatStringLiteral(_) => "format string literal",
			ExpressionKind::ArrayLiteral(_) => "array literal",
			ExpressionKind::SliceLiteral(_) => "slice literal",
			ExpressionKind::StructLiteral(_) => "struct literal",
			// ExpressionKind::FieldlessVariantLiteral => "fieldless variant literal",
			// ExpressionKind::IntegerBitflagsLiteral(_) => "integer bitflags literal",
			ExpressionKind::Call(_) => "function call",
			ExpressionKind::MethodCall(_) => "method call",
			ExpressionKind::Read(_) => "binding read",
			ExpressionKind::StaticRead(_) => "static read",
			ExpressionKind::FieldRead(_) => "field read",
			ExpressionKind::UnaryOperation(_) => "unary operation",
			ExpressionKind::BinaryOperation(_) => "binary operation",
			// ExpressionKind::EnumVariantToEnum(inner) => inner.expression.kind.name_with_article(),
			// ExpressionKind::UnionVariantToUnion(inner) => inner.expression.kind.name_with_article(),
			// ExpressionKind::SliceMutableToImmutable(inner) => {
			// inner.expression.kind.name_with_article()
			// }
			// ExpressionKind::StringToFormatString(inner) => inner.expression.kind.name_with_article(),
		}
	}

	pub fn name_with_article(&self) -> &'static str {
		match self {
			ExpressionKind::Void => "a void value",
			ExpressionKind::ModuleLayer(_) => "a module",
			ExpressionKind::Type { .. } => "a type",
			ExpressionKind::Block(_) => "a block",
			// ExpressionKind::Match(_) => "a match expression",
			ExpressionKind::NumberValue(_) => "an untyped number",
			ExpressionKind::BooleanLiteral(_) => "a boolean literal",
			ExpressionKind::StringLiteral(_) => "a string literal",
			ExpressionKind::FormatStringLiteral(_) => "a format string literal",
			ExpressionKind::ArrayLiteral(_) => "an array literal",
			ExpressionKind::SliceLiteral(_) => "a slice literal",
			ExpressionKind::StructLiteral(_) => "a struct literal",
			ExpressionKind::Call(_) => "a function call",
			ExpressionKind::MethodCall(_) => "a method call",
			ExpressionKind::Read(_) => "a binding read",
			ExpressionKind::StaticRead(_) => "a static read",
			ExpressionKind::FieldRead(_) => "a field read",
			ExpressionKind::UnaryOperation(_) => "an unary operation",
			ExpressionKind::BinaryOperation(_) => "a binary operation",
		}
	}
}
