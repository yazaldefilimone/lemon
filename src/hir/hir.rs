pub type SymbolId = u32;
pub type TypeId = u32;
pub type BlockId = u32;
pub type FunctionId = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct HirFile {
	pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirFunction {
	pub id: FunctionId,
	pub name: String,
	pub params: Vec<HirParam>,
	pub return_type: HirType,
	pub body: Option<HirBlockId>,
	pub visibility: Visibility,
	pub is_extern: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirParam {
	pub name: SymbolId,
	pub param_type: HirType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirBlock {
	pub id: BlockId,
	pub statements: Vec<HirStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirStatement {
	Declare(HirVar),
	Assign(HirVar, HirExpression),
	Return(HirExpression),
	If(HirExpression, HirBlockId, Option<HirBlockId>),
	While(HirExpression, HirBlockId),
	Block(HirBlockId),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirVar {
	pub id: SymbolId,
	pub var_type: HirType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirExpression {
	Variable(HirVar),
	Literal(Literal),
	Unary(UnaryOperator, Box<HirExpression>),
	Binary(BinaryOperator, Box<HirExpression>, Box<HirExpression>),
	Call(HirFunctionId, Vec<HirExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirType {
	pub id: TypeId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Int(i64),
	Float(f64),
	Bool(bool),
	String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
	Neg,
	Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Eq,
	Neq,
	Lt,
	Gt,
	Le,
	Ge,
	And,
	Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
	Public,
	Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirFunctionId {
	pub id: FunctionId,
	pub function_type: HirType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirBlockId {
	pub id: BlockId,
	pub block_type: HirType,
}
