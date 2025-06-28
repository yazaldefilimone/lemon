use std::io::Read;
use std::path::PathBuf;

use crate::usage_error;

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub struct SourceFile {
	pub source: String,
	pub path: PathBuf,
	pub index: u32,
}

impl std::fmt::Debug for SourceFile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SourceFile")
			.field("source", &"...")
			.field("path", &self.path)
			.field("index", &self.index)
			.finish()
	}
}

pub fn load_single_file(path: PathBuf, files: &mut Vec<SourceFile>) -> Result<String> {
	let mut file = std::fs::File::open(&path)?;
	let capacity = file.metadata().map(|m| m.len()).unwrap_or(0);
	let mut source = String::with_capacity(capacity as usize);
	file.read_to_string(&mut source)?;

	let Some(stem) = path.file_stem() else {
		usage_error!("Input file has no name");
	};

	let extension_len = ".fae".len();
	let full_name = stem.to_string_lossy();
	let file_name = full_name[..full_name.len() - extension_len].to_owned();

	let index = files.len() as u32;
	files.push(SourceFile { source, path, index });

	Ok(file_name)
}
