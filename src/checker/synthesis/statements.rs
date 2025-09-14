use crate::{checker::context::Context, messages::Messages};

pub fn synthesise_statement<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	statement: &'b crate::ast::Statement,
) {
	todo!("synthesise_statement");
}

pub fn synthesise_let<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	let_: &'b crate::ast::Let,
) {
	todo!("synthesise_let");
}

pub fn synthesise_constant<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	constant: &'b crate::ast::Constant,
) {
	todo!("synthesise_constant");
}

pub fn synthesise_command<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	command: &'b crate::ast::Command,
) {
	todo!("synthesise_command");
}

// pub fn synthesise_return<'a, 'b>(
// 	context: &mut Context<'a, 'b>,
// 	messages: &mut Messages,
// 	return_: &'b crate::ast::Return,
// ) {
// 	todo!("synthesise_return");
// }

// pub fn synthesise_break<'a, 'b>(
// 	context: &mut Context<'a, 'b>,
// 	messages: &mut Messages,
// 	break_: &'b crate::ast::Break,
// ) {
// 	todo!("synthesise_break");
// }

// pub fn synthesise_continue<'a, 'b>(
// 	context: &mut Context<'a, 'b>,
// 	messages: &mut Messages,
// 	continue_: &'b crate::ast::Continue,
// ) {
// 	todo!("synthesise_continue");
// }

// pub fn synthesise_if<'a, 'b>(
// 	context: &mut Context<'a, 'b>,
// 	messages: &mut Messages,
// 	if_: &'b crate::ast::If,
// ) {
// 	todo!("synthesise_if");
// }

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

pub fn synthesise_match<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	match_: &'b crate::ast::Match,
) {
	todo!("synthesise_match");
}
