use crate::{messages::Messages, span::Span};
use rustc_hash::FxHashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct ExternLocation<'a> {
	pub span: Span,
	pub module_path: &'a PathBuf,
}

impl<'a> ExternLocation<'a> {
	pub fn new(span: Span, module_path: &'a PathBuf) -> Self {
		Self { span, module_path }
	}
}

#[derive(Debug)]
pub struct Externs<'a> {
	pub externs: FxHashMap<String, Vec<ExternLocation<'a>>>,
}

impl<'a> Externs<'a> {
	pub fn new() -> Self {
		Self { externs: FxHashMap::default() }
	}

	pub fn push(&mut self, messages: &Messages<'a>, name: &str, span: Span) {
		let location = ExternLocation { span, module_path: &messages.file.path };
		self.externs.entry(name.to_string()).or_default().push(location);
	}
}
