use std::path::PathBuf;

use crate::report::throw_error;

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
	pub raw: String,
	pub pathbuf: PathBuf,
	pub pathname: String,
}

impl Source {
	pub fn new(pathbuf: PathBuf, pathname: String, raw: String) -> Self {
		Self { raw, pathbuf, pathname }
	}
	pub fn raw(&self) -> &str {
		&self.raw
	}

	pub fn pathbuf(&self) -> &PathBuf {
		&self.pathbuf
	}

	pub fn pathname(&self) -> &str {
		&self.pathname
	}

	pub fn name(&self) -> &str {
		match self.pathbuf.file_name() {
			Some(name) => name.to_str().unwrap_or_else(|| throw_error("could not get file name")),
			None => throw_error("could not get file name"),
		}
	}
}

impl Default for Source {
	fn default() -> Self {
		Self { raw: String::new(), pathbuf: PathBuf::new(), pathname: String::new() }
	}
}
