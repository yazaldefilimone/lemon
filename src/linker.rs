use std::{path::PathBuf, process::Command};

use crate::report::throw_linker_error;

pub struct Linker {
	pub input: PathBuf,
	pub output: PathBuf,
	bin_path: PathBuf,
}

impl Linker {
	pub fn new(input: PathBuf) -> Self {
		let bin_path = match which::which("clang").map(|p| p.to_string_lossy().to_string()) {
			Ok(path) => path,
			Err(_) => throw_linker_error("not found 'clang' binary"),
		};
		let output = input.with_extension("");
		Self { input, output, bin_path: PathBuf::from(bin_path) }
	}

	pub fn link(&self) -> String {
		let mut command = Command::new(&self.bin_path);
		let input = self.input.to_str().unwrap();
		let output_path = self.output.to_str().unwrap();
		command.arg(input).arg("-o").arg(output_path);
		let output = match command.output() {
			Ok(output) => output,
			Err(err) => throw_linker_error(format!("failed to link: {}", err)),
		};
		if !output.status.success() {
			let string = String::from_utf8_lossy(&output.stderr);
			//
			if string.contains("symbol(s) not found") {
				// assume that is not found main function
				throw_linker_error("not found main function");
			}
			throw_linker_error(format!("{}", string));
		}
		// remove object file
		std::fs::remove_file(input).unwrap_or_else(|err| throw_linker_error(format!("{}", err)));
		output_path.to_string()
	}
}
