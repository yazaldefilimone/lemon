use crate::message::MessageResult;
use crate::range::Range;

use super::diags::SyntaxErr;
use super::Checker;

use super::types::TypeId;

impl Checker<'_> {
	pub fn equal_type_id(&self, expected: TypeId, found: TypeId) -> bool {
		if expected == found {
			return true;
		}
		if expected.is_empty_type() && found.is_empty_type() {
			return true;
		}
		let expected_type = self.get_stored_type(expected);
		let found_type = self.get_stored_type(found);
		expected_type == found_type
	}

	pub fn equal_type_expected(
		&self,
		expected: TypeId,
		found: TypeId,
		range: Range,
	) -> MessageResult<TypeId> {
		if !self.equal_type_id(expected, found) {
			let expected = self.display_type(expected);
			let found = self.display_type(found);
			return Err(SyntaxErr::type_mismatch(expected, found, range));
		}
		Ok(expected)
	}
}
