#![allow(dead_code)]
use core::fmt;
use std::fmt::Display;

use crate::{checker::types::TypeId, range::Range};
use serde::{Deserialize, Serialize};
mod ast_type;
pub use ast_type::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
	pub stmts: Vec<Stmt>,
}

// ------- statements -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
	Let(LetStmt),
	Expr(Expr),
	Fn(FnStmt),
	ExternFn(ExternFnStmt),
	Ret(RetStmt),
	TypeDef(TypeDefStmt),
	ConstDel(ConstDelStmt),
	ConstFn(ConstFnStmt),
	Block(BlockStmt),
	If(IfStmt),

	// loop
	While(WhileStmt),
	For(ForStmt),
	Impl(ImplStmt),
}

impl Stmt {
	pub fn is_block(&self) -> bool {
		matches!(self, Stmt::Block(_))
	}
	pub fn get_range(&self) -> Range {
		match self {
			Stmt::Let(let_stmt) => let_stmt.get_range(),
			Stmt::Fn(function_stmt) => function_stmt.get_range(),
			Stmt::Block(block_stmt) => block_stmt.get_range(),
			Stmt::Expr(expr) => expr.get_range(),
			Stmt::ConstDel(const_del) => const_del.get_range(),
			Stmt::ConstFn(const_stmt) => const_stmt.get_range(),
			Stmt::Ret(ret_stmt) => ret_stmt.get_range(),
			Stmt::If(if_stmt) => if_stmt.get_range(),
			Stmt::ExternFn(extern_fn_stmt) => extern_fn_stmt.get_range(),
			Stmt::While(while_stmt) => while_stmt.get_range(),
			Stmt::For(for_stmt) => for_stmt.get_range(),
			Stmt::TypeDef(type_def_stmt) => type_def_stmt.get_range(),
			Stmt::Impl(impl_stmt) => impl_stmt.get_range(),
		}
	}
	pub fn ends_with_ret(&self) -> bool {
		match self {
			Stmt::Ret(_) => true,
			Stmt::Block(block_stmt) => block_stmt.ends_with_ret(),
			_ => false,
		}
	}
	pub fn last_stmt_range(&self) -> Range {
		match self {
			Stmt::Block(block_stmt) => block_stmt.last_stmt_range(),
			_ => self.get_range(),
		}
	}
}

// ret <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetStmt {
	pub expr: Option<Box<Expr>>,
	pub range: Range, // return range
	pub type_id: Option<TypeId>,
}

impl RetStmt {
	pub fn get_range(&self) -> Range {
		match &self.expr {
			Some(expr) => self.range.merged_with(&expr.get_range()),
			None => self.range.clone(),
		}
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStmt {
	pub cond: Box<Expr>,
	pub then: Box<Stmt>,
	pub otherwise: Option<Box<Stmt>>,
	pub range: Range, // if range
}

impl IfStmt {
	pub fn get_range(&self) -> Range {
		match &self.otherwise {
			Some(otherwise) => self.range.merged_with(&otherwise.get_range()),
			None => self.range.merged_with(&self.then.get_range()),
		}
	}
}

// const <fn>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstFnStmt {
	pub name: Ident,
	pub params: Vec<Binding>,
	pub ret_type: Option<ast_type::AstType>,
	pub body: FnBody,
	pub range: Range,    // const range
	pub fn_range: Range, // fn range
	pub ret_id: Option<TypeId>,
	pub is_pub: bool,
}

impl ConstFnStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.text
	}
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.body.get_range())
	}

	pub fn set_ret_id(&mut self, type_id: TypeId) {
		self.ret_id = Some(type_id);
	}
	pub fn get_ret_id(&self) -> Option<TypeId> {
		self.ret_id
	}

	pub fn has_pub(&mut self) {
		self.is_pub = true;
	}
	pub fn is_pub(&self) -> bool {
		self.is_pub
	}
}

// impl <name> = {
//   fn <name>(<params>): <ret_type> = {
//     <body>
//   }
// }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplStmt {
	pub self_name: Ident,
	pub items: Vec<FnStmt>,
	pub range: Range, // impl range
}

