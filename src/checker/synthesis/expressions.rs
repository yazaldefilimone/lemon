use crate::ast;
use crate::checker;
use crate::checker::{context::Context, types::TypeId};

pub fn synthesise_expression<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	expression: &'a ast::Node<ast::Expression>,
) -> checker::Result<TypeId> {
	match &expression.item {
		ast::Expression::StringLiteral(literal) => super::synthesise_string_literal(ctx, literal),
		ast::Expression::NumberLiteral(literal) => super::synthesise_number_literal(ctx, literal),
		ast::Expression::Block(block) => super::synthesise_block(ctx, block),
		ast::Expression::Call(call) => synthesise_call(ctx, &call),
		ast::Expression::Read(read) => synthesise_read(ctx, &read),
		ast::Expression::DotAccess(access) => synthesise_dot_access(ctx, access),
		ast::Expression::BooleanLiteral(_) => Ok(ctx.type_store.bool_type_id),
		ast::Expression::FormatStringLiteral(literal) => {
			super::synthesise_format_string_literal(ctx, literal)
		}
		// ast::Expression::SliceLiteral(literal) => synthesise_slice_literal(ctx, literal),
		ast::Expression::UnaryOperation(operation) => synthesise_unary_operation(ctx, operation),
		ast::Expression::BinaryOperation(operation) => synthesise_binary_operation(ctx, operation),
		_ => todo!(),
	}
}

pub fn synthesise_call<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	call: &'a ast::Call,
) -> checker::Result<TypeId> {
	// Synthesise the function being called
	// let fun_type = synthesise_expression(ctx, &call.name);

	// Synthesise arguments
	for argument in &call.arguments {
		synthesise_expression(ctx, &argument.expression);
	}

	// TODO: Actually resolve function return type
	// For now, return void
	Ok(ctx.type_store.void_type_id)
}

pub fn synthesise_read<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	read: &ast::Read,
) -> checker::Result<TypeId> {
	// Look up the symbol
	if let Some(symbol) = ctx.lookup_symbol(read.name.item) {
		if let Some(type_id) = ctx.get_symbol_type(&symbol) {
			return Ok(type_id);
		}
	}

	ctx.error(format!("undefined variable '{}'", read.name.item), Some(read.name.span));
	Ok(ctx.type_store.any_collapse_type_id)
}

pub fn synthesise_dot_access<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	access: &ast::DotAccess,
) -> checker::Result<TypeId> {
	// Synthesise the subject
	// let subject_type = synthesise_expression(ctx, &access.name);

	// TODO: Look up field/method on the type
	// For now, return any
	Ok(ctx.type_store.any_collapse_type_id)
}

pub fn synthesise_unary_operation<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	operation: &'a ast::UnaryOperation,
) -> checker::Result<TypeId> {
	let operand_type = synthesise_expression(ctx, &operation.expression)?;

	match &operation.operator.item {
		ast::UnaryOperator::Invert => Ok(ctx.type_store.bool_type_id),
		ast::UnaryOperator::Negate => Ok(operand_type),
		ast::UnaryOperator::AddressOf { mutable } => {
			// Create reference type
			Ok(ctx.create_pointer_type(operand_type, *mutable))
		}
		ast::UnaryOperator::BitwiseNot => Ok(operand_type),

		ast::UnaryOperator::Cast { parsed_type } => {
			// Synthesise the type
			// let type_id = synthesise_type(ctx, &parsed_type);

			// TODO: Check if operand is a pointer/reference and get inner type
			// For now, return any
			Ok(ctx.type_store.any_collapse_type_id)
		}
		ast::UnaryOperator::Index { expression } => {
			// Synthesise the index
			let index_type = synthesise_expression(ctx, &expression);

			// TODO: Check if operand is a pointer/reference and get inner type
			// For now, return any
			Ok(ctx.type_store.any_collapse_type_id)
		}

		ast::UnaryOperator::Dereference => {
			// TODO: Check if operand is a pointer/reference and get inner type
			// For now, return any
			Ok(ctx.type_store.any_collapse_type_id)
		}
	}
}

