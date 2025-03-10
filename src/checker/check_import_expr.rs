use crate::ast::{self};

use super::diags::SyntaxErr;
use super::types::TypeId;
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn check_import_expr(&mut self, import_expr: &mut ast::ImportExpr) -> TyResult<TypeId> {
		let filename = import_expr.get_path();
		let range = import_expr.get_range();
		let mod_id = match import_expr.module_id {
			Some(mod_id) => mod_id,
			None => return Err(SyntaxErr::not_found_module(filename.as_str(), range)),
		};
		println!("check import expr {}", filename);
		Err(SyntaxErr::not_found_module(import_expr.get_path().as_str(), import_expr.get_range()))
	}
}
