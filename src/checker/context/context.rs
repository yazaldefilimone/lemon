use super::store::FunctionStore;
use super::store::TypeStore;
use crate::ast::Node;
use crate::checker::types::TypeId;
use crate::cli::CompilerOptions;
use crate::hir;
use crate::messages::Message;
use crate::messages::Messages;
use crate::symbols::scope::SymbolsScope;
use crate::symbols::ReadableKind;
use crate::symbols::Readables;
use crate::symbols::Symbol;
use crate::symbols::SymbolKind;

#[derive(Debug)]
pub struct Context<'a, 'b> {
	pub file_index: u32,

	pub compiler_options: &'b CompilerOptions,

	pub scope_index: usize,
	pub next_scope_index: &'b mut usize,
	pub messages: &'b mut Messages<'a>,
	pub return_type: Option<TypeId>,

	pub type_store: &'b mut TypeStore<'a>,
	pub function_store: &'b FunctionStore<'a>,

	pub symbols_scope: SymbolsScope<'a, 'b>,

	pub readables: &'b mut Readables<'a>,

	pub initial_readables_starting_index: usize,
	pub initial_readables_overall_len: usize,
	pub function_initial_symbols_length: usize,
}

impl<'a, 'b> Drop for Context<'a, 'b> {
	fn drop(&mut self) {
		self.readables.readables.truncate(self.initial_readables_overall_len);
		self.readables.starting_index = self.initial_readables_starting_index;
	}
}

impl<'a, 'b> Context<'a, 'b> {
	pub fn child_scope<'s>(&'s mut self) -> Context<'a, 's> {
		let scope_index = *self.next_scope_index;
		*self.next_scope_index += 1;

		let initial_readables_starting_index = self.readables.starting_index;
		let initial_readables_overall_len = self.readables.overall_len();

		Context {
			file_index: self.file_index,
			compiler_options: self.compiler_options,
			scope_index,
			next_scope_index: self.next_scope_index,
			messages: self.messages,
			return_type: None,
			type_store: self.type_store,
			function_store: self.function_store,
			symbols_scope: self.symbols_scope.child_scope(),
			function_initial_symbols_length: self.function_initial_symbols_length, // TODO: Wrong?
			readables: self.readables,
			initial_readables_starting_index,
			initial_readables_overall_len,
		}
	}

	pub fn child_scope_for_function<'s>(&'s mut self, return_type: TypeId) -> Context<'a, 's> {
		let scope_index = *self.next_scope_index;
		*self.next_scope_index += 1;

		let initial_readables_starting_index = self.readables.starting_index;
		self.readables.starting_index = self.readables.overall_len();

		let function_initial_symbols_length = self.symbols_scope.symbols.symbols.len();
		let initial_readables_overall_len = self.readables.overall_len();

		Context {
			file_index: self.file_index,
			compiler_options: self.compiler_options,
			scope_index,
			next_scope_index: self.next_scope_index,
			messages: self.messages,
			return_type: Some(return_type),
			type_store: self.type_store,
			function_store: self.function_store,
			symbols_scope: self.symbols_scope.child_scope(),
			function_initial_symbols_length,
			readables: self.readables,
			initial_readables_starting_index,
			initial_readables_overall_len,
		}
	}

	pub fn message(&mut self, message: Message) {
		self.messages.message(message);
	}

	pub fn push_symbol(&mut self, symbol: Symbol<'a>) {
		let scope_start = self.function_initial_symbols_length;
		self.symbols_scope.symbols.push_symbol(self.messages, scope_start, symbol);
	}

	pub fn push_readable(
		&mut self,
		name: Node<&'a str>,
		type_id: TypeId,
		kind: ReadableKind,
		used: bool,
		is_pointer_access_mutable: bool,
	) -> usize {
		let readable_index = self.readables.push(name.item, type_id, kind, is_pointer_access_mutable);

		let span = Some(name.span);
		let name = name.item;
		let kind = match kind {
			ReadableKind::Let => SymbolKind::Let { readable_index },
			ReadableKind::Mut => SymbolKind::Mut { readable_index },
		};

		if name != "_" {
			self.push_symbol(Symbol { name, kind, span, used, imported: false });
		}

		readable_index
	}

	pub fn is_formattable(self, type_store: &TypeStore, expression: &hir::Expression) -> bool {
		todo!()
	}
}