impl ImplStmt {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

// type <name> = {} or type <name> = <type>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDefStmt {
	pub name: Ident,
	pub range: Range, // type range
	pub kind: TypeDefKind,
	pub is_pub: bool,
	pub type_id: Option<TypeId>,
}

impl TypeDefStmt {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn set_is_pub(&mut self, is_pub: bool) {
		self.is_pub = is_pub;
	}
	pub fn is_pub(&self) -> bool {
		self.is_pub
	}
	pub fn lexeme(&self) -> &str {
		&self.name.text
	}

	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}

	pub fn get_struct_def(&mut self) -> Option<&mut StructType> {
		match &mut self.kind {
			TypeDefKind::Struct(struct_def) => Some(struct_def),
			_ => None,
		}
	}

	pub fn get_alias(&self) -> Option<&AstType> {
		match &self.kind {
			TypeDefKind::Alias(alias) => Some(alias),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeDefKind {
	Struct(StructType),
	Alias(AstType),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructType {
	pub fields: Vec<FieldType>,
	pub range: Range, // struct range
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldType {
	pub ident: Ident,
	pub ast_type: AstType,
	pub is_pub: bool,
	pub type_id: Option<TypeId>,
}

impl FieldType {
	pub fn new(ident: Ident, ast_type: AstType, is_pub: bool) -> Self {
		Self { ident, ast_type, is_pub, type_id: None }
	}

	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}

	pub fn get_range(&self) -> Range {
		self.ident.get_range()
	}

	pub fn lexeme(&self) -> &str {
		&self.ident.text
	}
}

// const <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstDelStmt {
	pub name: Binding,
	pub expr: Expr,
	pub range: Range, // let range
	pub is_pub: bool,
	pub type_id: Option<TypeId>,
}

impl ConstDelStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.ident.text
	}

	pub fn has_pub(&mut self) {
		self.is_pub = true;
	}

	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.name.get_range().merged_with(&self.expr.get_range()))
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

// let <pat> = <expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStmt {
	pub bind: Binding,
	pub expr: Expr,
	pub mutable: Option<Range>,
	pub range: Range, // let range
	pub type_id: Option<TypeId>,
}

impl LetStmt {
	pub fn lexeme(&self) -> &str {
		&self.bind.ident.text
	}

	pub fn is_mut(&self) -> bool {
		self.mutable.is_some()
	}
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.bind.get_range().merged_with(&self.expr.get_range()))
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
		self.bind.set_type_id(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FnBody {
	Block(BlockStmt),
	Expr(Expr),
}

impl FnBody {
	pub fn get_range(&self) -> Range {
		match self {
			FnBody::Block(block) => block.get_range(),
			FnBody::Expr(expr) => expr.get_range(),
		}
	}
}

// ------- loops -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStmt {
	pub test: Box<Expr>,
	pub body: Box<Stmt>,
	pub range: Range, // while range
}

impl WhileStmt {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStmt {
	pub value: Ident,
	pub index: Option<Ident>,
	pub iterable: Box<Expr>,
	pub body: Box<Stmt>,
	pub range: Range, // for range
}

impl ForStmt {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

// extern fn <name>(<pats>): <type> = { }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternFnStmt {
	pub name: Ident,
	pub params: Vec<Binding>,
	pub ret_type: Option<ast_type::AstType>,
	pub range: Range,    // extern fn range
	pub fn_range: Range, // fn range
	pub var_packed: Option<Range>,
	pub ret_id: Option<TypeId>,
	pub is_pub: bool,
}

impl ExternFnStmt {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn set_is_pub(&mut self, is_pub: bool) {
		self.is_pub = is_pub;
	}

	pub fn is_pub(&self) -> bool {
		self.is_pub
	}

	pub fn get_ret_id(&self) -> Option<TypeId> {
		self.ret_id
	}

