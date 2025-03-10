#![allow(dead_code)]
mod config;
mod module_id;
pub use config::*;
use logos::Logos;
pub use module_id::*;
use rustc_hash::FxHashMap;
use std::{
	fs,
	path::{Path, PathBuf},
	sync::Arc,
};

use crate::{ast::Program, lexer::Token, parser::Parser, report::throw_error, source::Source};

pub struct Loader {
	sources: FxHashMap<ModuleId, Arc<Source>>,
	root_cache: FxHashMap<PathBuf, ModuleId>,
	root: FxHashMap<ModuleId, Program>,
	config: LoaderConfig,
	loading: FxHashMap<ModuleId, PathBuf>,
	pub entry_module: ModuleId,
}

type LoaderResult<T> = Result<T, String>;

impl Loader {
	pub fn new(config: LoaderConfig) -> Self {
		let sources = FxHashMap::default();
		let root_cache = FxHashMap::default();
		let root = FxHashMap::default();
		let loading = FxHashMap::default();
		let entry_module = ModuleId::new(u64::MAX);

		// todo: threads
		// if loader.config.max_threads > 0 {
		// 	rayon::ThreadPoolBuilder::new()
		// 		.num_threads(loader.config.max_threads)
		// 		.build_global()
		// 		.unwrap();
		// }
		Self { sources, root_cache, root, config, loading, entry_module }
	}

	pub fn load_file(&mut self, path: PathBuf) -> LoaderResult<ModuleId> {
		let module_id = self._load_file(path)?;
		Ok(module_id)
	}

	pub fn load_entry(&mut self) -> LoaderResult<ModuleId> {
		let path = self.config.entry_file.clone();
		let module_id = self._load_file(path)?;
		self.entry_module = module_id;
		Ok(module_id)
	}

	pub fn resolve_module(&mut self, path: String, from: ModuleId) -> LoaderResult<ModuleId> {
		// let path = PathBuf::from(path);
		let from_source = self.get_source(from)?;
		let base_dir = from_source.pathbuf().parent().unwrap_or(&self.config.cwd);
		let resolved_path = self.resolve_import_path(&path, base_dir)?;
		self.load_file(resolved_path)
	}

	fn finish(&mut self, module_id: ModuleId) {
		if self.root.contains_key(&module_id) {
			return;
		}
		let source = self.get_source_unwrap(module_id).clone();
		let mut lexer = Token::lexer(source.raw());
		let mut parser = Parser::new(&mut lexer, module_id, self);
		let ast = parser.parse_program().unwrap_or_else(|diag| diag.report_syntax_err_wrap(&source));
		self.root.insert(module_id, ast);
	}

	fn _load_file(&mut self, path: PathBuf) -> LoaderResult<ModuleId> {
		let path = self._canonicalize(path)?;
		if let Some(module_id) = self.root_cache.get(&path) {
			return Ok(*module_id);
		}
		let raw = self._read_file(path.clone())?;
		let name = self._strip_prefix(path.as_path())?.display().to_string();
		let module_id = self.add_module(path.clone(), name, raw)?;
		self.loading.insert(module_id, path);
		self.finish(module_id);
		self.loading.remove(&module_id);
		Ok(module_id)
	}

	fn add_module(&mut self, path: PathBuf, name: String, raw: String) -> LoaderResult<ModuleId> {
		let module_id = self.sources.len() as u64;
		self.check_circular(module_id.into())?;
		let source = Source::new(path.clone(), name, raw);
		self.sources.insert(module_id.into(), Arc::new(source));
		self.root_cache.insert(path, module_id.into());
		Ok(module_id.into())
	}

	fn check_circular(&mut self, module_id: ModuleId) -> LoaderResult<()> {
		if self.loading.contains_key(&module_id) {
			return Err("circular dependency detected".to_string());
		}
		Ok(())
	}

	pub fn get_source(&self, module_id: ModuleId) -> LoaderResult<&Source> {
		match self.sources.get(&module_id) {
			Some(source) => Ok(source),
			None => Err(format!("module '{}' not found", module_id)),
		}
	}

