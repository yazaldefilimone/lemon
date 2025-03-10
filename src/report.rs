use console::Style;

use crate::{
	diag::{Diag, Severity},
	range::Range,
	source::Source,
};

enum ReportKind {
	SyntaxErr,
	TypeErr,
	Err,
}

impl std::fmt::Display for ReportKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ReportKind::SyntaxErr => write!(f, "  syntax error"),
			ReportKind::TypeErr => write!(f, "  type error"),
			ReportKind::Err => write!(f, "  error"),
		}
	}
}

pub fn report_err(diag: &Diag, source: &Source) {
	report(diag, ReportKind::Err, source)
}

pub fn report_type_err(diag: &Diag, source: &Source) {
	report(diag, ReportKind::TypeErr, source)
}

pub fn report_engine_err(diag: &Diag) {
	let slug = text_red("comptime error");
	println!("{}: {}", slug, diag.message); // -- message

	if let Some(note) = &diag.note {
		println!("== {} {}", text_cyan("note:"), note);
	}
	std::process::exit(1);
}

pub fn report_syntax_err(diag: &Diag, source: &Source) {
	report(diag, ReportKind::SyntaxErr, source);
	std::process::exit(1);
}
pub fn throw_error_with_range(text: impl Into<String>, range: Range, source: &Source) -> ! {
	let start = range.start;
	let end = range.end;
	let err = codelighter::highlight_error(start, end, source.raw());
	let slug = text_red("error");
	println!("{}: {}", slug, text.into()); // -- message
	println!("---> {}", text_white(source.pathname())); // -- filename
	println!("{}", err);
	std::process::exit(1);
}
fn report(diag: &Diag, kind: ReportKind, source: &Source) {
	let slug = match diag.severity {
		Severity::Err => text_red(kind.to_string().as_str()),
		Severity::Warn => text_yellow("warning"),
		Severity::Note => text_green("info"),
	};
	println!("{}: {}", slug, diag.message); // -- message
	println!("---> {}", text_white(source.pathname())); // -- filename
	let start = diag.range.start;
	let end = diag.range.end;
	let code = match diag.severity {
		Severity::Err => codelighter::highlight_error(start, end, source.raw()),
		Severity::Warn => codelighter::highlight_warn(start, end, source.raw()),
		Severity::Note => codelighter::highlight_note(start, end, source.raw()),
	};
	println!("{}", code);
	if let Some(note) = &diag.note {
		println!("== {} {}", text_cyan("note:"), note);
	}
}

pub fn throw_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red("error:"), text_white(text.into().as_str()));
	std::process::exit(1);
}

// pub fn throw_engine_error(text: impl Into<String>) -> ! {
// 	println!("{} {}", text_red("  error:"), text_white(text.into().as_str()));
// 	std::process::exit(1);
// }

pub fn throw_llvm_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red("  error:"), text_white(text.into().as_str()));
	std::process::exit(1);
}

pub fn throw_linker_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red("  error:"), text_white(text.into().as_str()));
	std::process::exit(1);
}

pub fn throw_cross_compile_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red(" error:"), text_white(text.into().as_str()));
	std::process::exit(1);
}
pub fn throw_ir_build_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red("  error:"), text_white(text.into().as_str()));
	std::process::exit(1);
}
// let red = Style::new();
// 150 / 180 / 80
pub fn text_red(text: &str) -> String {
	let red = Style::new().color256(9);
	red.apply_to(text).bold().to_string()
}

pub fn text_yellow(text: &str) -> String {
	let yellow = Style::new().yellow();
	yellow.apply_to(text).bold().to_string()
}

pub fn text_green(text: &str) -> String {
	let green = Style::new().color256(10);
	green.apply_to(text).bold().to_string()
}

pub fn text_cyan(text: &str) -> String {
	let cyan = Style::new().color256(43);
	cyan.apply_to(text).bold().to_string()
}

pub fn text_white(text: &str) -> String {
	let white = Style::new().white();
	white.apply_to(text).bold().to_string()
}
