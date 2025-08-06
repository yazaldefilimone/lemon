use crate::{messages::Messages, parser::parse_file, resolver::loader::load_single_file};

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

fn emit(stages: &[cli::EmitStage], path: &std::path::Path) {
	let mut files = Vec::new();
	let index = files.len() as u32;
	if let Err(e) = load_single_file(path.to_path_buf(), &mut files) {
		eprintln!("error loading file: {}", e);
		std::process::exit(1);
	}
	let file = files.get(index as usize).unwrap();

	match stages {
		[cli::EmitStage::Ast] => {
			let mut messages = Messages::new(file);
			let mut lexer = lexer::Lexer::new(index, &file.source);
			let mut token_reader = lexer.reader(&mut messages);
			let file = parse_file(&file, &mut token_reader, &mut messages);
			messages.print("parser");
			println!("{:#?}", file);
		}
		_ => {
			todo!()
		}
	}
}

fn print(item: cli::PrintItem, path: &std::path::Path) {
	let mut files = Vec::new();
	let index = files.len() as u32;
	if let Err(e) = load_single_file(path.to_path_buf(), &mut files) {
		eprintln!("error loading file: {}", e);
		std::process::exit(1);
	}
	let file = files.get(index as usize).unwrap();
	match item {
		cli::PrintItem::Tokens => {
			let mut messages = Messages::new(file);
			let mut lexer = lexer::Lexer::new(index, &file.source);
			let mut token_reader = lexer.reader(&mut messages);
			while !token_reader.end() {
				if let Ok(token) = token_reader.next(&mut messages) {
					println!("{}", token);
				}
			}
			println!("errors: {}", messages.list.len());
			messages.print("lexer");
		}
		cli::PrintItem::Span => println!("span"),
		cli::PrintItem::Types => println!("types"),
	}
}

fn build() {
	println!("build");
}

fn main() {
	let arguments = cli::parse_args();
	let file = arguments.input.to_path_buf();
	// let output = arguments.output.as_ref().map(|p| p.to_path_buf());

	// let mut resolver = resolver::Resolver::new(file);
	// resolver.resolve();

	// let ast = resolver.ast();

	match arguments.mode() {
		cli::Mode::Emit(stages) => emit(&stages, &file),
		cli::Mode::Print(item) => print(item, &file),
		cli::Mode::Build => build(),
		cli::Mode::Check => check(),
	}
}
