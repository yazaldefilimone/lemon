use crate::{checker::types::TypeId, span::Span};

#[derive(Debug, Clone)]
pub struct Symbol<'a> {
	pub name: &'a str,
	pub kind: SymbolKind,
	pub span: Option<Span>,
	pub used: bool,
	pub imported: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
	BuiltinType { type_id: TypeId, methods_index: usize },
	UserType { shape_index: usize, methods_index: usize },
	UserTypeGeneric { shape_index: usize, generic_index: usize },
	FunctionGeneric { function_shape_index: usize, generic_index: usize },
	Function { function_shape_index: usize },
	Const { constant_index: usize },
	Static { static_index: usize },
	Let { readable_index: usize },
	Mut { readable_index: usize },
}

impl std::fmt::Display for SymbolKind {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let name = match self {
			SymbolKind::BuiltinType { .. } => "a built in type",
			SymbolKind::UserType { .. } => "a type",
			SymbolKind::UserTypeGeneric { .. } => "a type generic parameter",
			SymbolKind::FunctionGeneric { .. } => "a function generic parameter",
			SymbolKind::Function { .. } => "a function",
			SymbolKind::Const { .. } => "a constant",
			SymbolKind::Static { .. } => "a static",
			SymbolKind::Let { .. } => "an immutable binding",
			SymbolKind::Mut { .. } => "a mutable binding",
		};

		f.write_str(name)
	}
}
