#![allow(dead_code)]
use core::fmt;

use crate::{
	loader::{Loader, ModuleId},
	range::Range,
	report::{self},
	source::Source,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
	Err,
	Warn,
	Note,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diag {
	pub module_id: ModuleId,
	pub severity: Severity,
	pub message: String,
	pub note: Option<String>,
	pub range: Range,
}

impl Diag {
	pub fn new(severity: Severity, message: String, range: Range) -> Self {
		let module_id = ModuleId::new(u64::MAX);
		Self { module_id, severity, message, note: None, range }
	}

	pub fn error(message: impl Into<String>, range: Range) -> Self {
		Self::new(Severity::Err, message.into(), range)
	}

	pub fn warning(message: impl Into<String>, range: Range) -> Self {
		Self::new(Severity::Warn, message.into(), range)
	}

	pub fn error_without_range(message: impl Into<String>) -> Self {
		Self::error(message, Range::default())
	}

	pub fn warning_without_range(message: impl Into<String>) -> Self {
		Self::warning(message, Range::default())
	}

	pub fn note(message: impl Into<String>, range: Range) -> Self {
		Self::new(Severity::Note, message.into(), range)
	}

	pub fn with_module_id(mut self, module_id: ModuleId) -> Self {
		self.module_id = module_id;
		self
	}

	pub fn with_note(mut self, note: impl Into<String>) -> Self {
		self.note = Some(note.into());
		self
	}
	pub fn get_range(&self) -> &Range {
		&self.range
	}

	pub fn report_syntax_err(&self, source: &Source) {
		report::report_syntax_err(self, source);
	}

	pub fn report_type_err(&self, source: &Source) {
		report::report_type_err(self, source);
	}

	pub fn report_err(&self, source: &Source) {
		report::report_err(self, source);
	}

	pub fn report_syntax_err_wrap(&self, source: &Source) -> ! {
		self.report_syntax_err(source);
		std::process::exit(1);
	}

	pub fn report_type_err_wrap(&self, source: &Source) -> ! {
		self.report_type_err(source);
		std::process::exit(1);
	}

	pub fn report_err_wrap(&self, source: &Source) -> ! {
		self.report_err(source);
		std::process::exit(1);
	}

	pub fn report_engine_err_wrap(&self) -> ! {
		report::report_engine_err(self);
		std::process::exit(1);
	}
}

pub struct DiagGroup {
	pub diags: Vec<Diag>,
}

impl DiagGroup {
	pub fn new() -> Self {
		Self { diags: Vec::new() }
	}
	pub fn add(&mut self, diag: Diag) {
		self.diags.push(diag);
	}

	pub fn report(&self, loader: &Loader) {
		for diag in &self.diags {
			let source = loader.get_source_unwrap(diag.module_id);
			diag.report_err(source);
		}
	}
}

impl Default for DiagGroup {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Display for Diag {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "severity: {}", self.severity)?;
		writeln!(f, "message: {}", self.message)?;
		writeln!(f, "range: {}", self.range)
	}
}

impl fmt::Display for Severity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Severity::Err => write!(f, "error"),
			Severity::Warn => write!(f, "warning"),
			Severity::Note => write!(f, "note"),
		}
	}
}