	pub fn get_ast(&mut self, module_id: ModuleId) -> &mut Program {
		match self.root.get_mut(&module_id) {
			Some(ast) => ast,
			None => throw_error(format!("module '{}' not found", module_id)),
		}
	}

	pub fn get_source_unwrap(&self, module_id: ModuleId) -> &Source {
		self.get_source(module_id).unwrap_or_else(|err| throw_error(err))
	}

	#[rustfmt::skip]
	pub fn _read_file(&mut self, path: PathBuf) -> LoaderResult<String> {
		fs::read_to_string(&path).map_err(|e| match e.kind() {
			std::io::ErrorKind::NotFound => format!("file '{}' not found", path.display()),
			std::io::ErrorKind::PermissionDenied => format!("permission denied reading '{}'", path.display()),
			std::io::ErrorKind::IsADirectory => format!("expected file, found directory '{}'", path.display()),
			_ => format!("could not read '{}', unexpected error", path.display()),
		})
	}

	#[rustfmt::skip]
	pub fn _canonicalize(&mut self, path: PathBuf) -> LoaderResult<PathBuf> {
		 path.canonicalize().map_err(|err|	match err.kind() {
			std::io::ErrorKind::NotFound => format!("canonicalizing '{}' failed, file not found", path.display()),
			std::io::ErrorKind::PermissionDenied => format!("canonicalizing '{}' failed, permission denied", path.display()),
			_ => format!("error canonicalizing '{}', unexpected error", path.display()),
		})
	}

	pub fn _strip_prefix(&mut self, path: &Path) -> LoaderResult<PathBuf> {
		match path.strip_prefix(&self.config.cwd) {
			Ok(stripped) => Ok(stripped.to_path_buf()),
			// Err(_err) => Err(format!("could not strip prefix of '{}'", path.display())),
			Err(_err) => Ok(path.to_path_buf()),
		}
	}

	fn resolve_import_path(&self, path_str: &str, cwd: &Path) -> LoaderResult<PathBuf> {
		match path_str {
			// "core" => self.resolve_special_module("lemon/core/mod.ln", "core module not found"),
			// "std" => self.resolve_special_module("lemon/std/mod.ln", "std module not installed"),
			path if path.starts_with("@/") => self.resolve_absolute_path(path_str),
			path if path.starts_with("./") => self.resolve_relative_path(path_str, cwd),
			_ => {
				Err(format!("invalid import '{}': must be 'core', 'std', '@/path', or './name'", path_str))
			}
		}
	}

	// fn resolve_special_module(&self, default_path: &str, error_msg: &str) -> LoaderResult<PathBuf> {
	// 	let path = PathBuf::from(default_path);
	// 	if path.exists() {
	// 		Ok(path)
	// 	} else {
	// 		Err(error_msg.to_string())
	// 	}
	// }

	fn resolve_absolute_path(&self, path_str: &str) -> LoaderResult<PathBuf> {
		let relative_path = path_str.strip_prefix("@/").unwrap();
		if relative_path.contains("..") || relative_path.is_empty() {
			return Err(format!("'@/' imports must be a valid path without '..', got '{}'", path_str));
		}
		let candidate = self.config.cwd.join(relative_path).join("mod.ln");
		if candidate.exists() {
			Ok(candidate)
		} else {
			Err(format!("absolute module '{}' not found or lacks mod.ln", path_str))
		}
	}

	fn resolve_relative_path(&self, path_str: &str, cwd: &Path) -> LoaderResult<PathBuf> {
		if path_str.contains("..") {
			return Err(format!("relative imports cannot use '..', got '{}'", path_str));
		}
		if path_str.matches('/').count() > 1 {
			return Err(format!("relative imports must be in the form './name', got '{}'", path_str));
		}
		let module_name = path_str.strip_prefix("./").unwrap();
		let mut candidate = cwd.join(module_name);
		if candidate.is_dir() {
			candidate.push("mod.ln");
		} else if candidate.extension().is_none() {
			candidate.set_extension("ln");
		}
		if candidate.exists() {
			Ok(candidate)
		} else {
			Err(format!("module '{}' not found or lacks mod.ln", path_str))
		}
	}
}
