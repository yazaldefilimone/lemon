use crate::{
	ast::{Node, Statement},
	loader::SourceFile,
};

#[derive(Debug)]
pub struct File<'a> {
	pub source_file: &'a SourceFile,
	pub block: Block<'a>,
}

#[derive(Debug)]
pub struct Block<'a> {
	pub statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub enum Type<'a> {
	Void,

	Pointer {
		pointee: &'a Node<Type<'a>>,
		mutable: bool,
	},

	Array {
		item: &'a Node<Type<'a>>,
		length: u64, // TODO: Allow array length to be a proper expression
	},

	Slice {
		pointee: &'a Node<Type<'a>>,
		mutable: bool,
	},

	Path {
		// path_segments: Node<PathSegments<'a>>,
		type_arguments: &'a [Node<Type<'a>>],
		dot_access_chain: &'a [Node<&'a str>],
	},
}
