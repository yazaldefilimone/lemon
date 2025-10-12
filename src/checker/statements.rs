use crate::{ast, context, hir};
use context::Context;

pub fn check_block<'a, 'b>(ctx: &mut Context<'a, 'b>, block: &ast::Block<'a>) -> hir::Block<'a> {
	let mut hir_block = hir::Block::new();
	for statement in block.statements.iter() {
		if let Some(hir_statement) = check_statement(ctx, statement) {
			hir_block.statements.push(hir_statement);
		}
	}
	hir_block
}

pub fn check_statement<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	statement: &ast::Statement<'a>,
) -> Option<hir::Statement<'a>> {
	match statement {
		ast::Statement::Function(function) => {
			super::functions::check_function(ctx, function);
			None
		}
		ast::Statement::Expression(expression) => {
			let expression = super::expressions::check_expression(ctx, expression);
			let span = expression.span.clone();
			let kind = hir::StatementKind::Expression(expression);
			Some(hir::Statement::new(kind, span))
		}
		_ => todo!(),
	}
}
