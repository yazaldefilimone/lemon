use crate::{checker::context::Context, messages::Messages};

pub fn synthesise_function<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	function: &'b crate::ast::Function,
) {
	todo!("synthesise_function");
}

pub fn synthesise_function_parameters<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	parameters: &'b [crate::ast::Parameter],
) {
	for parameter in parameters {
		todo!("synthesise_function_parameters");
	}
}

pub fn synthesise_anonymous_function<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	function: &'b crate::ast::Function,
) {
	todo!("synthesise_anonymous_function");
}

pub fn synthesise_function_return_type<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	return_type: &'b crate::ast::Type,
) {
	todo!("synthesise_function_return_type");
}

pub fn synthesise_function_body<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	body: &'b crate::ast::Block,
) {
	todo!("synthesise_function_body");
}

pub fn synthesise_type_parameters<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	type_parameters: &'b [crate::ast::Parameter],
) {
	for type_parameter in type_parameters {
		todo!("synthesise_type_parameters");
	}
}
