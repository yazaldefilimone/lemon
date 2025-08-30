use crate::{
	ast::{self, AstType},
	checker::{
		context::Context,
		types::{BorrowType, FnType, TypeId},
	},
	error_type,
	message::MessageResult,
};

pub fn synthesise_ast_type(ast_type: &ast::AstType, ctx: &mut Context) -> MessageResult<TypeId> {
	match ast_type {
		AstType::Number(number) => synthesise_number_type(number, ctx),
		AstType::Float(float) => synthesise_float_type(float, ctx),
		AstType::Bool(bool) => Ok(TypeId::BOOL),
		AstType::Char(char) => Ok(TypeId::CHAR),
		AstType::String(string) => Ok(TypeId::STRING),
		AstType::Str(str_type) => Ok(TypeId::STR),
		AstType::Fn(fn_type) => synthesise_fn_type(fn_type, ctx),
		AstType::Borrow(borrow) => synthesise_borrow_type(borrow, ctx),
		AstType::Ident(ident) => synthesise_ident_type(ident, ctx),
		AstType::Void(_) => Ok(TypeId::VOID),
	}
}

fn synthesise_ident_type(ident: &ast::IdentType, ctx: &mut Context) -> MessageResult<TypeId> {
	if ctx.has_implementation_scope() && ident.lexeme() == "self" {
		let self_type = match ctx.get_self_scope_type() {
			Some(self_type) => self_type,
			None => return Err(error_type!("self scope not found to infer type")),
		};
		return Ok(self_type);
	}
	if let Some(infer_id) = ctx.type_store.lookup_infer_id(ident.lexeme()) {
		return Ok(*infer_id);
	}

	if let Some(type_id) = ctx.type_store.lookup_type_definition(ident.lexeme()) {
		return Ok(*type_id);
	}

	Err(error_type!("unknown type '{}' at {:?}", ident.lexeme(), ident.get_range()))
}

fn synthesise_number_type(number: &ast::NumberType, ctx: &mut Context) -> MessageResult<TypeId> {
	if number.bits == 8 {
		return if number.signed { Ok(TypeId::I8) } else { Ok(TypeId::U8) };
	}
	if number.bits == 16 {
		return if number.signed { Ok(TypeId::I16) } else { Ok(TypeId::U16) };
	}
	if number.bits == 32 {
		return if number.signed { Ok(TypeId::I32) } else { Ok(TypeId::U32) };
	}
	if number.bits == 64 {
		return if number.signed { Ok(TypeId::I64) } else { Ok(TypeId::U64) };
	}
	unreachable!();
}

fn synthesise_float_type(float: &ast::FloatType, ctx: &mut Context) -> MessageResult<TypeId> {
	if float.bits == 32 {
		return Ok(TypeId::F32);
	}
	if float.bits == 64 {
		return Ok(TypeId::F64);
	}
	unreachable!();
}

fn synthesise_fn_type(fn_type: &ast::FnType, ctx: &mut Context) -> MessageResult<TypeId> {
	let args = fn_type
		.params
		.iter()
		.map(|param| synthesise_ast_type(param, ctx))
		.collect::<Result<Vec<_>, _>>()?;

	let ret = match fn_type.ret_type.as_ref() {
		Some(ty) => synthesise_ast_type(ty, ctx)?,
		None => TypeId::VOID,
	};
	let fn_type = FnType::new(args, ret);
	Ok(ctx.type_store.add_type(fn_type.into()))
}

pub fn synthesise_borrow_type(
	borrow_type: &ast::BorrowType,
	ctx: &mut Context,
) -> MessageResult<TypeId> {
	let value = synthesise_ast_type(&borrow_type.value, ctx)?;
	let borrow = BorrowType::new(value, borrow_type.mutable);
	Ok(ctx.type_store.add_type(borrow.into()))
}
