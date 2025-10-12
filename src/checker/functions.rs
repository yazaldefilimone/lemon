use crate::{ast, context, hir, reference::Ref, symbols};
use context::Context;

pub fn check_function<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	function_node: &ast::Node<ast::Function<'a>>,
) {
	let function_span = function_node.span;
	let function = &function_node.item;
	let name = function.name.clone();

	let main = true;

	let function_c_varargs = function.parameters.item.c_varargs;

	let return_node_type = &function.return_type;
	let parameters = check_parameters(ctx, &function.parameters);

	let c_varargs = function_c_varargs;

	// Node::new(return_type, parsed_type.span)
	let return_type = match return_node_type {
		Some(_return_type) => {
			ast::Node::new(ctx.type_store.builtin.void, name.span)
			// ast::Node::new(return_type, return_node_type.span)
		}
		None => ast::Node::new(ctx.type_store.builtin.void, name.span),
	};

	let mut function_shape = hir::FunctionShape::new(name, main, parameters, c_varargs, return_type);

	if let Some(block) = &function.body {
		let block = super::statements::check_block(ctx, &block.item);
		function_shape.add_block(Ref::new(block));
	}

	ctx.function_store.add_shape(function_shape);
}

pub fn check_parameters<'a>(
	ctx: &mut Context,
	parameters_node: &ast::Node<ast::Parameters>,
) -> ast::Node<Vec<hir::ParameterShape<'a>>> {
	let parameters_span = parameters_node.span;
	let parameters = &parameters_node.item.parameters;

	let mut parameter_shapes: Vec<hir::ParameterShape<'a>> = Vec::new();

	for parameter in parameters.iter() {
		let parameter_shape = check_parameter(ctx, parameter);
		parameter_shapes.push(parameter_shape);
	}

	return ast::Node::new(parameter_shapes, parameters_span);
}

pub fn check_parameter<'a>(
	ctx: &mut Context,
	parameter: &ast::Node<ast::Parameter>,
) -> hir::ParameterShape<'a> {
	let span = parameter.span;
	let parameter = &parameter.item;
	let name_node = &parameter.name;

	let readable_index = 0;
	let mutable = parameter.mutable;
	let symbol = symbols::Symbol::variable(name_node.item, readable_index, mutable, span);
	// ctx.readables.push(name, type_id, kind, is_pointer_access_mutable);
	let hir_type = super::types::check_type(ctx, &parameter.param_type);

	todo!()
}
