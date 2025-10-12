use crate::{reference::Ref, root_layers::RootLayer, span::Span, types::TypeId};

#[derive(Debug, Clone)]
pub struct Symbol<'a> {
	pub name: &'a str,
	pub kind: SymbolKind<'a>,
	pub span: Option<Span>,
	pub used: bool,
	pub imported: bool,
}

impl<'a> Symbol<'a> {
	pub fn new(name: &'a str, kind: SymbolKind<'a>, span: Option<Span>) -> Self {
		let mut symbol = Self { name, kind, span, used: false, imported: false };
		if name.starts_with('_') {
			symbol.used = true;
		}
		symbol
	}

	pub fn builtin_type(name: &'a str, type_id: TypeId, methods_index: usize) -> Self {
		Self::new(name, SymbolKind::BuiltinType { type_id, methods_index }, None)
	}

	pub fn function(name: &'a str, function_shape_index: usize, span: Span) -> Self {
		Self::new(name, SymbolKind::Function { function_shape_index }, Some(span))
	}

	pub fn variable(name: &'a str, readable_index: usize, mutable: bool, span: Span) -> Self {
		let kind = SymbolKind::Variable { readable_index, mutable };
		Self::new(name, kind, Some(span))
	}

	pub fn is_type(&self) -> bool {
		matches!(
			self.kind,
			SymbolKind::BuiltinType { .. }
				| SymbolKind::UserType { .. }
				| SymbolKind::UserTypeGeneric { .. }
		)
	}

	pub fn is_value(&self) -> bool {
		matches!(
			self.kind,
			SymbolKind::Variable { .. }
				| SymbolKind::Const { .. }
				| SymbolKind::Static { .. }
				| SymbolKind::Function { .. }
		)
	}

	pub fn type_id(&self) -> Option<TypeId> {
		match self.kind {
			SymbolKind::BuiltinType { type_id, .. } => Some(type_id),
			_ => None,
		}
	}
}

#[derive(Debug, Clone)]
pub enum SymbolKind<'a> {
	BuiltinType { type_id: TypeId, methods_index: usize },
	UserType { shape_index: usize, methods_index: usize },
	UserTypeGeneric { shape_index: usize, generic_index: usize },
	FunctionGeneric { function_shape_index: usize, generic_index: usize },
	Function { function_shape_index: usize },
	Const { constant_index: usize },
	Static { static_index: usize },
	Variable { readable_index: usize, mutable: bool },
	Module { layer: Ref<RootLayer<'a>> },
}

impl<'a> SymbolKind<'a> {
	pub fn can_shadow(&self) -> bool {
		matches!(
			self,
			SymbolKind::Variable { .. }
				| SymbolKind::FunctionGeneric { .. }
				| SymbolKind::UserTypeGeneric { .. }
		)
	}
}

impl<'a> std::fmt::Display for SymbolKind<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let name = match self {
			SymbolKind::BuiltinType { .. } => "a built-in type",
			SymbolKind::UserType { .. } => "a type",
			SymbolKind::UserTypeGeneric { .. } => "a type generic parameter",
			SymbolKind::FunctionGeneric { .. } => "a function generic parameter",
			SymbolKind::Function { .. } => "a function",
			SymbolKind::Const { .. } => "a constant",
			SymbolKind::Static { .. } => "a static",
			SymbolKind::Variable { mutable, .. } => {
				if *mutable {
					"a mutable binding"
				} else {
					"an immutable binding"
				}
			}
			SymbolKind::Module { .. } => "an imported module",
		};
		f.write_str(name)
	}
}
