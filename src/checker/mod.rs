use crate::{ast, context};
use context::Context;
mod attributes;
mod expressions;
mod functions;
mod patterns;
mod statements;
mod types;

pub fn check_file<'a, 'b>(ctx: &mut Context<'a, 'b>, ast_file: &ast::File<'a>) {
	check_block(ctx, &ast_file.block);
}

fn check_block<'a, 'b>(ctx: &mut Context<'a, 'b>, block: &ast::Block<'a>) {
	for statement in block.statements.iter() {
		check_statement(ctx, statement);
	}
}

fn check_statement<'a, 'b>(ctx: &mut Context<'a, 'b>, statement: &ast::Statement<'a>) {
	match statement {
		ast::Statement::Function(function) => functions::check_function(ctx, function),
		_ => todo!(),
	}
}
