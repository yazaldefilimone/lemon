use crate::symbols::{self, Symbol};

#[derive(Debug)]
pub struct Scope<'a, 'b> {
	pub symbols: Vec<Symbol<'a>>,
	pub readables: &'b mut symbols::Readables<'a>,
}

impl<'a, 'b> Scope<'a, 'b> {
	pub fn new(readables: &'b mut symbols::Readables<'a>) -> Self {
		Self { symbols: Vec::new(), readables }
	}
}

#[derive(Debug)]
pub struct Scopes<'a, 'b> {
	items: Vec<Scope<'a, 'b>>,
	pointer: usize,
}

impl<'a, 'b> Scopes<'a, 'b> {
	pub fn new(readables: &'b mut symbols::Readables<'a>) -> Self {
		Self { items: vec![Scope::new(readables)], pointer: 0 }
	}

	pub fn enter(&mut self, readables: &'b mut symbols::Readables<'a>) {
		self.items.push(Scope::new(readables));
		self.pointer = self.items.len() - 1;
	}

	pub fn exit(&mut self) {
		if self.items.len() <= 1 {
			return;
		}
		self.items.pop();
		self.pointer = self.items.len() - 1;
	}

	pub fn current_mut(&mut self) -> &mut Scope<'a, 'b> {
		&mut self.items[self.pointer]
	}

	pub fn current(&self) -> &Scope<'a, 'b> {
		&self.items[self.pointer]
	}

	pub fn add_symbol(&mut self, symbol: Symbol<'a>) {
		self.current_mut().symbols.push(symbol);
	}

	pub fn find_symbol(&self, name: &str) -> Option<&Symbol<'a>> {
		for scope in self.items.iter().rev() {
			if let Some(symbol) = scope.symbols.iter().find(|s| s.name == name) {
				return Some(symbol);
			}
		}
		None
	}
}
