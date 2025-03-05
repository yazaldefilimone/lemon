use crate::ast::Program;
use crate::lexer::Token;
use crate::{parser::Parser, report::throw_error, source::Source};
use logos::Logos;
use rustc_hash::FxHashMap;
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(u64);

impl FileId {
	pub fn new(id: u64) -> Self {
		Self(id)
	}

	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

pub struct Loader {
	sources: FxHashMap<FileId, Source>,
	cache: FxHashMap<PathBuf, FileId>,
	parsed_asts: FxHashMap<FileId, Program>,
}

impl Loader {
	pub fn new() -> Self {
		Self {
			sources: FxHashMap::default(),
			cache: FxHashMap::default(),
			parsed_asts: FxHashMap::default(),
		}
	}

	fn validate_extension(path: &Path) {
		match path.extension().and_then(|ext| ext.to_str()) {
			Some("ln") | Some("lemon") => (),
			Some(ext) => throw_error(format!("unknown file extension '{}', expected 'ln'", ext)),
			None => throw_error("extension not found, expected 'ln'"),
		}
	}

	pub fn load(&mut self, path_str: &str) -> FileId {
		let path = PathBuf::from(path_str);
		Self::validate_extension(&path);

		let path_abs = path.canonicalize().unwrap_or(path.clone());

		if let Some(&file_id) = self.cache.get(&path_abs) {
			return file_id;
		}

		let file_id = self.add_file(&path_abs);
		self.load_file(file_id);
		file_id
	}

	fn load_file(&mut self, file_id: FileId) {
		let path = self.get_file(file_id);
		let raw = match fs::read_to_string(path) {
			Ok(content) => content,
			Err(err) => throw_error(format!("error reading file '{}': {}", path.display(), err)),
		};
		let source = Source::new(raw, path.to_owned());
		self.sources.insert(file_id, source);
		let ast = self.parse_program(file_id);
		self.parsed_asts.insert(file_id, ast);
	}

	fn parse_program(&mut self, file_id: FileId) -> Program {
		let source = self.sources.get(&file_id).unwrap();
		let mut lexer = Token::lexer(source.raw());
		let mut parser = Parser::new(&mut lexer, file_id);
		match parser.parse_program() {
			Ok(ast) => ast,
			Err(diag) => diag.report_syntax_err_wrap(source),
		}
	}
	fn get_file(&self, file_id: FileId) -> &PathBuf {
		match self.sources.get(&file_id).map(|source| &source.pathbuf) {
			Some(path) => path,
			None => throw_error(format!("file ID {} not found", file_id.as_usize())),
		}
	}

	pub fn get_source(&self, file_id: FileId) -> &Source {
		match self.sources.get(&file_id) {
			Some(source) => source,
			None => throw_error(format!("file ID {} not found", file_id.as_usize())),
		}
	}

	fn add_file(&mut self, path_abs: &Path) -> FileId {
		if let Some(&file_id) = self.cache.get(path_abs) {
			return file_id;
		}
		let file_id = FileId::new(self.sources.len() as u64);
		self.sources.insert(file_id, Source::new(String::new(), path_abs.to_owned()));
		self.cache.insert(path_abs.to_path_buf(), file_id);
		file_id
	}

	pub fn get_program(&mut self, file_id: FileId) -> &mut Program {
		match self.parsed_asts.get_mut(&file_id) {
			Some(ast) => ast,
			None => throw_error(format!("file ID {} not found", file_id.as_usize())),
		}
	}
}

impl Default for Loader {
	fn default() -> Self {
		Self::new()
	}
}
