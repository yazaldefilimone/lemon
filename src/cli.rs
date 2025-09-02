use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lemonc", version, about = "the lemon language compiler")]
pub struct Cli {
	/// Path to the input source file.
	#[arg(value_name = "INPUT_FILE")]
	pub input_file: PathBuf,

	/// Optional output file path.
	#[arg(short = 'o', long, value_name = "OUTPUT_FILE")]
	pub output_file: Option<PathBuf>,

	/// Enable release mode optimizations.
	#[arg(short = 'r', long)]
	pub release_mode: bool,

	/// only check the code, do not build.
	#[arg(long)]
	pub check_only: bool,

	/// emit intermediate representations (AST, HIR, MIR, LLVMIR).
	#[arg(long, value_enum, value_delimiter = ',', value_name = "STAGES")]
	pub emit_stages: Vec<EmitStage>,

	/// print internal compiler data (tokens, spans, types).
	#[arg(long, value_enum, value_name = "ITEM")]
	pub print_item: Option<PrintItem>,
}

/// intermediate representation stages that can be emitted.
#[derive(Debug, Clone, ValueEnum)]
pub enum EmitStage {
	AST,
	HIR,
	MIR,
	LLVMIR,
}

/// Internal compiler data that can be printed.
#[derive(Debug, Clone, ValueEnum)]
pub enum PrintItem {
	Tokens,
	Spans,
	Types,
}

#[derive(Debug)]
pub enum Mode {
	CheckOnly,
	EmitStages(Vec<EmitStage>),
	Inspect(PrintItem),
	Build,
}

#[derive(Debug)]
pub enum Backend {
	LLVM,
	Cranelift,
}

impl Default for Backend {
	fn default() -> Self {
		Backend::LLVM
	}
}

#[derive(Debug)]
pub enum OptimizerLevel {
	None,
	Speed,
	Size,
	Max,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Linker {
	Lld,  // best for macOS(cross-platform)
	Mold, // best for linux/windows
	Msvc, // best for windows
}

impl Default for Linker {
	fn default() -> Self {
		Linker::Lld
	}
}

impl Default for OptimizerLevel {
	fn default() -> Self {
		OptimizerLevel::None
	}
}

#[derive(Debug)]
pub struct CompilerOptions {
	pub release_mode: bool,
	/// Path to the input source file.
	pub input_file: PathBuf,

	/// Path to the output file (binary, object, etc.).
	pub output_file: Option<PathBuf>,

	/// Optimization level (0–3, s, z).
	pub optimizer_level: OptimizerLevel,

	/// high-level compiler mode (check, emit, inspect, build).
	pub mode: Mode,

	/// selected code generation backend.
	pub backend: Backend,

	/// whether to link against the standard library.
	pub std_enabled: bool,

	/// whether to use colored diagnostics.
	pub color_messages: bool,

	pub verbose: bool,

	pub target: Option<String>, // ex: "x86_64-unknown-linux-gnu"

	pub linker: Linker,

	// debug options
	pub check_llvm_ir: bool,
	// pub debug_ast: bool,
	// pub debug_hir: bool,
	// pub debug_mir: bool,
	// pub debug_resolver: bool,
	pub debug_type: bool,
}

impl Default for CompilerOptions {
	fn default() -> Self {
		CompilerOptions {
			release_mode: false,
			input_file: PathBuf::new(),
			output_file: None,
			optimizer_level: OptimizerLevel::default(),
			mode: Mode::Build,
			backend: Backend::default(),
			std_enabled: true,
			color_messages: true,
			verbose: false,
			target: None,
			check_llvm_ir: false,
			linker: Linker::default(),
			// debug_ast: false,
			// debug_hir: false,
			// debug_mir: false,
			// debug_resolver: false,
			debug_type: false,
		}
	}
}

impl Cli {
	pub fn mode(&self) -> Mode {
		if self.check_only {
			Mode::CheckOnly
		} else if !self.emit_stages.is_empty() {
			Mode::EmitStages(self.emit_stages.clone())
		} else if let Some(print_item) = &self.print_item {
			Mode::Inspect(print_item.clone())
		} else {
			Mode::Build
		}
	}
}

/// Parse command-line arguments into a `Cli` struct.
pub fn parse_args() -> Cli {
	Cli::parse()
}
