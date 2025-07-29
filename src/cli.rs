use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lemonc", version, about = "lemon language compiler")]
pub struct Cli {
	#[arg(value_name = "input", help = "path to the input source file")]
	pub input: PathBuf,

	#[arg(short = 'o', long, value_name = "output", help = "optional output file path")]
	pub output: Option<PathBuf>,

	#[arg(short = 'r', long, help = "enable release mode optimizations")]
	pub release: bool,

	#[arg(long, help = "only check the code, do not build")]
	pub check: bool,

	#[arg(
		long,
		value_enum,
		value_delimiter = ',',
		value_name = "stage",
		help = "emit intermediate representations (ast, hir, mir, llvmir)"
	)]
	pub emit: Vec<EmitStage>,

	#[arg(
		long,
		value_enum,
		value_name = "item",
		help = "print internal compiler data (tokens, span, types)"
	)]
	pub print: Option<PrintItem>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmitStage {
	Ast,
	Hir,
	Mir,
	LlvmIr,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PrintItem {
	Tokens,
	Span,
	Types,
}
pub enum Mode {
	Check,
	Emit(Vec<EmitStage>),
	Print(PrintItem),
	Build,
}

impl Cli {
	pub fn mode(&self) -> Mode {
		if self.check {
			Mode::Check
		} else if !self.emit.is_empty() {
			Mode::Emit(self.emit.clone())
		} else if let Some(print_item) = self.print.clone() {
			Mode::Print(print_item)
		} else {
			Mode::Build
		}
	}
}

pub fn parse_args() -> Cli {
	Cli::parse()
}
