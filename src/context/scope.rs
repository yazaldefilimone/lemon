use crate::symbols::Symbol;

#[derive(Debug, Clone)]
pub struct Scope<'a> {
	pub symbols: Vec<Symbol<'a>>,
	pub readables: Vec<Symbol<'a>>,
}

impl<'a> Scope<'a> {
	pub fn new() -> Self {
		Self { symbols: Vec::new(), readables: Vec::new() }
	}
}

#[derive(Debug)]
pub struct Scopes<'a> {
	items: Vec<Scope<'a>>,
	pointer: usize,
}

impl<'a> Scopes<'a> {
	pub fn new() -> Self {
		Self { items: vec![Scope::new()], pointer: 0 }
	}

	pub fn enter(&mut self) {
		self.items.push(Scope::new());
		self.pointer = self.items.len() - 1;
	}

	pub fn exit(&mut self) {
		if self.items.len() <= 1 {
			return;
		}
		self.items.pop();
		self.pointer = self.items.len() - 1;
	}

	pub fn current_mut(&mut self) -> &mut Scope<'a> {
		&mut self.items[self.pointer]
	}

	pub fn current(&self) -> &Scope<'a> {
		&self.items[self.pointer]
	}

	pub fn add_symbol(&mut self, symbol: Symbol<'a>) {
		self.current_mut().symbols.push(symbol);
	}

	pub fn find_symbol(&self, name: &str) -> Option<&Symbol<'a>> {
		for scope in self.items.iter().rev() {
			if let Some(s) = scope.symbols.iter().find(|s| s.name == name) {
				return Some(s);
			}
		}
		None
	}
}