	pub fn set_ret_id(&mut self, ret_id: TypeId) {
		self.ret_id = Some(ret_id);
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Generic {
	pub ident: Ident,
	pub bound: Option<AstType>,
}

impl Generic {
	pub fn get_range(&self) -> Range {
		self.ident.get_range()
	}

	pub fn lexeme(&self) -> String {
		self.ident.text.clone()
	}
}

// fn <name>(<pats>): <type> = { <stmts> }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnStmt {
	pub name: Ident,
	pub params: Vec<Binding>,
	pub is_pub: bool,
	pub ret_type: Option<ast_type::AstType>, // todo: implement this
	pub body: FnBody,
	pub range: Range, // fn range
	pub generics: Vec<Generic>,
	pub ret_id: Option<TypeId>,
}

impl FnStmt {
	pub fn lexeme(&self) -> &str {
		&self.name.text
	}

	pub fn set_is_pub(&mut self, is_pub: bool) {
		self.is_pub = is_pub;
	}

	pub fn is_pub(&self) -> bool {
		self.is_pub
	}
	pub fn get_range(&self) -> Range {
		// fn ... body
		self.range.merged_with(&self.body.get_range())
	}

	pub fn set_ret_id(&mut self, ret_id: TypeId) {
		self.ret_id = Some(ret_id);
	}
	pub fn get_ret_id(&self) -> Option<TypeId> {
		self.ret_id
	}

