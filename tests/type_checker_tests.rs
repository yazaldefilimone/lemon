#[cfg(test)]
mod type_checker_tests {
	use lemon::{
		ast,
		checker::{context::Context, types::TypeId, Checker},
		lexer::Lexer,
		loader::Loader,
		parser::Parser,
		source::Source,
	};

	fn setup_checker(code: &str) -> (Checker, ast::Program) {
		let source = Source::from_string(code.to_string());
		let mut lexer = Lexer::new(source.clone());
		let tokens = lexer.tokenize().unwrap();
		let mut parser = Parser::new(tokens, source);
		let program = parser.parse().unwrap();
		
		let mut context = Context::new();
		let mut loader = Loader::new();
		let checker = Checker::new(&mut context, &mut loader);
		
		(checker, program)
	}

	#[test]
	fn test_basic_type_inference() {
		let code = r#"
			fn main() = {
				let x = 42;
				let y: i32 = x;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_type_mismatch_error() {
		let code = r#"
			fn main() = {
				let x: i32 = 42;
				let y: str = x;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		let mut has_error = false;
		for stmt in program.stmts.iter_mut() {
			if let Err(_) = checker.check_stmt(stmt) {
				has_error = true;
			}
		}
		assert!(has_error, "Expected type mismatch error");
	}

	#[test]
	fn test_borrow_checking() {
		let code = r#"
			fn main() = {
				let x = 42;
				let y = &x;
				let z = &mut x;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		let mut has_error = false;
		for stmt in program.stmts.iter_mut() {
			if let Err(_) = checker.check_stmt(stmt) {
				has_error = true;
			}
		}
		assert!(has_error, "Expected borrow checking error - cannot have mutable borrow while immutable borrow exists");
	}

	#[test]
	fn test_function_type_checking() {
		let code = r#"
			fn add(a: i32, b: i32): i32 = {
				return a + b;
			}
			
			fn main() = {
				let result = add(1, 2);
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_missing_return() {
		let code = r#"
			fn get_value(): i32 = {
				let x = 42;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		let mut has_error = false;
		for stmt in program.stmts.iter_mut() {
			if let Err(_) = checker.check_stmt(stmt) {
				has_error = true;
			}
		}
		assert!(has_error, "Expected missing return error");
	}

	#[test]
	fn test_struct_field_access() {
		let code = r#"
			struct Point {
				x: i32,
				y: i32,
			}
			
			fn main() = {
				let p = Point { x: 10, y: 20 };
				let x_val = p.x;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_numeric_type_coercion() {
		let code = r#"
			fn main() = {
				let x: i8 = 10;
				let y: i16 = x;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			// This should be OK if coercion is implemented
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_if_expression_type_unification() {
		let code = r#"
			fn main() = {
				let x = if true { 42 } else { 100 };
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_if_expression_type_mismatch() {
		let code = r#"
			fn main() = {
				let x = if true { 42 } else { "hello" };
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		let mut has_error = false;
		for stmt in program.stmts.iter_mut() {
			if let Err(_) = checker.check_stmt(stmt) {
				has_error = true;
			}
		}
		assert!(has_error, "Expected type mismatch in if branches");
	}

	#[test]
	fn test_loop_type_checking() {
		let code = r#"
			fn main() = {
				let mut i = 0;
				while i < 10 {
					i = i + 1;
				}
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_external_function() {
		let code = r#"
			extern fn printf(fmt: str, ...): i32 = {};
			
			fn main() = {
				printf("Hello %d\n", 42);
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_impl_block() {
		let code = r#"
			struct Vector {
				x: f32,
				y: f32,
			}
			
			impl Vector {
				fn length(self: &Vector): f32 = {
					return (self.x * self.x + self.y * self.y);
				}
			}
			
			fn main() = {
				let v = Vector { x: 3.0, y: 4.0 };
				let len = v.length();
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_type_alias() {
		let code = r#"
			type MyInt = i32;
			
			fn main() = {
				let x: MyInt = 42;
				let y: i32 = x;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_const_evaluation() {
		let code = r#"
			const MAX_SIZE: i32 = 100;
			
			fn main() = {
				let arr_size = MAX_SIZE;
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		for stmt in program.stmts.iter_mut() {
			let result = checker.check_stmt(stmt);
			assert!(result.is_ok());
		}
	}

	#[test]
	fn test_ownership_transfer() {
		let code = r#"
			struct Box {
				value: i32,
			}
			
			fn take_ownership(b: Box) = {
				// Takes ownership
			}
			
			fn main() = {
				let b = Box { value: 42 };
				take_ownership(b);
				// b is no longer valid here
				let x = b.value; // Should error
			}
		"#;
		
		let (mut checker, mut program) = setup_checker(code);
		let mut has_error = false;
		for stmt in program.stmts.iter_mut() {
			if let Err(_) = checker.check_stmt(stmt) {
				has_error = true;
			}
		}
		assert!(has_error, "Expected ownership error");
	}
}