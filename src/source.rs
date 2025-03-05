use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
	pub raw: String,
	pub pathbuf: PathBuf,
}

impl Default for Source {
	fn default() -> Self {
		Self { raw: String::new(), pathbuf: PathBuf::new() }
	}
}

impl Source {
	pub fn new(raw: String, pathbuf: PathBuf) -> Self {
		Self { raw, pathbuf }
	}
	pub fn raw(&self) -> &str {
		&self.raw
	}

	pub fn path(&self) -> &PathBuf {
		&self.pathbuf
	}

	pub fn path_str(&self) -> String {
		self.path().display().to_string()
	}

	#[allow(dead_code)]
	pub fn file_name(&self) -> &str {
		self.path().file_name().unwrap().to_str().unwrap()
	}
}
