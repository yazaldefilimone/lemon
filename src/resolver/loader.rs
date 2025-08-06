use crate::resolver::file::{Result, SourceFile};
use crate::usage_error;

use std::{fs::File, io::Read, path::PathBuf};

pub fn load_single_file(path: PathBuf, files: &mut Vec<SourceFile>) -> Result<String> {
	let mut file = File::open(&path)?;

	let capacity = file.metadata()?.len() as usize;
	let mut source = String::with_capacity(capacity);
	file.read_to_string(&mut source)?;

	let Some(stem) = path.file_stem() else {
		usage_error!("input file has no name");
	};

	let name = stem.to_string_lossy();
	let name_without_ext = name.strip_suffix(".ln").unwrap_or(&name).to_owned();

	let index = files.len() as u32;
	files.push(SourceFile::new(source, path, index));

	Ok(name_without_ext)
}
