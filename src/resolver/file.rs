use std::{io, path::PathBuf};

pub type Result<T> = std::result::Result<T, io::Error>;

pub struct SourceFile {
	pub source: String,
	pub path: PathBuf,
	pub index: u32,
}

impl SourceFile {
	pub fn new(source: String, path: PathBuf, index: u32) -> Self {
		Self { source, path, index }
	}
}

impl std::fmt::Debug for SourceFile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SourceFile")
			.field("source", &"...")
			.field("path", &self.path)
			.field("index", &self.index)
			.finish()
	}
}
