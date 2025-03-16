use crate::ast::{self};

use super::types::TypeId;
use super::{synthesis, Checker, TyResult};
impl Checker<'_> {
	pub fn check_literal(&mut self, lit: &ast::Literal) -> TyResult<TypeId> {
		synthesis::synthesise_literal(lit, self.ctx)
	}
}
