use crate::{
	ast::Node,
	checker::types::store::TypeStore,
	error,
	messages::Messages,
	root_layers::RootLayers,
	symbols::{Symbol, SymbolKind},
	warning,
};

#[derive(Debug)]
pub struct SymbolsScope<'a, 'b> {
	pub initial_symbols_length: usize,
	pub symbols: &'b mut Symbols<'a>,
}

#[derive(Debug, Clone)]
pub struct Symbols<'a> {
	pub symbols: Vec<Symbol<'a>>,
}

impl<'a> Symbols<'a> {
	pub fn new() -> Self {
		Symbols { symbols: Vec::new() }
	}

	pub fn child_scope<'s>(&'s mut self) -> SymbolsScope<'a, 's> {
		let initial_symbols_length = self.symbols.len();
		SymbolsScope { symbols: self, initial_symbols_length }
	}

	pub fn push_symbol(
		&mut self,
		messages: &mut Messages,
		scope_start: usize,
		mut symbol: Symbol<'a>,
	) {
		if !symbol.kind.can_shadow() {
			if let Some(found) = self.find_symbol_matching_name(symbol.name, scope_start, false) {
				let message = error!("duplicate symbol `{}`", symbol.name);
				let message = message
					.with_span_if_some(symbol.span)
					.with_note_if_some(found.span, "original symbol here");
				messages.message(message);
			}
		}

		if symbol.name.starts_with('_') {
			symbol.used = true;
		}

		self.symbols.push(symbol);
	}

	pub fn find_symbol_matching_name(
		&mut self,
		name: &str,
		scope_start: usize,
		mark_used: bool,
	) -> Option<&mut Symbol<'a>> {
		let scope_len = self.symbols.len();
		for (iteration_index, symbol) in self.symbols.iter_mut().rev().enumerate() {
			let symbol_index = scope_len - iteration_index;
			if symbol.name != name {
				continue;
			}

			symbol.used |= mark_used;

			if symbol_index > scope_start {
				match symbol.kind {
					SymbolKind::Let { .. }
					| SymbolKind::Mut { .. }
					| SymbolKind::FunctionGeneric { .. }
					| SymbolKind::UserTypeGeneric { .. } => break,
					_ => {}
				}
			}
			return Some(symbol);
		}

		None
	}

	pub fn lookup_symbol_by_name(
		&mut self,
		messages: &mut Messages,
		root_layers: &RootLayers<'a>,
		type_store: &TypeStore<'a>,
		scope_start: usize,
		name: Node<&'a str>,
	) -> Option<Symbol<'a>> {
		let primatives = &type_store.primative_type_symbols;
		if let Some(found) = primatives.iter().find(|symbol| symbol.name == name.item) {
			return Some(found.clone());
		}

		if let Some(found) = self.find_symbol_matching_name(name.item, scope_start, true) {
			return Some(found.clone());
		}

		if let Some(layer) = root_layers.layer_for_module_name(&name) {
			let kind = SymbolKind::Module { layer };
			let span = Some(name.span);
			let name = name.item;
			let symbol = Symbol { name, kind, span, used: false, imported: false };
			return Some(symbol);
		}

		let error = error!("no symbol `{}` in the current scope", name.item);
		messages.message(error.with_span(name.span));
		None
	}
}

impl<'a, 'b> SymbolsScope<'a, 'b> {
	pub fn child_scope<'s>(&'s mut self) -> SymbolsScope<'a, 's> {
		let initial_symbols_length = self.symbols.symbols.len();
		SymbolsScope { symbols: self.symbols, initial_symbols_length }
	}

	pub fn report_unused(&self, messages: &mut Messages<'a>) {
		let scope = &self.symbols.symbols[self.initial_symbols_length..];
		report_unused(scope, messages);
	}
}

pub fn report_unused<'a>(scope: &[Symbol<'a>], messages: &mut Messages<'a>) {
	for symbol in scope {
		if !symbol.used {
			let warning = warning!("unused symbol `{}`", symbol.name);
			messages.message(warning.with_span_if_some(symbol.span));
		}
	}
}

impl<'a, 'b> Drop for SymbolsScope<'a, 'b> {
	fn drop(&mut self) {
		self.symbols.symbols.truncate(self.initial_symbols_length);
	}
}
