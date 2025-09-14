use std::cmp::Ordering;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::{
	checker::{context::store::TypeStore, types::TypeId},
	error,
	messages::Messages,
	span::Span,
};

#[derive(Debug, Copy, Clone)]
pub struct NumberValue {
	value: Decimal,
	pub span: Span,
	collapse: Option<TypeId>,
}

#[track_caller]
fn assert_not_collapsed(collapse: Option<TypeId>) {
	if collapse.is_some() {
		panic!("assertion collapse is none failed: {collapse:?}");
	}
}

impl NumberValue {
	pub fn new(value: Decimal, span: Span) -> NumberValue {
		NumberValue { value, span, collapse: None }
	}

	pub fn new_collapsed(value: Decimal, span: Span, collapse: TypeId) -> NumberValue {
		NumberValue { value, span, collapse: Some(collapse) }
	}

	pub fn value(&self) -> Decimal {
		self.value
	}

	pub fn is_integer(&self) -> bool {
		self.value.is_integer()
	}

	// return value indicates success if true
	pub fn collapse(&mut self, type_store: &TypeStore, type_id: TypeId) -> bool {
		let mut success = true;
		if let Some(collapsed) = self.collapse {
			success = type_store.direct_match(collapsed, type_id);
		}
		self.collapse = Some(type_id);
		success
	}

	pub fn collapsed(&self) -> Option<TypeId> {
		self.collapse
	}

	pub fn negate(&mut self, sign_span: Span) {
		assert_not_collapsed(self.collapse);
		self.value.set_sign_positive(self.value.is_sign_negative());
		self.span += sign_span;
	}

	pub fn add(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		assert_not_collapsed(self.collapse);
		assert_not_collapsed(other.collapse);
		let span = self.span + other.span;

		let Some(value) = self.value.checked_add(other.value) else {
			let err = error!("overflow or underflow in constant addition");
			messages.message(err.with_span(span));
			return None;
		};

		Some(NumberValue { value, span, collapse: None })
	}

	pub fn sub(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		assert_not_collapsed(self.collapse);
		assert_not_collapsed(other.collapse);

		let span = self.span + other.span;

		let Some(value) = self.value.checked_sub(other.value) else {
			let err = error!("overflow or underflow in constant subtraction");
			messages.message(err.with_span(span));
			return None;
		};

		Some(NumberValue { value, span, collapse: None })
	}

	pub fn mul(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		assert_not_collapsed(self.collapse);
		assert_not_collapsed(other.collapse);
		let span = self.span + other.span;

		let Some(value) = self.value.checked_mul(other.value) else {
			let err = error!("overflow or underflow in constant multiplication");
			messages.message(err.with_span(span));
			return None;
		};

		Some(NumberValue { value, span, collapse: None })
	}

	pub fn div(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		assert_not_collapsed(self.collapse);
		assert_not_collapsed(other.collapse);
		let span = self.span + other.span;

		let Some(value) = self.value.checked_div(other.value) else {
			let err = error!("Overflow or underflow in constant division");
			messages.message(err.with_span(span));
			return None;
		};

		Some(NumberValue { value, span, collapse: None })
	}

	// TODO: These error messages should be thought out a bit better
	pub fn modulo(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		assert_not_collapsed(self.collapse);
		assert_not_collapsed(other.collapse);
		let span = self.span + other.span;

		let truncated_remainder = self.value % other.value;
		let value = if truncated_remainder.is_sign_negative() {
			let abs = other.value.abs();
			let Some(added) = truncated_remainder.checked_add(abs) else {
				let err = error!("addition failure in constant modulo");
				messages.message(err.with_span(span));
				return None;
			};

			added
		} else {
			truncated_remainder
		};

		Some(NumberValue { value, span, collapse: None })
	}

	fn check_not_integer_error(
		&self,
		messages: &mut Messages,
		other: NumberValue,
		op: &str,
		side: &str,
	) -> bool {
		if !self.value.is_integer() {
			let error = error!("cannot perform constant {op} with {side} side value {}", self.value);
			messages.message(error.with_span(self.span + other.span));
			return true;
		}

		false
	}

	pub fn bitwise_and(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		if self.check_not_integer_error(messages, other, "bitwise and", "left") {
			return None;
		}

		if other.check_not_integer_error(messages, self, "bitwise and", "right") {
			return None;
		}

		let result = self.value.to_i128().unwrap() & other.value.to_i128().unwrap();
		let value = Decimal::from(result);
		let span = self.span + other.span;
		Some(NumberValue { value, span, collapse: None })
	}

	pub fn bitwise_or(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		if self.check_not_integer_error(messages, other, "bitwise or", "left") {
			return None;
		}

		if other.check_not_integer_error(messages, self, "bitwise or", "right") {
			return None;
		}

		let result = self.value.to_i128().unwrap() | other.value.to_i128().unwrap();
		let value = Decimal::from(result);
		let span = self.span + other.span;
		Some(NumberValue { value, span, collapse: None })
	}

	pub fn bitwise_xor(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		if self.check_not_integer_error(messages, other, "bitwise xor", "left") {
			return None;
		}

		if other.check_not_integer_error(messages, self, "bitwise xor", "right") {
			return None;
		}

		let result = self.value.to_i128().unwrap() ^ other.value.to_i128().unwrap();
		let value = Decimal::from(result);
		let span = self.span + other.span;
		Some(NumberValue { value, span, collapse: None })
	}

	pub fn bitshift_left(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		if !self.value.is_integer() {
			let error =
				error!("cannot perform constant bitshift left with left side value {}", self.value);
			messages.message(error.with_span(self.span + other.span));
			return None;
		}

		if other.value.is_sign_negative() || !other.value.is_integer() {
			let error =
				error!("cannot perform constant bitshift left with right side value {}", other.value);
			messages.message(error.with_span(self.span + other.span));
			return None;
		}

		if other.value.is_zero() {
			return Some(self);
		}

		// TODO: This is rather silly but rust_decimal does not seem to have "proper" shift methods

		let span = self.span + other.span;
		let mut count = other.value;
		let mut value = self.value;
		while count.cmp(&Decimal::ZERO) == Ordering::Greater {
			count = count.saturating_sub(Decimal::ONE);

			let Some(new_value) = value.checked_mul(Decimal::TWO) else {
				let error = error!("overflow or underflow in constant bitshift left");
				messages.message(error.with_span(span));
				return None;
			};

			value = new_value;
		}

		Some(NumberValue { value, span, collapse: None })
	}

	pub fn bitshift_right(self, messages: &mut Messages, other: NumberValue) -> Option<NumberValue> {
		if !self.value.is_integer() {
			let error =
				error!("cannot perform constant bitshift right with left side value {}", self.value);
			messages.message(error.with_span(self.span + other.span));
			return None;
		}

		if other.value.is_sign_negative() || !other.value.is_integer() {
			let error =
				error!("cannot perform constant bitshift right with right side value {}", other.value);
			messages.message(error.with_span(self.span + other.span));
			return None;
		}

		if other.value.is_zero() {
			return Some(self);
		}

		// TODO: This is rather silly but rust_decimal does not seem to have "proper" shift methods

		let span = self.span + other.span;
		let mut count = other.value;
		let mut value = self.value;
		while count.cmp(&Decimal::ZERO) == Ordering::Greater {
			count = count.saturating_sub(Decimal::ONE);

			let Some(new_value) = value.checked_div(Decimal::TWO) else {
				let error = error!("underflow or underflow in constant bitshift right");
				messages.message(error.with_span(span));
				return None;
			};

			value = new_value.trunc();
		}

		Some(NumberValue { value, span, collapse: None })
	}
}
