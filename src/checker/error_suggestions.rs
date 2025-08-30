use crate::{
	message::Message,
	range::Range,
};

use super::{
	types::{Type, TypeId},
	Checker,
};

pub struct TypeErrorSuggestion {
	pub message: String,
	pub code_hint: Option<String>,
}

impl TypeErrorSuggestion {
	pub fn new(message: impl Into<String>) -> Self {
		Self {
			message: message.into(),
			code_hint: None,
		}
	}

	pub fn with_code_hint(mut self, hint: impl Into<String>) -> Self {
		self.code_hint = Some(hint.into());
		self
	}
}

impl<'ckr> Checker<'ckr> {
	pub fn suggest_type_fix(
		&self,
		expected: TypeId,
		found: TypeId,
		range: Range,
	) -> Vec<TypeErrorSuggestion> {
		let mut suggestions = Vec::new();
		
		let expected_type = self.lookup_stored_type(expected);
		let found_type = self.lookup_stored_type(found);
		
		match (expected_type, found_type) {
			(Type::Borrow(expected_borrow), Type::Borrow(found_borrow)) => {
				if expected_borrow.mutable && !found_borrow.mutable {
					suggestions.push(
						TypeErrorSuggestion::new("Consider using a mutable borrow")
							.with_code_hint("&mut")
					);
				} else if !expected_borrow.mutable && found_borrow.mutable {
					suggestions.push(
						TypeErrorSuggestion::new("Consider using an immutable borrow")
							.with_code_hint("&")
					);
				}
			}
			(Type::Borrow(_), _) => {
				suggestions.push(
					TypeErrorSuggestion::new("Consider borrowing the value")
						.with_code_hint(format!("&{}", self.display_type(found)))
				);
			}
			(_, Type::Borrow(borrow)) => {
				suggestions.push(
					TypeErrorSuggestion::new("Consider dereferencing the value")
						.with_code_hint("*")
				);
			}
			(Type::Number(expected_num), Type::Number(found_num)) => {
				suggestions.push(
					TypeErrorSuggestion::new(format!(
						"Consider casting from {} to {}",
						self.display_type(found),
						self.display_type(expected)
					))
					.with_code_hint(format!("as {}", self.display_type(expected)))
				);
			}
			(Type::Str, Type::String) | (Type::String, Type::Str) => {
				suggestions.push(
					TypeErrorSuggestion::new("String and str are different types")
				);
				if matches!(expected_type, Type::Str) {
					suggestions.push(
						TypeErrorSuggestion::new("Consider using .as_str() to convert String to str")
							.with_code_hint(".as_str()")
					);
				} else {
					suggestions.push(
						TypeErrorSuggestion::new("Consider using .to_string() to convert str to String")
							.with_code_hint(".to_string()")
					);
				}
			}
			(Type::Fn(expected_fn), Type::Fn(found_fn)) => {
				if expected_fn.args.len() != found_fn.args.len() {
					suggestions.push(
						TypeErrorSuggestion::new(format!(
							"Function expects {} arguments but found {}",
							expected_fn.args.len(),
							found_fn.args.len()
						))
					);
				} else {
					for (i, (exp_arg, found_arg)) in expected_fn.args.iter().zip(found_fn.args.iter()).enumerate() {
						if exp_arg != found_arg {
							suggestions.push(
								TypeErrorSuggestion::new(format!(
									"Argument {} has wrong type: expected {}, found {}",
									i + 1,
									self.display_type(*exp_arg),
									self.display_type(*found_arg)
								))
							);
						}
					}
				}
				
				if expected_fn.ret != found_fn.ret {
					suggestions.push(
						TypeErrorSuggestion::new(format!(
							"Return type mismatch: expected {}, found {}",
							self.display_type(expected_fn.ret),
							self.display_type(found_fn.ret)
						))
					);
				}
			}
			_ => {
				// Generic suggestion for other type mismatches
				if self.can_coerce(found, expected).unwrap_or(false) {
					suggestions.push(
						TypeErrorSuggestion::new("Type can be coerced automatically")
					);
				}
			}
		}
		
		suggestions
	}

	pub fn suggest_similar_names(&self, name: &str, available: &[String]) -> Vec<String> {
		let mut similar = Vec::new();
		
		for available_name in available {
			let distance = levenshtein_distance(name, available_name);
			if distance <= 2 {
				similar.push(available_name.clone());
			}
		}
		
		similar.sort_by_key(|n| levenshtein_distance(name, n));
		similar.truncate(3);
		similar
	}

	pub fn enhance_error_with_suggestions(&self, mut error: Message, suggestions: Vec<TypeErrorSuggestion>) -> Message {
		use crate::message::Note;
		
		for suggestion in suggestions {
			let mut help_text = suggestion.message;
			if let Some(hint) = suggestion.code_hint {
				help_text.push_str(&format!(" - try: {}", hint));
			}
			let note = Note::new(help_text);
			error = error.note(note);
		}
		error
	}
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
	let len1 = s1.len();
	let len2 = s2.len();
	
	if len1 == 0 {
		return len2;
	}
	if len2 == 0 {
		return len1;
	}
	
	let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
	
	for i in 0..=len1 {
		matrix[i][0] = i;
	}
	for j in 0..=len2 {
		matrix[0][j] = j;
	}
	
	for (i, c1) in s1.chars().enumerate() {
		for (j, c2) in s2.chars().enumerate() {
			let cost = if c1 == c2 { 0 } else { 1 };
			matrix[i + 1][j + 1] = std::cmp::min(
				std::cmp::min(
					matrix[i][j + 1] + 1,      // deletion
					matrix[i + 1][j] + 1        // insertion
				),
				matrix[i][j] + cost             // substitution
			);
		}
	}
	
	matrix[len1][len2]
}