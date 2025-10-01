use crate::{checker::types::TypeId, messages::Messages, symbols::Symbol};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ScopeManager<'a> {
	scopes: Vec<Scope<'a>>,
	current_scope_index: usize,
}

impl<'a> ScopeManager<'a> {
	pub fn new() -> Self {
		let global_scope = Scope::new(ScopeKind::Global);
		Self { scopes: vec![global_scope], current_scope_index: 0 }
	}

	pub fn enter_scope(&mut self, kind: ScopeKind<'a>) -> ScopeGuard {
		let parent_index = self.current_scope_index;
		let scope = Scope::with_parent(kind, parent_index);
		self.scopes.push(scope);
		self.current_scope_index = self.scopes.len() - 1;

		ScopeGuard { manager_index: self.current_scope_index, parent_index }
	}

	pub fn exit_scope(&mut self, guard: ScopeGuard) {
		assert_eq!(self.current_scope_index, guard.manager_index);
		self.current_scope_index = guard.parent_index;
	}

	pub fn add_symbol(&mut self, symbol: Symbol<'a>, messages: &mut Messages<'a>) -> Result<(), ()> {
		let current_scope = &mut self.scopes[self.current_scope_index];

		// check for duplicates if symbol can't shadow
		if !symbol.kind.can_shadow() {
			if let Some(existing) = current_scope.find_symbol(symbol.name) {
				let message = crate::error!("duplicate symbol `{}`", symbol.name)
					.with_span_if_some(symbol.span)
					.with_note_if_some(existing.span, "original symbol here");
				messages.message(message);
				return Err(());
			}
		}

		current_scope.add_symbol(symbol);
		Ok(())
	}

	pub fn lookup_symbol(&mut self, name: &str, mark_used: bool) -> Option<Symbol<'a>> {
		let mut scope_index = Some(self.current_scope_index);

		while let Some(index) = scope_index {
			let scope = self.scopes.get_mut(index)?;

			if let Some(symbol) = scope.find_symbol_mut(name) {
				if mark_used {
					symbol.used = true;
				}
				// TODO: return a reference to the symbol?
				return Some(symbol.clone());
			}
			scope_index = scope.parent_index;
		}

		None
	}

	pub fn current_scope_kind(&self) -> &ScopeKind<'a> {
		&self.scopes[self.current_scope_index].kind
	}

	pub fn in_function_scope(&self) -> Option<TypeId> {
		let mut scope_index = Some(self.current_scope_index);

		while let Some(index) = scope_index {
			if let ScopeKind::Function { return_type, .. } = &self.scopes[index].kind {
				return Some(*return_type);
			}
			scope_index = self.scopes[index].parent_index;
		}

		None
	}

	pub fn in_loop_scope(&self) -> Option<Option<&'a str>> {
		let mut scope_index = Some(self.current_scope_index);

		while let Some(index) = scope_index {
			if let ScopeKind::Loop { label, .. } = &self.scopes[index].kind {
				return Some(*label);
			}
			scope_index = self.scopes[index].parent_index;
		}

		None
	}

	pub fn report_unused(&self, messages: &mut Messages<'a>) {
		let current_scope = &self.scopes[self.current_scope_index];
		for symbol in &current_scope.symbols {
			if !symbol.used && !symbol.name.starts_with('_') {
				messages.message(
					crate::warning!("unused symbol `{}`", symbol.name).with_span_if_some(symbol.span),
				);
			}
		}
	}

	pub fn current_scope_symbols(&self) -> &[Symbol<'a>] {
		&self.scopes[self.current_scope_index].symbols
	}

	pub fn mark_all_paths_return(&mut self) {
		let mut scope_index = Some(self.current_scope_index);

		while let Some(index) = scope_index {
			if let ScopeKind::Function { all_paths_return, .. } = &mut self.scopes[index].kind {
				*all_paths_return = true;
				break;
			}
			scope_index = self.scopes[index].parent_index;
		}
	}
}

pub struct ScopeGuard {
	manager_index: usize,
	parent_index: usize,
}

#[derive(Debug)]
struct Scope<'a> {
	kind: ScopeKind<'a>,
	symbols: Vec<Symbol<'a>>,
	symbol_map: HashMap<&'a str, usize>,
	parent_index: Option<usize>,
}

impl<'a> Scope<'a> {
	fn new(kind: ScopeKind<'a>) -> Self {
		Self { kind, symbols: Vec::new(), symbol_map: HashMap::new(), parent_index: None }
	}

	fn with_parent(kind: ScopeKind<'a>, parent: usize) -> Self {
		Self { kind, symbols: Vec::new(), symbol_map: HashMap::new(), parent_index: Some(parent) }
	}

	fn add_symbol(&mut self, symbol: Symbol<'a>) {
		let index = self.symbols.len();
		self.symbol_map.insert(symbol.name, index);
		self.symbols.push(symbol);
	}

	fn find_symbol(&self, name: &str) -> Option<&Symbol<'a>> {
		self.symbol_map.get(name).map(|&i| &self.symbols[i])
	}

	fn find_symbol_mut(&mut self, name: &str) -> Option<&mut Symbol<'a>> {
		self.symbol_map.get(name).map(|&i| &mut self.symbols[i])
	}
}

#[derive(Debug)]
pub enum ScopeKind<'a> {
	Global,
	Module { name: &'a str },
	Function { name: &'a str, return_type: TypeId, all_paths_return: bool, is_async: bool },
	Closure { return_type: Option<TypeId>, captures: Vec<Symbol<'a>> },
	Block,
	Loop { label: Option<&'a str> },
	MatchArm,
}

impl<'a> ScopeKind<'a> {
	pub fn function(name: &'a str, return_type: TypeId, is_async: bool) -> Self {
		ScopeKind::Function { name, return_type, all_paths_return: false, is_async }
	}

	pub fn closure(return_type: Option<TypeId>) -> Self {
		ScopeKind::Closure { return_type, captures: Vec::new() }
	}

	pub fn loop_scope(label: Option<&'a str>) -> Self {
		ScopeKind::Loop { label }
	}
}
