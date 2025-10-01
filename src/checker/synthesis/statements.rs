use crate::checker::{context::Context, types::TypeId};
use crate::{ast, checker};

pub fn synthesise_statement<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	statement: &'a ast::Statement,
) -> checker::Result<TypeId> {
	match statement {
		ast::Statement::Let(let_decl) => super::synthesise_let(ctx, let_decl),
		ast::Statement::Const(const_decl) => super::synthesise_constant(ctx, const_decl),
		ast::Statement::Expression(expr) => super::synthesise_expression(ctx, expr),
		ast::Statement::Function(function) => super::synthesise_function(ctx, function),
		_ => unimplemented!(),
	}
}

pub fn synthesise_block<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	block: &'a ast::Block,
) -> checker::Result<TypeId> {
	// Enter new block scope
	let guard = ctx.enter_block_scope();

	let mut last_type = ctx.type_store.void_type_id;

	// Process all statements
	for statement in &block.statements {
		last_type = super::synthesise_statement(ctx, statement)?;
	}

	// Exit scope
	ctx.exit_scope(guard);

	Ok(last_type)
}
// pub fn synthesise_return<'a, 'b>(ctx: &mut Context<'a, 'b>, ret: &'b ast::Return) -> TypeId {
// 	// Check if we're in a function
// 	let expected_return_type = if let Some(return_type) = ctx.current_return_type() {
// 		return_type
// 	} else {
// 		ctx.error("return statement outside of function", Some(ret.span));
// 		return ctx.type_store.noreturn_type_id;
// 	};

// 	// Synthesise return value if present
// 	let actual_return_type = if let Some(value) = &ret.value {
// 		synthesis::synthesise_expression(ctx, value)
// 	} else {
// 		ctx.type_store.void_type_id
// 	};

// 	// Type check return value
// 	if !ctx.types_match(expected_return_type, actual_return_type) {
// 		ctx.error(format!("return type mismatch"), Some(ret.span));
// 	}

// 	// Mark that we've seen a return
// 	ctx.mark_all_paths_return();

// 	// Return statements don't produce a value
// 	ctx.type_store.noreturn_type_id
// }

// pub fn synthesise_break<'a, 'b>(ctx: &mut Context<'a, 'b>, brk: &'b crate::ast::Break) -> TypeId {
// 	// Check if we're in a loop
// 	if !ctx.in_loop() {
// 		ctx.error("break statement outside of loop", Some(brk.span));
// 	}

// 	// TODO: Handle labeled breaks

// 	// Break statements don't produce a value
// 	ctx.type_store.noreturn_type_id
// }

// pub fn synthesise_continue<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	cont: &'b crate::ast::Continue,
// ) -> TypeId {
// 	// Check if we're in a loop
// 	if !ctx.in_loop() {
// 		ctx.error("continue statement outside of loop", Some(cont.span));
// 	}

// 	// TODO: Handle labeled continues

// 	// Continue statements don't produce a value
// 	ctx.type_store.noreturn_type_id
// }

// pub fn synthesise_command<'a, 'b>(
// 	ctx: &mut Context<'a, 'b>,
// 	command: &'b ast::Node<ast::Command>,
// ) -> TypeId {
// 	// Commands are like macro invocations
// 	// TODO: Implement command synthesis properly
// 	ctx.type_store.void_type_id
// }
