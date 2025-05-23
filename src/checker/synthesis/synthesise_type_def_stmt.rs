use crate::checker::context::Context;
use crate::checker::events::EventId;
use crate::checker::types::FieldType;
use crate::{ast, loader::ModId, message::MessageResult};

use super::synthesise_ast_type;

pub fn synthesise_struct_def(
	struct_def: &mut ast::StructType,
	ctx: &mut Context,
	mod_id: ModId,
) -> MessageResult<Vec<FieldType>> {
	let fields = struct_def.fields.iter_mut().map(|param| synthesise_field(param, ctx, mod_id));
	fields.collect::<Result<Vec<_>, _>>()
}

pub fn synthesise_field(
	field: &mut ast::FieldType,
	ctx: &mut Context,
	mod_id: ModId,
) -> MessageResult<FieldType> {
	let name = field.lexeme().to_owned();
	let type_id = synthesise_ast_type(&field.ast_type, ctx)?;
	ctx.event.add_type(EventId::new(mod_id, field.get_range()), type_id);
	// todo: is correct?
	Ok(FieldType::new(name, type_id, ctx.borrow.create_owner()))
}
