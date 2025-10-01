use super::scope_manager::{ScopeGuard, ScopeKind, ScopeManager};
use crate::ast::Node;
use crate::checker::types;
use crate::checker::types::{TypeEntry, TypeEntryKind, TypeId};
use crate::cli::CompilerOptions;
use crate::messages;
use crate::span::Span;
use crate::symbols::{ReadableKind, Readables, Symbol, SymbolKind};

#[derive(Debug)]
pub struct Context<'a, 'b> {
	pub file_index: u32,
	pub compiler_options: &'b CompilerOptions,
	pub messages: &'b mut messages::Messages<'a>,

	pub type_store: &'b mut types::store::TypeStore<'a>,
	pub function_store: &'b types::store::FunctionStore<'a>,

	scope_manager: ScopeManager<'a>,

	pub readables: &'b mut Readables<'a>,
	readables_checkpoint: usize,
}

impl<'a, 'b> Drop for Context<'a, 'b> {
	fn drop(&mut self) {
		self.readables.readables.truncate(self.readables_checkpoint);
	}
}

impl<'a, 'b> Context<'a, 'b> {
	pub fn new(
		file_index: u32,
		compiler_options: &'b CompilerOptions,
		messages: &'b mut messages::Messages<'a>,
		type_store: &'b mut types::store::TypeStore<'a>,
		function_store: &'b types::store::FunctionStore<'a>,
		readables: &'b mut Readables<'a>,
	) -> Self {
		let readables_checkpoint = readables.overall_len();
		Self {
			file_index,
			compiler_options,
			messages,
			type_store,
			function_store,
			scope_manager: ScopeManager::new(),
			readables,
			readables_checkpoint,
		}
	}

	pub fn enter_block_scope(&mut self) -> ScopeGuard {
		self.scope_manager.enter_scope(ScopeKind::Block)
	}

	pub fn enter_function_scope(&mut self, name: &'a str, return_type: TypeId) -> ScopeGuard {
		let kind = ScopeKind::function(name, return_type, false);
		self.scope_manager.enter_scope(kind)
	}

	pub fn enter_loop_scope(&mut self, label: Option<&'a str>) -> ScopeGuard {
		self.scope_manager.enter_scope(ScopeKind::loop_scope(label))
	}

	pub fn enter_closure_scope(&mut self, return_type: Option<TypeId>) -> ScopeGuard {
		self.scope_manager.enter_scope(ScopeKind::closure(return_type))
	}

	pub fn exit_scope(&mut self, guard: ScopeGuard) {
		self.scope_manager.report_unused(self.messages);
		self.scope_manager.exit_scope(guard);
	}

	pub fn current_return_type(&self) -> Option<TypeId> {
		self.scope_manager.in_function_scope()
	}

	pub fn in_loop(&self) -> bool {
		self.scope_manager.in_loop_scope().is_some()
	}

	pub fn mark_all_paths_return(&mut self) {
		self.scope_manager.mark_all_paths_return();
	}

	pub fn add_symbol(&mut self, symbol: Symbol<'a>) -> Result<(), ()> {
		self.scope_manager.add_symbol(symbol, self.messages)
	}

	pub fn lookup_symbol(&mut self, name: &str) -> Option<Symbol<'a>> {
		self.scope_manager.lookup_symbol(name, true)
	}

	pub fn add_variable(
		&mut self,
		name: Node<&'a str>,
		type_id: TypeId,
		mutable: bool,
		used: bool,
	) -> usize {
		let readable_index = self.readables.push(
			name.item,
			type_id,
			if mutable { ReadableKind::Mut } else { ReadableKind::Let },
			false,
		);

		if name.item != "_" {
			let symbol = Symbol::variable(name.item, readable_index, mutable, name.span);
			let mut symbol = symbol;
			symbol.used = used;
			self.add_symbol(symbol).ok();
		}

		readable_index
	}

	pub fn get_symbol_type(&self, symbol: &Symbol) -> Option<TypeId> {
		match symbol.kind {
			SymbolKind::BuiltinType { type_id, .. } => Some(type_id),
			SymbolKind::Let { readable_index } | SymbolKind::Mut { readable_index } => {
				self.readables.get(readable_index).map(|r| r.type_id)
			}
			_ => None,
		}
	}

	pub fn create_type(&mut self, kind: TypeEntryKind) -> TypeId {
		self.type_store.create_type(kind)
	}

	pub fn get_type(&self, type_id: TypeId) -> &TypeEntry {
		self.type_store.get_type(type_id)
	}

	pub fn get_type_mut(&mut self, type_id: TypeId) -> &mut TypeEntry {
		self.type_store.get_type_mut(type_id)
	}

	pub fn types_match(&self, a: TypeId, b: TypeId) -> bool {
		self.type_store.direct_match(a, b)
	}

	pub fn create_array_type(&mut self, item_type: TypeId, length: u64) -> TypeId {
		use crate::checker::types::Array;
		let array = Array { item_type_id: item_type, length, array_type_index: 0 };
		self.create_type(TypeEntryKind::Array(array))
	}

	pub fn create_slice_type(&mut self, item_type: TypeId, mutable: bool) -> TypeId {
		use crate::checker::types::Slice;
		let slice = Slice { item_type_id: item_type, mutable };
		self.create_type(TypeEntryKind::Slice(slice))
	}

	pub fn create_pointer_type(&mut self, target_type: TypeId, mutable: bool) -> TypeId {
		use crate::checker::types::Pointer;
		let pointer = Pointer { type_id: target_type, mutable };
		self.create_type(TypeEntryKind::Pointer(pointer))
	}

	pub fn message(&mut self, message: messages::Message) {
		self.messages.message(message);
	}

	pub fn error(&mut self, text: impl Into<String>, span: Option<Span>) {
		let msg = crate::error!("{}", text.into());
		let msg = if let Some(span) = span { msg.with_span(span) } else { msg };
		self.message(msg);
	}

	pub fn warning(&mut self, text: impl Into<String>, span: Option<Span>) {
		let msg = crate::warning!("{}", text.into());
		let msg = if let Some(span) = span { msg.with_span(span) } else { msg };
		self.message(msg);
	}

	pub fn has_errors(&self) -> bool {
		self.messages.has_errors()
	}
}
