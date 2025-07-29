mod ast;
mod checker;
mod cli;
mod color;
mod comptime;
mod hir;
mod lexer;
mod llvm;
mod macros;
mod messages;
mod mir;
mod parser;
mod resolver;
mod span;
mod token;
mod token_reader;

fn check() {
	println!("check");
}

fn emit(stages: &[cli::EmitStage]) {
	println!("emit: {:?}", stages);
}

fn print(item: cli::PrintItem) {
	println!("print: {:?}", item);
}

fn build() {
	println!("build");
}

fn main() {
	let arguments = cli::parse_args();
	let file = arguments.input.to_path_buf();
	let output = arguments.output.as_ref().map(|p| p.to_path_buf());

	// let mut resolver = resolver::Resolver::new(file);
	// resolver.resolve();

	// let ast = resolver.ast();

	match arguments.mode() {
		cli::Mode::Emit(stages) => emit(&stages),
		cli::Mode::Print(item) => print(item),
		cli::Mode::Build => build(),
		cli::Mode::Check => check(),
	}
}
