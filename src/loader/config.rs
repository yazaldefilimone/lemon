#![allow(dead_code)]
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LoaderConfig {
	pub search_paths: Vec<PathBuf>,
	pub extensions: Vec<String>,
	pub cwd: PathBuf,
	pub entry_file: PathBuf,
	pub max_threads: usize,
}
const DEFAULT_MAX_THREADS: usize = 4;

fn get_entry_file_and_cwd(entry_file: &str) -> (PathBuf, PathBuf) {
	let entry_file = PathBuf::from(entry_file);
	let cwd = match entry_file.parent() {
		Some(dir) => dir.to_path_buf(),
		None => PathBuf::from("."),
	};
	(entry_file, cwd)
}

impl LoaderConfig {
	pub fn new(entry_file: &str) -> Self {
		let (entry_file, cwd) = get_entry_file_and_cwd(entry_file);
		let max_threads = DEFAULT_MAX_THREADS;
		let extensions = vec!["rs".to_string()];
		let search_paths = vec![cwd.clone()];
		Self { search_paths, extensions, cwd, entry_file, max_threads }
	}

	pub fn with_search_paths(mut self, search_paths: Vec<PathBuf>) -> Self {
		self.search_paths = search_paths;
		self
	}

	pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
		self.extensions = extensions;
		self
	}

	pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
		self.cwd = cwd;
		self
	}

	pub fn with_entry_file(mut self, entry_file: &str) -> Self {
		let (entry_file, cwd) = get_entry_file_and_cwd(entry_file);
		self.entry_file = entry_file;
		self.cwd = cwd;
		self
	}

	pub fn with_max_threads(mut self, max_threads: usize) -> Self {
		self.max_threads = max_threads;
		self
	}
}
