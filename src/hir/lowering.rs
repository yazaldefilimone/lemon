use crate::{ast, hir::hir};

pub fn lower_file(ast_file: ast::File) -> hir::HirFile {
	let mut functions = vec![];
	for statement in ast_file.block.statements {
		match statement {
			ast::Statement::Function(function) => {
				let hir_function = lower_function(function);
				functions.push(hir_function);
			}
			_ => {}
		}
	}

	hir::HirFile { functions }
}

pub fn lower_function(function: ast::Node<ast::Function>) -> hir::HirFunction {
	todo!()
}
