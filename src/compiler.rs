use std::{io, path::Path, time::Instant};

use crate::{
	builder::Builder,
	checker::{context::Context, Checker},
	cross::Cross,
	disassembler::Disassembler,
	file_system::FileSystem,
	linker::Linker,
	llvm,
	loader::Loader,
	parse_mod,
	shio::ShioConfig,
	throw_error,
	time::format_time,
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

	let path = Path::new(path_name);
	let shio = ShioConfig::with_defaults(path.to_path_buf());
	let cwd = shio.loader.cwd.clone();
	let file_system = FileSystem::from_current_dir(cwd);
	let mut loader = Loader::new(shio, file_system);
	let mod_id = loader.load_entry().unwrap_or_else(|message| message.report(&loader));
	let mut ctx = Context::new();
	let source = loader.lookup_source_unchecked(mod_id).clone();

	parse_mod(mod_id, &mut loader);
	// check
	write_in_term(&term, " check...", false);
	let mut checker = Checker::new(&mut ctx, &mut loader);
	checker.check(mod_id);

	// emit lnr
	//
	write_in_term(&term, " emit lnr...", true);
	let mut ir_builder = Builder::new(&ctx.type_store, &mut ctx.event, &mut loader);
	let ir = ir_builder.build(mod_id);

	// optimize::optimize(&mut ir);

	if true {
		let disassembler = Disassembler::new(&ctx.type_store);
		println!();
		let mut ir_text = String::new();
		disassembler.disassemble_program(&ir, &mut ir_text);
		println!("{}", ir_text);
	}

	// emit llvm
	//
	write_in_term(&term, " emit llvm...", true);
	let llvm_context = inkwell::context::Context::create();
	let llvm_module = llvm::create_module_from_source(&llvm_context, &source);
	let mut llvm = llvm::Llvm::new(&llvm_context, llvm_module, &loader, &ctx.type_store);
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

	let output = generate_output_filename(&source.abs_path);
	let output_path = Path::new(&output);
	match cross.emit(&llvm.module, FileType::Object, output_path) {
		Ok(_) => {}
		Err(err) => throw_error!("{}", err),
	}

	// link
	write_in_term(&term, " linking...", true);
	let linker = Linker::new(output_path.to_path_buf());
	linker.link();
	execute_term(term.clear_last_lines(1), "");

	let text = format!(" finished in {}.", format_time(timer.elapsed(), true));
	write_in_term(&term, text, false);
}

fn generate_output_filename(path: &Path) -> String {
	let file_name = path.file_name().unwrap().to_str().unwrap();
	let file_name_without_ext = file_name.split('.').next().unwrap();
	format!("{}.o", file_name_without_ext)
}
