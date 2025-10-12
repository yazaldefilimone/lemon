use crate::{cli, context::scope};
use crate::{hir, messages, root_layers, span::Span, store, symbols, types::TypeId};
use store::{function_store, type_store};

#[derive(Debug)]
pub struct Context<'a, 'b> {
	pub compiler_options: &'b cli::CompilerOptions,

	pub messages: &'b mut messages::Messages<'a>,
	pub type_store: &'b mut type_store::TypeStore<'a>,
	pub function_store: &'b mut function_store::FunctionStore<'a>,
	pub constants: &'b mut Vec<hir::ConstantValue<'a>>,

	// pub readables: &'b mut symbols::Readables<'a>,
	pub root_layers: &'b mut root_layers::RootLayers<'a>,
	pub scopes: scope::Scopes<'a, 'b>,
}

impl<'a, 'b> Context<'a, 'b> {
	pub fn new(
		compiler_options: &'b cli::CompilerOptions,
		messages: &'b mut messages::Messages<'a>,
		type_store: &'b mut type_store::TypeStore<'a>,
		function_store: &'b mut function_store::FunctionStore<'a>,
		constants: &'b mut Vec<hir::ConstantValue<'a>>,
		readables: &'b mut symbols::Readables<'a>,
		root_layers: &'b mut root_layers::RootLayers<'a>,
	) -> Self {
		let scopes = scope::Scopes::new(readables);

		Self {
			compiler_options,
			messages,
			scopes,
			type_store,
			function_store,
			constants,
			// readables,
			root_layers,
		}
	}

	pub fn type_name(&mut self, type_id: TypeId) -> String {
		self.type_store.type_name(type_id)
	}

	pub fn message(&mut self, message: messages::Message) {
		self.messages.message(message);
	}

	pub fn error(&mut self, text: impl Into<String>, span: Option<Span>) {
		let message = crate::error!("{}", text.into());
		self.message(message.with_span_if_some(span));
	}

	pub fn warning(&mut self, text: impl Into<String>, span: Option<Span>) {
		let message = crate::warning!("{}", text.into());
		self.message(message.with_span_if_some(span));
	}
}
