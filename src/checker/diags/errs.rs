use crate::{ast::Operator, error_type, message::Message, range::Range};

// todo: improve this...
#[derive(Debug)]
pub enum SyntaxErr<'tce> {
	// type errors
	TypeMismatch { expected: String, found: String, range: Range },
	NotFn { found: String, range: Range },
	UnsupportedOperator { left: String, right: String, operator: &'tce Operator },
	ValueExpected { value: String, range: Range },
	UnexpectedValue { value: String, range: Range },
	BoundsError { value: String, found: String, range: Range },
	ArgsMismatch { expected: usize, found: usize, range: Range },

	// fn errors
	RedefineFnInSameScope { name: String, range: Range },
	ReturnLocalBorrow { range: Range },

	// module errors
	MainInNonMainModule { range: Range, pathname: String },
	MissingMainInEntryModule { range: Range, pathname: String },
	CannotReassignModule { range: Range },
	TypeAnnotationNotAllowedForModule { range: Range },
	NotFoundPubItem { name: String, range: Range },

	// context errors
	ReturnOutsideFn { range: Range },
	ReturnNotInFnScope { range: Range },
	RequiredTypeNotation { range: Range },
	ConnotReturnLocalRerefence { range: Range },

	// borrow errors
	Immutable { name: &'tce str, range: Range },
	BorrowConflict { range: Range },
	DoubleMutBorrow { range: Range },
	InvalidBorrow { range: Range },
	BorrowedValueDropped { range: Range },
	CannotDereference { type_name: String, range: Range },
	CannotBorrowAsMutable { name: String, range: Range },
	CannotBorrowAsMutableMoreThanOnce { name: String, range: Range },
	BorrowExpected { range: Range },
	CannotAssignImmutable { name: String, range: Range },

	// other errors
	NotFoundValue { name: &'tce str, range: Range },
	NotFoundModule { name: &'tce str, range: Range },
	InvalidFloat { range: Range },
	NumberTooLarge { range: Range },
	ExpectedNumber { range: Range },
	LeftHandCannotBeAssigned { range: Range },
	// const errors
	ConstOutsideGlobalScope { range: Range },
	ConstRedefinition { range: Range },
	// type alias / struct / enum...  errors
	NotFoundType { name: &'tce str, range: Range },

	// instaced impl
	ExpectInstacedType { found: String, range: Range },
	NotFoundField { name: &'tce str, range: Range },
	NotImpl { found: String, range: Range },

	// member and associated fn errors
	NotFoundMethodNamed { name: String, found: String, range: Range },
	NotFoundAssociateField { name: String, found: String, range: Range },
	// module errors
	// InvalidModulePath { path: String, range: Range },
	// ModuleRedefined { name: String, range: Range },
	// ModuleImportFailed { name: String, range: Range },
	WrongArgCount { expected: usize, found: usize, range: Range },
}

impl<'tce> SyntaxErr<'tce> {
	#[inline]
	pub fn const_outside_global_scope(range: Range) -> Message {
		error_type!("const can only be defined at global scope").range(range)
	}

	#[inline]
	pub fn const_redefinition(range: Range) -> Message {
		error_type!("const already defined").range(range)
	}

	#[inline]
	pub fn immutable(name: &'tce str, range: Range) -> Message {
		error_type!("value '{}' is not mutable", name).range(range)
	}

	#[inline]
	pub fn type_mismatch(expected: String, found: String, range: Range) -> Message {
		error_type!("expected '{}', found '{}'", expected, found).range(range)
	}

	#[inline]
	pub fn args_mismatch(expected: usize, found: usize, range: Range) -> Message {
		if expected > 1 {
			error_type!("expected {} args, found {}", expected, found).range(range)
		} else {
			error_type!("expected {} arg, found {}", expected, found).range(range)
		}
	}

	#[inline]
	pub fn wrong_arg_count(expected: usize, found: usize, range: Range) -> Message {
		if expected == 0 {
			error_type!("expected no args, found {}", found).range(range)
		} else if expected == 1 {
			error_type!("expected one arg, found {}", found).range(range)
		} else {
			error_type!("expected {} args, found {}", expected, found).range(range)
		}
	}
	#[inline]
	pub fn unexpected_arity(expected: usize, found: usize, range: Option<Range>) -> Message {
		let message = match expected {
			0 => format!("expected no arguments, but got {found}"),
			1 => format!("expected a single argument, but found {found}"),
			n => format!("expected {n} arguments, but found {found}"),
		};
		error_type!("{message}").range_if_some(range)
	}

	#[inline]
	pub fn not_found_value(name: &'tce str, range: Range) -> Message {
		error_type!("value '{}' not found", name).range(range)
	}

	#[inline]
	pub fn bounds_error(value: String, found: String, range: Range) -> Message {
		error_type!("'{}' out of bounds, expected '{}'", value, found).range(range)
	}

	#[inline]
	pub fn required_type_notation(range: Range) -> Message {
		error_type!("required type notation, cannot infer type").range(range)
	}

	#[inline]
	pub fn cannot_infer_type(range: Range) -> Message {
		error_type!("cannot infer type").range(range)
	}
	#[inline]
	pub fn value_expected(value: String, range: Range) -> Message {
		error_type!("expected '{}', found unit", value).range(range)
	}

	#[inline]
	pub fn unexpected_value(value: String, range: Range) -> Message {
		error_type!("no expected value, found '{}'", value).range(range)
	}

	#[inline]
	pub fn return_outside_fn(range: Range) -> Message {
		error_type!("cannot return outside of a fn").range(range)
	}

	#[inline]
	pub fn invalid_float(range: Range) -> Message {
		// Self::InvalidFloat { range }.into()
		error_type!("floating-point number out of range").range(range)
	}

	#[inline]
	pub fn number_too_large(range: Range) -> Message {
		// Self::NumberTooLarge { range }.into()
		error_type!("unsupport number, out of range").range(range)
	}

	#[inline]
	pub fn expected_number(range: Range) -> Message {
		error_type!("expected number").range(range)
	}

	#[inline]
	pub fn return_not_in_fn_scope(range: Range) -> Message {
		error_type!("cannot return outside of a fn").range(range)
	}
	#[inline]
	pub fn not_found_module(name: &'tce str, range: Range) -> Message {
		error_type!("module '{}' not found", name).range(range)
	}

	#[inline]
	pub fn cannot_dereference(type_name: String, range: Range) -> Message {
		error_type!("cannot dereference '{}'", type_name).range(range)
	}
	#[inline]
	pub fn not_a_fn(found: String, range: Range) -> Message {
		error_type!("expected a fn, found '{}'", found).range(range)
	}

	#[inline]
	pub fn borrow_conflict(range: Range) -> Message {
		error_type!("mutable and immutable borrows conflict").range(range)
	}

	#[inline]
	pub fn double_mut_borrow(range: Range) -> Message {
		error_type!("already mutably borrowed").range(range)
	}

	#[inline]
	pub fn invalid_borrow(range: Range) -> Message {
		error_type!("invalid borrow, value already freed").range(range)
	}

	#[inline]
	pub fn borrowed_value_dropped(range: Range) -> Message {
		error_type!("borrowed value was dropped before borrow ended").range(range)
	}

	#[inline]
	pub fn cannot_borrow_as_mutable(name: &str, range: Range) -> Message {
		error_type!("cannot borrow as mutable '{}'", name).range(range)
	}

	#[inline]
	pub fn cannot_borrow_as_mutable_more_than_once(name: &str, range: Range) -> Message {
		error_type!("cannot borrow as mutable more than once '{}'", name).range(range)
	}
	#[inline]
	pub fn connot_return_local_rerefence(range: Range) -> Message {
		error_type!("cannot return a reference to a scoped value").range(range)
	}
	#[inline]
	pub fn unsupported_operator(left: String, right: String, operator: &'tce Operator) -> Message {
		error_type!("cannot {} '{}' to '{}'", operator.display(), left, right)
			.range(operator.get_range())
	}

	#[inline]
	pub fn redefine_fn_in_same_scope(name: &str, range: Range) -> Message {
		error_type!("function '{}' is already defined in this scope", name).range(range)
	}

	#[inline]
	pub fn return_local_borrow(range: Range) -> Message {
		error_type!("cannot return a local borrow").range(range)
	}

	#[inline]
	pub fn borrow_expected(range: Range) -> Message {
		error_type!("consider adding a borrow").range(range)
	}

	#[inline]
	pub fn cannot_assign_immutable(name: &str, range: Range) -> Message {
		error_type!("cannot assign immutable '{}'", name).range(range)
	}

	#[inline]
	pub fn not_found_type(name: &'tce str, range: Range) -> Message {
		error_type!("type '{}' not found in current scope", name).range(range)
	}

	#[inline]
	pub fn expect_instaced_type(found: String, range: Range) -> Message {
		error_type!("expected `struct` or `enum`, found '{}'", found).range(range)
	}

	#[inline]
	pub fn not_found_field(name: &'tce str, range: Range) -> Message {
		error_type!("field '{}' not found", name).range(range)
	}

	#[inline]
	pub fn not_impl(found: String, range: Range) -> Message {
		error_type!("'{}' is not implemented", found).range(range)
	}

	#[inline]
	pub fn not_found_method_named(name: String, found: String, range: Range) -> Message {
		error_type!("'{}' has no method named '{}'", found, name).range(range)
	}
	#[inline]
	pub fn not_found_associate_field(name: String, found: String, range: Range) -> Message {
		error_type!("'{}' has no associated field named '{}'", found, name).range(range)
	}

	#[inline]
	pub fn left_hand_cannot_be_assigned(range: Range) -> Message {
		error_type!("left-hand side can't be assigned").range(range)
	}

	// module errors
	#[inline]
	pub fn cannot_reassign_module(range: Range) -> Message {
		error_type!("cannot reassign module").range(range)
	}

	#[inline]
	pub fn main_in_non_main_module(range: Range, pathname: String) -> Message {
		error_type!("non-entry module '{}' contains 'fn main'", pathname).range(range)
	}

	#[inline]
	pub fn missing_main_in_entry_module(range: Range, pathname: String) -> Message {
		error_type!("entry module '{}' must contain 'fn main'", pathname).range(range)
	}

	#[inline]
	pub fn type_annotation_not_allowed_for_module(range: Range) -> Message {
		error_type!("type annotation not allowed for module").range(range)
	}

	#[inline]
	pub fn not_found_pub_item(name: String, range: Range) -> Message {
		error_type!("pub item '{}' not found", name).range(range)
	}

	#[inline]
	pub fn cannot_return_local_reference(range: Range) -> Message {
		error_type!("cannot return a local reference").range(range)
	}

	#[inline]
	pub fn missing_return(expected: String, range: Range) -> Message {
		error_type!("missing return statement, expected type '{}'", expected).range(range)
	}

	#[inline]
	pub fn pattern_type_mismatch(expected: String, range: Range) -> Message {
		error_type!("pattern does not match expected type '{}'", expected).range(range)
	}

	#[inline]
	pub fn not_generic_type(type_name: String, range: Range) -> Message {
		error_type!("'{}' is not a generic type", type_name).range(range)
	}
}
