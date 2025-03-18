#![allow(dead_code)]
mod mod_id;
use std::path::{Path, PathBuf};

pub use mod_id::*;
use rustc_hash::FxHashMap;

use crate::{ast, file_system::FileSystem, report::throw_error, shio::ShioConfig, source::Source};

pub struct Loader {
	shio: ShioConfig,
	file_system: FileSystem,
	root: FxHashMap<ModId, Source>,
	mods: FxHashMap<ModId, ast::Program>,
}

impl Loader {
	pub fn new(shio: ShioConfig, file_system: FileSystem) -> Self {
		Self { shio, file_system, root: FxHashMap::default(), mods: FxHashMap::default() }
	}

	pub fn load_entry(&mut self) -> Result<ModId, String> {
		let pathname = self.shio.loader.main.display().to_string();
		let (raw, abs_path) = self.file_system.load_mod_entry(&pathname)?;
		let source = Source::new(raw, abs_path, pathname);
		Ok(self.register_source(source))
	}

	pub fn load_source(&mut self, path: &str, base_mod_id: ModId) -> Result<ModId, String> {
		let mod_path = self.get_source_unchecked(base_mod_id).abs_path.clone();
		let (raw, abs_mod_path) = self.file_system.load_mod_from_base(mod_path, path)?;
		let path = self.resolve_path(path);
		let source = Source::new(raw, abs_mod_path, path.display().to_string());
		Ok(self.register_source(source))
	}
	fn register_source(&mut self, source: Source) -> ModId {
		let mod_id = ModId::new(self.root.len() as u64);
		self.root.insert(mod_id, source);
		mod_id
	}

	fn resolve_path(&self, path: &str) -> PathBuf {
		let given_path = Path::new(path);
		if given_path.extension().is_none() {
			return given_path.join("mod.ln");
		}
		given_path.to_path_buf()
	}

	pub fn get_source(&self, mod_id: ModId) -> Option<&Source> {
		self.root.get(&mod_id)
	}

	pub fn get_source_unchecked(&self, mod_id: ModId) -> &Source {
		self.get_source_result(mod_id).unwrap_or_else(|err| throw_error(err))
	}

	pub fn get_source_result(&self, mod_id: ModId) -> Result<&Source, String> {
		self.root.get(&mod_id).ok_or_else(|| format!("source for '{}' not found", mod_id))
	}

	pub fn add_mod(&mut self, mod_id: ModId, ast: ast::Program) {
		self.mods.insert(mod_id, ast);
	}

	pub fn get_mod(&self, mod_id: ModId) -> Option<&ast::Program> {
		self.mods.get(&mod_id)
	}

	pub fn get_mod_result(&mut self, mod_id: ModId) -> Result<&mut ast::Program, String> {
		self.mods.get_mut(&mod_id).ok_or_else(|| format!("ast for '{}' not found", mod_id))
	}
}
