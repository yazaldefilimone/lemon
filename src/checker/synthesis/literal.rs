use crate::{
	ast,
	checker::{self, context::Context, types::TypeId},
};

#[inline(always)]
pub fn synthesise_number_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	_number_literal: &'b ast::NumberLiteral,
) -> checker::Result<TypeId> {
	Ok(ctx.type_store.number_type_id)
}

#[inline(always)]
pub fn synthesise_boolean_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	_boolean_literal: bool,
) -> checker::Result<TypeId> {
	Ok(ctx.type_store.bool_type_id)
}

#[inline(always)]
pub fn synthesise_string_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	_string_literal: &'b ast::StringLiteral,
) -> checker::Result<TypeId> {
	Ok(ctx.type_store.string_type_id)
}

#[inline(always)]
pub fn synthesise_format_string_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	_format_string_literal: &'b ast::FormatStringLiteral,
) -> checker::Result<TypeId> {
	Ok(ctx.type_store.format_string_type_id)
}

pub fn synthesise_slice_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	literal: ast::SliceLiteral,
) -> checker::Result<TypeId> {
	// TODO: Implement slice literal synthesis
	Ok(ctx.type_store.any_collapse_type_id)
}

pub fn synthesise_struct_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	literal: &'b ast::Node<ast::StructLiteral>,
) -> checker::Result<TypeId> {
	// TODO: Implement struct literal synthesis
	Ok(ctx.type_store.any_collapse_type_id)
}
// pub fn synthesise_array_literal<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	array_literal: &'b ast::ArrayLiteral,
// ) -> checker::Result<TypeId> {
// 	if array_literal.elements.is_empty() {
// 		return error!("empty array literal needs type annotation");
// 	}

// 	let first_item = &array_literal.item.elements[0];
// 	let element_type = synthesis::synthesise_expression(ctx, first_item);

// 	for item in &array_literal.item.elements[1..] {
// 		let item_type = synthesis::synthesise_expression(ctx, item);
// 		if !ctx.types_match(element_type, item_type) {
// 			ctx.error(format!("array elements must have same type"), Some(item.span));
// 		}
// 	}

// 	let length = array_literal.item.elements.len() as u64;
// 	Ok(ctx.create_array_type(element_type, length))
// }
