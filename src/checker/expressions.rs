use crate::{
	ast::{self, Node},
	context::Context,
	hir,
};

pub fn check_expression<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	expr_node: &ast::Node<ast::Expression<'a>>,
) -> hir::Expression<'b> {
	let span = expr_node.span;
	match &expr_node.item {
		ast::Expression::NumberLiteral(literal) => check_number_literal(ctx, literal),
		ast::Expression::BooleanLiteral(bool) => check_boolean_literal(ctx, Node::new(*bool, span)),
		ast::Expression::StringLiteral(literal) => check_string_literal(ctx, Node::new(literal, span)),
		ast::Expression::FormatStringLiteral(literal) => {
			check_format_string_literal(ctx, Node::new(literal, span))
		}
		ast::Expression::Call(call) => todo!(),
		ast::Expression::Read(read) => todo!(),
		ast::Expression::UnaryOperation(unary_operation) => todo!(),
		ast::Expression::BinaryOperation(binary_operation) => todo!(),
		_ => todo!(),
	}
}

pub fn check_number_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	literal: &ast::NumberLiteral,
) -> hir::Expression<'b> {
	let span = literal.value.span;
	let value = literal.value.item.clone();
	let number_value = hir::NumberValue::new(value, span);

	let type_id = ctx.type_store.builtin.number;

	let kind = hir::ExpressionKind::NumberValue(number_value);

	hir::Expression::new(type_id, kind, span)
}

pub fn check_boolean_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	literal: ast::Node<bool>,
) -> hir::Expression<'b> {
	let span = literal.span;

	let type_id = ctx.type_store.builtin.bool;

	let kind = hir::ExpressionKind::BooleanLiteral(literal.item);

	hir::Expression::new(type_id, kind, span)
}

pub fn check_string_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	literal: ast::Node<&ast::StringLiteral<'a>>,
) -> hir::Expression<'b> {
	let span = literal.span;
	let value = literal.item.value.clone();
	let string_value = hir::StringLiteral { value };

	let type_id = ctx.type_store.builtin.string;

	let kind = hir::ExpressionKind::StringLiteral(string_value);

	hir::Expression::new(type_id, kind, span)
}

pub fn check_format_string_literal<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	literal: ast::Node<&ast::FormatStringLiteral<'a>>,
) -> hir::Expression<'b> {
	let span = literal.span;
	let value = literal.item.clone();
	// let format_string_value = hir::FormatStringLiteral;
	let format_string_literal = hir::FormatStringLiteral::new();
	let type_id = ctx.type_store.builtin.format_string;

	for format_string_item in literal.item.items.iter() {
		// format_string_literal.add_argument(check_expression(ctx, arg));
	}

	let kind = hir::ExpressionKind::FormatStringLiteral(format_string_literal);

	hir::Expression::new(type_id, kind, span)
}

pub fn check_call<'a, 'b>(
	ctx: &mut Context<'a, 'b>,
	call: ast::Node<&ast::Call<'a>>,
) -> hir::Expression<'b> {
	let span = call.span;
	// let callee = check_expression(ctx, call.item.);
	// let arguments = call.item.arguments.iter().map(|arg| check_expression(ctx, arg)).collect();

	// let type_id = ctx.type_store.builtin.function;

	// let kind = hir::ExpressionKind::Call(callee, arguments);

	todo!()
	// hir::Expression::new(type_id, kind, span)
}
