use crate::checker::{context::Context, types::TypeId};
use crate::{ast, checker};
use crate::{error, symbols};

pub fn synthesise_function<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	function_node: &'a ast::Node<ast::Function>,
) -> checker::Result<TypeId> {
	let function = &function_node.item;
	let return_type = match &function.return_type {
		Some(ret_type) => synthesise_type(ctx, ret_type)?,
		None => ctx.type_store.void_type_id,
	};

	// TODO: Proper function shape storage
	let func_symbol = symbols::Symbol::function(
		function.name.item,
		0, // TODO: proper function shape index
		function.name.span,
	);

	// Add function to scope
	ctx.add_symbol(func_symbol).ok();

	// Enter function scope
	let guard = ctx.enter_function_scope(function.name.item, return_type);

	// Process parameters
	synthesise_function_parameters(ctx, &function.parameters)?;

	if let Some(body) = &function.body {
		ctx.exit_scope(guard);
		return synthesise_function_body(ctx, body, return_type);
	}

	// Exit function scope
	ctx.exit_scope(guard);

	// Functions themselves don't have a value type
	Ok(ctx.type_store.void_type_id)
}

pub fn synthesise_function_parameters<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	parameters: &'a [ast::Parameter],
) -> checker::Result<()> {
	for parameter in parameters {
		// Synthesise parameter type
		let param_type = synthesise_type(ctx, &parameter.param_type)?;
		// Add parameter as variable in scope
		let parameter_name = ast::Node::new(parameter.name.item, parameter.name.span);
		ctx.add_variable(
			parameter_name,
			param_type,
			parameter.mutable,
			false, // not used yet
		);
	}
	Ok(())
}

pub fn synthesise_anonymous_function<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	function: &'a ast::Function,
) -> checker::Result<TypeId> {
	let return_type = match &function.return_type {
		Some(ret_type) => synthesise_type(ctx, ret_type)?,
		None => ctx.type_store.void_type_id,
	};

	// Enter closure scope
	let guard = ctx.enter_closure_scope(Some(return_type));

	// Process parameters
	synthesise_function_parameters(ctx, &function.parameters);

	if let Some(body) = &function.body {
		ctx.exit_scope(guard);
		return synthesise_function_body(ctx, body, return_type);
	}
	// Exit closure scope
	ctx.exit_scope(guard);

	// TODO: Create proper closure type
	Ok(ctx.type_store.any_collapse_type_id)
}

fn synthesise_function_body<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	body: &'a ast::Node<ast::Block>,
	expected_return_type: TypeId,
) -> checker::Result<TypeId> {
	// Synthesise the body block
	let body_type = super::synthesise_block(ctx, &body.item)?;

	// Check if body type matches expected return type
	// (only if there's no explicit return statement)
	let void_type = ctx.type_store.void_type_id;
	if !ctx.types_match(expected_return_type, void_type) {
		// Function with non-void return should return a value
		if !ctx.types_match(body_type, expected_return_type) {
			let message = error!("function body type doesn't match return type");
			return Err(message.with_span(body.span));
		}
	}

	Ok(body_type)
}

pub fn synthesise_type<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	type_annotation: &'a ast::Node<ast::Type>,
) -> checker::Result<TypeId> {
	todo!()
	// match type_annotation.item {
	// 	ast::Type::Named(name) => {
	// 		// Look up the type name
	// 		if let Some(symbol) = ctx.lookup_symbol(name.item) {
	// 			if let Some(type_id) = symbol.type_id {
	// 				return type_id;
	// 			}
	// 		}

	// 		// Try primitive types
	// 		match name.item {
	// 			"bool" => ctx.type_store.bool_type_id,
	// 			"i8" => ctx.type_store.i8_type_id,
	// 			"i16" => ctx.type_store.i16_type_id,
	// 			"i32" => ctx.type_store.i32_type_id,
	// 			"i64" => ctx.type_store.i64_type_id,
	// 			"u8" => ctx.type_store.u8_type_id,
	// 			"u16" => ctx.type_store.u16_type_id,
	// 			"u32" => ctx.type_store.u32_type_id,
	// 			"u64" => ctx.type_store.u64_type_id,
	// 			"isize" => ctx.type_store.isize_type_id,
	// 			"usize" => ctx.type_store.usize_type_id,
	// 			"f32" => ctx.type_store.f32_type_id,
	// 			"f64" => ctx.type_store.f64_type_id,
	// 			"str" => ctx.type_store.string_type_id,
	// 			"void" => ctx.type_store.void_type_id,
	// 			_ => {
	// 				ctx.error(format!("unknown type '{}'", name.item), Some(type_annotation.span));
	// 				ctx.type_store.any_collapse_type_id
	// 			}
	// 		}
	// 	}
	// 	ast::Type::Reference { inner, mutable } => {
	// 		let inner_type = synthesise_type(ctx, inner);
	// 		ctx.create_pointer_type(inner_type, *mutable)
	// 	}
	// 	ast::Type::Pointer { inner, mutable } => {
	// 		let inner_type = synthesise_type(ctx, inner);
	// 		ctx.create_pointer_type(inner_type, *mutable)
	// 	}
	// 	ast::Type::Array { inner, size } => {
	// 		let element_type = synthesise_type(ctx, inner);
	// 		// TODO: Evaluate constant size expression
	// 		ctx.create_array_type(element_type, 0)
	// 	}
	// 	ast::Type::Slice { inner, mutable } => {
	// 		let element_type = synthesise_type(ctx, inner);
	// 		ctx.create_slice_type(element_type, *mutable)
	// 	}
	// 	ast::Type::Tuple(types) => {
	// 		// TODO: Implement tuple types
	// 		ctx.type_store.any_collapse_type_id
	// 	}
	// 	ast::Type::Function { parameters, return_type } => {
	// 		// TODO: Implement function types
	// 		ctx.type_store.any_collapse_type_id
	// 	}
	// 	ast::Type::Generic { base, arguments } => {
	// 		// TODO: Implement generic types
	// 		ctx.type_store.any_collapse_type_id
	// 	}
	// 	ast::Type::Union(types) => {
	// 		// TODO: Implement union types
	// 		ctx.type_store.any_collapse_type_id
	// 	}
	// 	ast::Type::Infer => {
	// 		// Type inference placeholder
	// 		ctx.type_store.any_collapse_type_id
	// 	}
	// }
}