	pub fn is_generic(&self) -> bool {
		!self.generics.is_empty()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStmt {
	pub stmts: Vec<Stmt>,
	range: Range,
}

impl BlockStmt {
	pub fn new(stmts: Vec<Stmt>, range: Range) -> Self {
		Self { stmts, range }
	}

	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn ends_with_ret(&self) -> bool {
		self.stmts.last().map(|stmt| stmt.ends_with_ret()).unwrap_or(false)
	}

	pub fn last_stmt_range(&self) -> Range {
		self.stmts.last().map(|stmt| stmt.get_range()).unwrap_or(self.range.clone())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident {
	pub range: Range,
	pub text: String,
	pub type_id: Option<TypeId>,
}

impl Ident {
	pub fn lexeme(&self) -> &str {
		&self.text
	}
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
	pub ident: Ident,
	pub ty: Option<ast_type::AstType>,
	pub type_id: Option<TypeId>,
}

impl Binding {
	pub fn lexeme(&self) -> &str {
		&self.ident.text
	}
	pub fn get_range(&self) -> Range {
		if let Some(ty) = &self.ty {
			self.ident.get_range().merged_with(&ty.get_range())
		} else {
			self.ident.get_range()
		}
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

// ------- expressions -------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
	Group(GroupExpr),
	Fn(FnExpr),
	Assign(AssignExpr),
	Associate(AssociateExpr),
	Member(MemberExpr),
	Binary(BinaryExpr),
	Break(BaseExpr),
	Skip(BaseExpr),
	Pipe(PipeExpr),
	Unary(UnaryExpr),
	Call(CallExpr),
	Import(ImportExpr),
	Ident(Ident),
	Literal(Literal),
	Borrow(BorrowExpr),
	Deref(DerefExpr),
	StructInit(StructInitExpr),
}

impl Expr {
	pub fn get_range(&self) -> Range {
		match self {
			Expr::Fn(fn_expr) => fn_expr.get_range(),
			Expr::Group(group) => group.get_range(),
			Expr::Binary(binary) => binary.get_range(),
			Expr::Pipe(pipe) => pipe.get_range(),
			Expr::Unary(unary) => unary.get_range(),
			Expr::Call(call) => call.get_range(),
			// Expr::Ret(ret_expr) => ret_expr.get_range(),
			Expr::Ident(ident) => ident.get_range(),
			Expr::Assign(assign) => assign.get_range(),
			Expr::Literal(literal) => literal.get_range(),
			Expr::Import(import) => import.get_range(),
			Expr::Break(break_) => break_.get_range(),
			Expr::Skip(skip) => skip.get_range(),
			Expr::Borrow(ref_expr) => ref_expr.get_range(),
			Expr::Deref(deref_expr) => deref_expr.get_range(),
			Expr::Associate(associate_expr) => associate_expr.get_range(),
			Expr::Member(member_expr) => member_expr.get_range(),
			Expr::StructInit(struct_init_expr) => struct_init_expr.get_range(),
		}
	}

	pub fn get_bind_type_id(&self) -> Option<TypeId> {
		match self {
			Expr::Ident(ident) => ident.type_id,
			Expr::Borrow(ref_expr) => ref_expr.type_id,
			Expr::Deref(deref_expr) => deref_expr.type_id,
			_ => None,
		}
	}
	pub fn valid_assign_expr(&self) -> bool {
		matches!(self, Expr::Ident(_))
			| matches!(self, Expr::Borrow(_))
			| matches!(self, Expr::Deref(_))
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructInitExpr {
	pub name: Ident,
	pub fields: Vec<FiledExpr>,
	pub range: Range, // struct init range
	pub type_id: Option<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiledExpr {
	pub name: Ident,
	pub value: Expr,
	pub range: Range, // struct field range
}

impl StructInitExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleExpr {
	pub values: Vec<Expr>,
	pub range: Range, // tuple range
}
impl TupleExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnExpr {
	pub params: Vec<Binding>,
	pub ret_type: Option<ast_type::AstType>,
	pub body: Box<Stmt>,
	pub range: Range, // fn range
	pub type_id: Option<TypeId>,
}

impl FnExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.body.get_range())
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub range: Range, // assign range
	pub type_id: Option<TypeId>,
}

impl AssignExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.left.get_range()).merged_with(&self.right.get_range())
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}

	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssociateExpr {
	pub self_name: Ident,
	pub self_type: Option<TypeId>,
	pub method: Ident,
	pub range: Range, // ::
}
impl AssociateExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn set_self_type(&mut self, self_type: TypeId) {
		self.self_type = Some(self_type);
	}

	pub fn get_self_type(&self) -> Option<TypeId> {
		self.self_type
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberExpr {
	pub left: Box<Expr>,
	pub left_type: Option<TypeId>,
	pub method: Ident,
	pub range: Range, // .
}
impl MemberExpr {
	pub fn get_range(&self) -> Range {
		self.left.get_range().merged_with(&self.method.get_range())
	}

	pub fn set_left_type(&mut self, left_type: TypeId) {
		self.left_type = Some(left_type);
	}
	pub fn get_left_type(&self) -> Option<TypeId> {
		self.left_type
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupExpr {
	pub expr: Box<Expr>,
	pub range: Range, // group range (  )
}

impl GroupExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipeExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub range: Range, // pipe range
}

impl PipeExpr {
	pub fn get_range(&self) -> Range {
		self.left.get_range().merged_with(&self.range).merged_with(&self.right.get_range())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
	pub operator: Operator,
	pub type_id: Option<TypeId>,
}

impl BinaryExpr {
	pub fn new(left: Box<Expr>, operator: Operator, right: Box<Expr>) -> Self {
		Self { left, operator, right, type_id: None }
	}

	pub fn get_range(&self) -> Range {
		self.left.get_range().merged_with(&self.operator.range).merged_with(&self.right.get_range())
	}

	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id)
	}

	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
	pub operand: Box<Expr>,
	pub operator: Operator,
	pub range: Range,
}

impl UnaryExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.operand.get_range())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
	pub callee: Box<Expr>,
	pub args: Vec<Expr>,
	pub args_type: Vec<TypeId>,
	pub range: Range, // (args...)
	pub generics: Vec<AstType>,
	pub ret_type_id: Option<TypeId>,
}

impl CallExpr {
	pub fn new(callee: Expr, args: Vec<Expr>, range: Range, generics: Vec<AstType>) -> Self {
		Self { callee: Box::new(callee), args, range, generics, ret_type_id: None, args_type: vec![] }
	}

	pub fn get_range(&self) -> Range {
		self.callee.get_range().merged_with(&self.range)
	}
	pub fn set_ret_type_id(&mut self, ret_type_id: TypeId) {
		self.ret_type_id = Some(ret_type_id);
	}
	pub fn get_ret_type_id(&self) -> Option<TypeId> {
		self.ret_type_id
	}
	pub fn set_args_type(&mut self, args_type: Vec<TypeId>) {
		self.args_type = args_type;
	}
	pub fn get_args_type(&self) -> &Vec<TypeId> {
		&self.args_type
	}
}

// &<expr>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowExpr {
	pub expr: Box<Expr>,
	pub range: Range,           // ref range
	pub mutable: Option<Range>, // mutable range
	pub type_id: Option<TypeId>,
}

impl BorrowExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.expr.get_range())
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerefExpr {
	pub expr: Box<Expr>,
	pub range: Range, // deref range
	pub type_id: Option<TypeId>,
}

impl DerefExpr {
	pub fn get_range(&self) -> Range {
		self.range.merged_with(&self.expr.get_range())
	}
	pub fn set_type_id(&mut self, type_id: TypeId) {
		self.type_id = Some(type_id);
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		self.type_id
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportExpr {
	pub path: StringLiteral,
	pub range: Range,
}

impl ImportExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
	pub fn get_path(&self) -> String {
		// remove " "
		self.path.text.replace("\"", "")
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
	Number(NumberLiteral),
	String(StringLiteral),
	Char(CharLiteral),
	Bool(BoolLiteral),
	Null(BaseExpr),
}

impl Literal {
	pub fn get_range(&self) -> Range {
		match self {
			Literal::Number(num) => num.get_range(),
			Literal::String(string) => string.get_range(),
			Literal::Bool(bool) => bool.get_range(),
			Literal::Null(null) => null.get_range(),
			Literal::Char(char) => char.get_range(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberLiteral {
	pub range: Range,
	pub text: String,
	pub base: u8,     // hex 0x = 16, bin 0b  = 2, decimal = 10
	pub as_dot: bool, // float
}

pub const BASE_DECIMAL: u8 = 10;
pub const BASE_HEX: u8 = 16;
pub const BASE_BIN: u8 = 2;

impl NumberLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn as_dot(&self) -> bool {
		self.as_dot
	}

	pub fn as_hex(&self) -> bool {
		self.base == BASE_HEX
	}

	pub fn as_bin(&self) -> bool {
		self.base == BASE_BIN
	}

	pub fn as_decimal(&self) -> bool {
		self.base == BASE_DECIMAL
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
	pub range: Range,
	pub text: String,
}

impl StringLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharLiteral {
	pub range: Range,
	pub value: char,
}

impl CharLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolLiteral {
	pub range: Range,
	pub value: bool,
}

impl BoolLiteral {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseExpr {
	pub range: Range,
}

impl BaseExpr {
	pub fn get_range(&self) -> Range {
		self.range.clone()
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum OperatorKind {
	ADD,   // +
	SUB,   // -
	MUL,   // *
	DIV,   // /
	MOD,   // %
	RANGE, // ..
	EQ,    // ==
	NOTEQ, // !=
	ADDEQ, // +=
	SUBEQ, // -=
	MULEQ, // *=
	DIVEQ, // /=
	MODEQ, // %=
	LT,    // <
	GT,    // >
	AND,   // &&
	OR,    // ||
	XOR,   // ^
	BOR,   // |
	SHL,   // <<
	SHR,   // >>
	POW,   // **
	LE,    // <=
	GE,    // >=
	NOT,   // !
	PIPE,  // |>
}
pub(crate) const MIN_PDE: u8 = 0; // e.g `|`, `..`
pub(crate) const CMP_PDE: u8 = 1; // e.g `<`, `<=`, `>`, `>=`, `==`, `!=`
pub(crate) const ADD_PDE: u8 = 2; // e.g `+`, `-`
pub(crate) const MUL_PDE: u8 = 3; // e.g `*`, `/`, `%`
pub(crate) const MAX_PDE: u8 = 4; // e.g `^`, `**`
pub(crate) const UNA_PDE: u8 = 5; // e.g `!`, `-`

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operator {
	pub kind: OperatorKind,
	pub range: Range,
}

impl Operator {
	#[rustfmt::skip]
	pub fn pde(&self) -> u8 {
    match self.kind {
      OperatorKind::LT | OperatorKind::LE | OperatorKind::GT |
      OperatorKind::GE | OperatorKind::EQ | OperatorKind::NOTEQ => CMP_PDE,
      OperatorKind::ADD | OperatorKind::SUB => ADD_PDE,
      OperatorKind::MUL | OperatorKind::DIV | OperatorKind::MOD => MUL_PDE,
      OperatorKind::POW => MAX_PDE,
      OperatorKind::NOT => UNA_PDE,
      OperatorKind::PIPE | OperatorKind::RANGE => MIN_PDE,
      _ => MIN_PDE, // default as minimum
    }
  }

	pub fn next_pde(&self) -> u8 {
		if self.pde() >= MAX_PDE {
			return MAX_PDE;
		}
		self.pde() + 1
	}

	pub fn is_right_associative(&self) -> bool {
		matches!(self.kind, OperatorKind::POW)
	}

	pub fn get_range(&self) -> Range {
		self.range.clone()
	}

	pub fn display(&self) -> &'static str {
		match self.kind {
			OperatorKind::ADD => "add",
			OperatorKind::SUB => "sub",
			OperatorKind::MUL => "mul",
			OperatorKind::DIV => "div",
			OperatorKind::MOD => "get mod",
			OperatorKind::RANGE => "concat",
			OperatorKind::EQ => "compare",
			OperatorKind::NOTEQ => "compare",
			OperatorKind::LT => "compare",
			OperatorKind::GT => "compare",
			OperatorKind::AND => "compare",
			OperatorKind::LE => "compare",
			OperatorKind::GE => "compare",
			OperatorKind::NOT => "negate",
			OperatorKind::ADDEQ => "add assign",
			OperatorKind::SUBEQ => "sub assign",
			OperatorKind::MULEQ => "mul assign",
			OperatorKind::DIVEQ => "div assign",
			OperatorKind::MODEQ => "mod assign",
			OperatorKind::OR => "or",
			OperatorKind::XOR => "xor",
			OperatorKind::BOR => "bor",
			OperatorKind::SHL => "shl",
			OperatorKind::SHR => "shr",
			OperatorKind::POW => "pow",
			OperatorKind::PIPE => "pipe",
		}
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.kind {
			OperatorKind::ADD => write!(f, "+"),
			OperatorKind::SUB => write!(f, "-"),
			OperatorKind::MUL => write!(f, "*"),
			OperatorKind::DIV => write!(f, "/"),
			OperatorKind::MOD => write!(f, "%"),
			OperatorKind::RANGE => write!(f, ".."),
			OperatorKind::EQ => write!(f, "=="),
			OperatorKind::NOTEQ => write!(f, "!="),
			OperatorKind::ADDEQ => write!(f, "+="),
			OperatorKind::SUBEQ => write!(f, "-="),
			OperatorKind::MULEQ => write!(f, "*="),
			OperatorKind::DIVEQ => write!(f, "/="),
			OperatorKind::MODEQ => write!(f, "%="),
			OperatorKind::LT => write!(f, "<"),
			OperatorKind::GT => write!(f, ">"),
			OperatorKind::AND => write!(f, "&&"),
			OperatorKind::OR => write!(f, "||"),
			OperatorKind::XOR => write!(f, "^"),
			OperatorKind::BOR => write!(f, "|"),
			OperatorKind::SHL => write!(f, "<<"),
			OperatorKind::SHR => write!(f, ">>"),
			OperatorKind::POW => write!(f, "**"),
			OperatorKind::LE => write!(f, "<="),
			OperatorKind::GE => write!(f, ">="),
			OperatorKind::NOT => write!(f, "!"),
			OperatorKind::PIPE => write!(f, "|>"),
		}
	}
}
