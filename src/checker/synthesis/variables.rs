use crate::{checker::context::Context, messages::Messages};

pub fn synthesise_let<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	declaration: &'b crate::ast::Let,
) {
	todo!("synthesise_let_declaration");
}

pub fn synthesise_constant<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	declaration: &'b crate::ast::Constant,
) {
	todo!("synthesise_const_declaration");
}
