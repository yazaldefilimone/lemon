use crate::{lexer::Lexer, messages::Messages};
mod ast;
mod color;
mod lexer;
mod loader;
mod messages;
mod parser;

#[macro_export]
macro_rules! usage_error {
	($($arg:tt)*) => {{
		eprint!("{}Usage error:{} ", crate::color::BOLD_RED, crate::color::RESET);
		eprintln!($( $arg )*);
		std::process::exit(-1);
	}}
}

fn main() {
	let source = r#"
	extern fn printf(fmt: str, ...): i32 = {}

fn println(value: i32) = {
		printf("%d\n", value)
}

fn test(x: i32): i32 = {
  if (x > 5) return x
  let n = x + 1
  return n
}

fn main() = {
  let x = test(1)
  println(x)
}
"#;

	let mut lex = Lexer::new(0, source);
	let paths = &["main.ln".to_string()];
	let mut messages = Messages::new(paths);
	let mut token_stream = lex.tokenize(&mut messages);

	println!("error: {:?}", messages.messages);
	while let Ok(token) = token_stream.advance(&mut messages) {
		println!("{}", token);
	}
	println!("error: {:?}", messages.messages);
}
