mod ast;
mod builder;
mod checker;
mod cli;
mod compiler;
// mod comptime;
mod cross;
mod diag;
mod disassembler;
mod ir;
mod lexer;
mod linker;
mod llvm;
mod loader;
// mod optimize;
mod parser;
mod range;
mod report;
mod source;

use checker::{context::Context, Checker};
use compiler::compile;
use diag::DiagGroup;
use lexer::Token;
use loader::{Loader, LoaderConfig};
use logos::Logos;
use parser::Parser;
use report::throw_error;
fn check(path_name: &str) {
	let config = LoaderConfig::new(path_name);
	let mut loader = Loader::new(config);

	let module_id = loader.load_entry().unwrap_or_else(|err| throw_error(err));
	let source = loader.get_source_unwrap(module_id).clone();
	let mut lexer = Token::lexer(source.raw());
	let mut parser = Parser::new(&mut lexer, module_id, &mut loader);

	let mut ast = match parser.parse_program() {
		Ok(ast) => ast,
		Err(diag) => diag.report_syntax_err_wrap(&source),
	};
	let mut diag_group = DiagGroup::new();
	let mut ctx = Context::new();
	let mut checker = Checker::new(&mut diag_group, &mut ctx, &mut loader);
	let _ = match checker.check_program() {
		Ok(tyy) => tyy,
		Err(diag) => diag.report_type_err_wrap(&source),
	};
	println!("ok.");
}

fn lex(path_name: &str) {
	let config = LoaderConfig::new(path_name);
	let mut loader = Loader::new(config);
	let module_id = loader.load_entry().unwrap_or_else(|err| throw_error(err));
	let source = loader.get_source_unwrap(module_id);
	let mut lexer = Token::lexer(source.raw());
	while let Some(token) = lexer.next() {
		println!("{:?}: {:?}", token, lexer.slice());
	}
}

fn token(path_name: &str) {
	let config = LoaderConfig::new(path_name);
	let mut loader = Loader::new(config);
	let module_id = loader.load_entry().unwrap_or_else(|err| throw_error(err));
	let source = loader.get_source_unwrap(module_id);
	let mut lexer = Token::lexer(source.raw());
	while let Some(token) = lexer.next() {
		println!("{:?}: {:?}", token, lexer.slice());
	}
}
fn ast(path_name: &str) {
	let config = LoaderConfig::new(path_name);
	let mut loader = Loader::new(config);
	let module_id = loader.load_entry().unwrap_or_else(|err| throw_error(err));
	let source = loader.get_source_unwrap(module_id).clone();
	let mut lexer = Token::lexer(source.raw());
	let mut parser = Parser::new(&mut lexer, module_id, &mut loader);
	let ast = match parser.parse_program() {
		Ok(ast) => ast,
		Err(diag) => diag.report_syntax_err_wrap(&source),
	};
	println!("{:#?}", ast);
}

fn main() {
	let matches = cli::command_line();
	match matches.subcommand() {
		Some(("check", matches)) => {
			let path_name = matches.get_one::<String>("file").expect("file is required");
			check(path_name);
		}

		Some(("compile", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			compile(path_name, matches);
		}
		Some(("lex", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			lex(path_name);
		}
		Some(("run", _matches)) => {
			// let path_name = matches.get_one::<String>("file").unwrap();
			todo!("run command is not implemented yet");
			// run(path_name);
		}

		Some(("token", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			token(path_name);
		}
		Some(("ast", matches)) => {
			let path_name = matches.get_one::<String>("file").unwrap();
			ast(path_name);
		}
		_ => {
			panic!("unknown command");
		}
	}
}
