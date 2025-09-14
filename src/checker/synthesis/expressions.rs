use crate::{checker::context::Context, messages::Messages};

pub fn synthesise_expression<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	expression: &'b crate::ast::Expression,
) {
	todo!("synthesise_expression");
}

pub fn synthesise_if_else_chain<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	if_else_chain: &'b crate::ast::IfElseChain,
) {
	todo!("synthesise_if_else_chain");
}

pub fn synthesise_match<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	match_: &'b crate::ast::Match,
) {
	todo!("synthesise_match");
}

pub fn synthesise_while<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	while_: &'b crate::ast::While,
) {
	todo!("synthesise_while");
}

pub fn synthesise_for<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	for_: &'b crate::ast::For,
) {
	todo!("synthesise_for");
}

pub fn synthesise_check_is<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	check_is: &'b crate::ast::CheckIs,
) {
	todo!("synthesise_check_is");
}

pub fn synthesise_command<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	command: &'b crate::ast::Command,
) {
	todo!("synthesise_command");
}
