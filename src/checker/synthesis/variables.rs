use crate::{
	ast::{self, Node},
	checker::{self, context::Context, types::TypeId},
	symbols::{Symbol, SymbolKind},
};

pub fn synthesise_let<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	_declaration: &'b Node<ast::Let>,
) -> checker::Result<TypeId> {
	Ok(ctx.type_store.void_type_id)
}

pub fn synthesise_constant<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	declaration: &'a Node<ast::Constant>,
) -> checker::Result<TypeId> {
	// Constants must have compile-time known values
	let value_type = super::synthesise_expression(ctx, &declaration.item.expression)?;

	// Create a const symbol
	// TODO: We need a proper constant storage like we have for readables
	// For now, just add it as a symbol
	let declaration_name = declaration.item.name.item;
	let declaration_kind = SymbolKind::Const { constant_index: 0 }; // TODO: proper index
	let declaration_span = declaration.item.name.span;

	let symbol = Symbol::new(declaration_name, declaration_kind, Some(declaration_span));

	ctx.add_symbol(symbol).ok();

	Ok(value_type)
}
