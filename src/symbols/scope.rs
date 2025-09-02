use crate::{messages::Messages, symbols::Symbol, warning};

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