pub fn synthesise_binary_operation<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	operation: &'a ast::BinaryOperation,
) -> checker::Result<TypeId> {
	let left_type = synthesise_expression(ctx, &operation.left)?;
	let right_type = synthesise_expression(ctx, &operation.right)?;

	// Type check operands
	// if !ctx.types_match(left_type, right_type) {
	// 	ctx.warning("binary operation on different types", Some(operation.span));
	// }

	use ast::BinaryOperator::*;
	match operation.operator.item {
		Assign => todo!(),
		Add => todo!(),
		AddAssign => todo!(),
		Sub => todo!(),
		SubAssign => todo!(),
		Mul => todo!(),
		MulAssign => todo!(),
		Div => todo!(),
		DivAssign => todo!(),
		Modulo => todo!(),
		ModuloAssign => todo!(),
		BitshiftLeft => todo!(),
		BitshiftLeftAssign => todo!(),
		BitshiftRight => todo!(),
		BitshiftRightAssign => todo!(),
		BitwiseAnd => todo!(),
		BitwiseAndAssign => todo!(),
		BitwiseOr => todo!(),
		BitwiseOrAssign => todo!(),
		BitwiseXor => todo!(),
		BitwiseXorAssign => todo!(),
		Equals => todo!(),
		NotEquals => todo!(),
		GreaterThan => todo!(),
		GreaterThanEquals => todo!(),
		LessThan => todo!(),
		LessThanEquals => todo!(),
		LogicalAnd => todo!(),
		LogicalIsAnd => todo!(),
		LogicalOr => todo!(),
		Range => todo!(),
	}
}

// pub fn synthesise_if_else_chain<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	if_else_chain: &'a ast::Node<ast::IfElseChain>,
// ) -> checker::Result<TypeId> {
// 	let mut result_type = ctx.type_store.void_type_id;

// 	// Process each if/else if branch
// 	for entry in &if_else_chain.entries {
// 		// Check condition is boolean
// 		let condition_type = synthesise_expression(ctx, &entry.condition);
// 		let bool_type = ctx.type_store.bool_type_id;
// 		if !ctx.types_match(condition_type, bool_type) {
// 			ctx.error("if condition must be boolean", Some(entry.condition.span()));
// 		}

// 		// Synthesise body
// 		let body_type = synthesise_block(ctx, &entry.body);
// 		result_type = body_type; // TODO: Unify branch types
// 	}

// 	// Process else branch if present
// 	if let Some(else_body) = &if_else_chain.else_body {
// 		let else_type = synthesise_block(ctx, else_body);
// 		result_type = else_type; // TODO: Unify with other branches
// 	}

// 	result_type
// }

// pub fn synthesise_match<'a, 'b>(ctx: &mut Context<'a, 'b>, match_: &'b Match) -> checker::Result<TypeId> {
// 	// Synthesise the scrutinee
// 	let scrutinee_type = synthesise_expression(ctx, &match_.scrutinee);

// 	let mut result_type = ctx.type_store.void_type_id;

// 	// Process each arm
// 	for arm in &match_.arms {
// 		// TODO: Pattern matching synthesis
// 		// For now, just synthesise the body
// 		let body_type = synthesise_expression(ctx, &arm.body);
// 		result_type = body_type; // TODO: Unify arm types
// 	}

// 	result_type
// }

// pub fn synthesise_while<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	while_: &'a ast::Node<ast::While>,
// ) -> checker::Result<TypeId> {
// 	// Enter loop scope
// 	let guard = ctx.enter_loop_scope(while_.label.as_ref().map(|n| n.item));

// 	// Check condition is boolean
// 	let condition_type = synthesise_expression(ctx, &while_.condition);
// 	let bool_type = ctx.type_store.bool_type_id;
// 	if !ctx.types_match(condition_type, bool_type) {
// 		ctx.error("while condition must be boolean", Some(while_.condition.span()));
// 	}

// 	// Synthesise body
// 	synthesise_block(ctx, &while_.body);

// 	// Exit loop scope
// 	ctx.exit_scope(guard);

// 	// While loops return void
// 	ctx.type_store.void_type_id
// }

// pub fn synthesise_for<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	for_: &'a ast::Node<ast::For>,
// ) -> checker::Result<TypeId> {
// 	// Enter loop scope
// 	let guard = ctx.enter_loop_scope(for_.label.as_ref().map(|n| n.item));

// 	// TODO: Synthesise iterator and pattern

// 	// Synthesise body
// 	synthesise_block(ctx, &for_.body);

// 	// Exit loop scope
// 	ctx.exit_scope(guard);

// 	// For loops return void
// 	ctx.type_store.void_type_id
// }

// pub fn synthesise_check_is<'a, 'b>(ctx: &mut Context<'a, 'b>, check_is: &'b CheckIs) -> checker::Result<TypeId> {
// 	// Synthesise the expression being checked
// 	let expr_type = synthesise_expression(ctx, &check_is.value);

// 	// TODO: Pattern type checking

// 	// is expressions return bool
// 	ctx.type_store.bool_type_id
// }

// pub fn synthesise_command<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	command: &'a ast::Node<ast::Command>,
// ) -> checker::Result<TypeId> {
// 	// Commands are like macro invocations
// 	// TODO: Implement command synthesis
// 	ctx.type_store.void_type_id
// }
