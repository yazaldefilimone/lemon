use crate::{checker::context::Context, messages::Messages};

pub fn synthesise_number_literal<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	number_literal: &'b crate::ast::NumberLiteral,
) {
	todo!("synthesise_number_literal");
}

pub fn synthesise_boolean_literal<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	boolean_literal: &'b bool,
) {
	todo!("synthesise_boolean_literal");
}

pub fn synthesise_string_literal<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	string_literal: &'b crate::ast::StringLiteral,
) {
	todo!("synthesise_string_literal");
}

pub fn synthesise_format_string_literal<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	format_string_literal: &'b crate::ast::FormatStringLiteral,
) {
	todo!("synthesise_format_string_literal");
}

// array
pub fn synthesise_array_literal<'a, 'b>(
	context: &mut Context<'a, 'b>,
	messages: &mut Messages,
	array_literal: &'b crate::ast::ArrayLiteral,
) {
	todo!("synthesise_array_literal");
}
