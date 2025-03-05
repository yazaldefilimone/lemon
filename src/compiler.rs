use std::{io, path::Path, time::Instant};

use crate::{
	builder::Builder,
	checker::{context::Context, Checker},
	cross::Cross,
	diag::DiagGroup,
	disassembler::Disassembler,
	linker::Linker,
	llvm,
	loader::Loader,
	report::throw_cross_compile_error,
};

use clap::ArgMatches;
use console::{Style, Term};
use inkwell::targets::FileType;
use target_lexicon::HOST;

pub fn execute_term(respose: io::Result<()>, text: &str) {
	match respose {
		Ok(_) => {}
		Err(_) => println!("{}", text),
	}
}

pub fn write_in_term(term: &Term, text: impl Into<String>, is_clear: bool) {
	if is_clear {
		execute_term(term.clear_last_lines(1), "");
	}
	let text = text.into();
	execute_term(term.write_line(text.as_str()), text.as_str());
}

pub fn compile(path_name: &str, matches: &ArgMatches) {
	let timer = Instant::now();
	let term = Term::stdout();
	let style = Style::new();
	let compile_green_text = style.green().apply_to("compiling...").bold();
	write_in_term(&term, compile_green_text.to_string(), false);

	let mut loader = Loader::new();
	let file_id = loader.load(path_name);
	let source = loader.get_source(file_id).clone();
	let ast = loader.get_program(file_id);

	let mut diag_group = DiagGroup::new();
	let mut ctx = Context::new();

	// check
	write_in_term(&term, " check...", false);
	let mut checker = Checker::new(&mut diag_group, &mut ctx);
	let _ = match checker.check_program(ast) {
		Ok(tyy) => tyy,
		Err(diag) => diag.report_type_err_wrap(&source),
	};

	// emit lnr
	//
	write_in_term(&term, " emit lnr...", true);
	let mut ir_builder = Builder::new(&ctx.type_store, &source);
	let ir = ir_builder.build(ast);

	// optimize::optimize(&mut ir);

	if matches.get_flag("lnr") {
		let disassembler = Disassembler::new(&ctx.type_store);
		println!();
		let mut ir_text = String::new();
		disassembler.disassemble_program(&ir, &mut ir_text);
		println!("{}", ir_text);
		return;
	}

	// emit llvm
	//
	write_in_term(&term, " emit llvm...", true);
	let llvm_context = inkwell::context::Context::create();
	let llvm_module = llvm::create_module_from_source(&llvm_context, &source);
	let mut llvm = llvm::Llvm::new(&llvm_context, llvm_module, &ctx.type_store);
	llvm.compile_ir(&ir);
	if matches.get_flag("llr") {
		println!("{}", llvm.module.print_to_string().to_string());
		return;
	}

	// cross compile

	// println!("emit object...", HOST.architecture);
	write_in_term(&term, " emit object...", true);
	let triple = HOST.to_string();
	let cross = Cross::new(&triple);

	let output = generate_output_filename(&source.pathbuf);
	let output_path = Path::new(&output);
	match cross.emit(&llvm.module, FileType::Object, output_path) {
		Ok(_) => {}
		Err(err) => throw_cross_compile_error(err),
	}

	// link
	write_in_term(&term, " linking...", true);
	let linker = Linker::new(output_path.to_path_buf());
	linker.link();
	execute_term(term.clear_last_lines(1), "");

	let elapsed = timer.elapsed();
	let elapsed_text = if elapsed.as_secs_f64() < 1.0 {
		format!("{}ms", elapsed.as_millis())
	} else {
		format!("{:.2}s", elapsed.as_secs_f64())
	};

	let text = format!(" finished in {}.", elapsed_text);
	write_in_term(&term, text, false);
}

fn generate_output_filename(path: &Path) -> String {
	let file_name = path.file_name().unwrap().to_str().unwrap();
	let file_name_without_ext = file_name.split('.').next().unwrap();
	format!("{}.o", file_name_without_ext)
}

// pub fn run(path_name: &str) {
// 	let mut loader = Loader::new();
// 	let file_id = loader.load(path_name);
// 	let source = loader.get_source(file_id);
// 	let mut lexer = Token::lexer(source.raw());
// 	let mut parser = Parser::new(&mut lexer, file_id);
// 	let mut ast = match parser.parse_program() {
// 		Ok(ast) => ast,
// 		Err(diag) => diag.report_syntax_err_wrap(source),
// 	};

// 	let mut diag_group = DiagGroup::new(&loader);

// 	let mut ctx = Context::new();

// 	// check

// 	println!("check...");
// 	let mut checker = Checker::new(&mut diag_group, &mut ctx);
// 	let _ = match checker.check_program(&mut ast) {
// 		Ok(tyy) => tyy,
// 		Err(diag) => diag.report_type_err_wrap(source),
// 	};
// 	// emit lnr
// 	//
// 	println!("emit lnr...");
// 	let mut ir_builder = Builder::new(&ctx.type_store, source);
// 	let ir = ir_builder.build(&mut ast);

// 	println!("emit llvm...");
// 	let llvm_context = inkwell::context::Context::create();
// 	let llvm_module = llvm::create_module_from_source(&llvm_context, source);
// 	let mut llvm = llvm::Llvm::new(&llvm_context, llvm_module, &ctx.type_store);
// 	llvm.compile_ir(&ir);

// 	// cross compile

// 	println!("emit '{}' binary...", HOST.architecture);
// 	let triple = HOST.to_string();
// 	let cross = Cross::new(&triple);

// 	let output = generate_output_filename(&source.pathbuf);
// 	let output_path = Path::new(&output);

// 	match cross.emit(&llvm.module, FileType::Object, output_path) {
// 		Ok(_) => {}
// 		Err(err) => throw_cross_compile_error(err),
// 	}

// 	// link
// 	//
// 	println!("linking...");
// 	let linker = Linker::new(output_path.to_path_buf());
// 	let bin = linker.link();
// 	let command = format!("./{}", bin);
// 	println!("running...");
// 	std::process::Command::new("sh").arg("-c").arg(command).status().unwrap();
// }
