use crate::{resolver::file::SourceFile, span::Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
	Error,
	Warning,
}

#[derive(Debug)]
pub struct Message {
	pub kind: MessageKind,
	pub text: String,
	pub span: Option<Span>,
	pub notes: Vec<Note>,
}

impl Message {
	pub fn error(text: impl Into<String>) -> Self {
		Self { kind: MessageKind::Error, text: text.into(), span: None, notes: vec![] }
	}

	pub fn warning(text: impl Into<String>) -> Self {
		Self { kind: MessageKind::Warning, text: text.into(), span: None, notes: vec![] }
	}

	pub fn with_span(mut self, span: Span) -> Self {
		self.span = Some(span);
		self
	}

	pub fn with_note(mut self, note: Note) -> Self {
		self.notes.push(note);
		self
	}

	pub fn with_span_if_some(mut self, span: Option<Span>) -> Message {
		self.span = self.span.or(span);
		self
	}

	pub fn with_note_if_some(mut self, span: Option<Span>, text: &str) -> Message {
		if let Some(note) = Note::maybe_new(text, span) {
			self.notes.push(note);
		}
		self
	}
}

#[derive(Debug)]
pub struct Note {
	pub text: String,
	pub span: Span,
}

impl Note {
	pub fn new(span: Span, text: impl Into<String>) -> Self {
		Self { span, text: text.into() }
	}
	pub fn maybe_new(text: &str, span: Option<Span>) -> Option<Note> {
		Some(Note { text: text.to_owned(), span: span? })
	}
}

// Messages
//
#[derive(Debug)]
pub struct Messages<'a> {
	pub list: Vec<Message>,
	pub file: &'a SourceFile,
}

impl<'a> Messages<'a> {
	pub fn new(file: &'a SourceFile) -> Self {
		Self { list: vec![], file }
	}

	pub fn message(&mut self, message: Message) {
		self.list.push(message);
	}

	pub fn has_errors(&self) -> bool {
		self.list.iter().any(|m| m.kind == MessageKind::Error)
	}

	pub fn print(&self, stage: &str) {
		for message in &self.list {
			println!("[{}] {}: {}", stage, message.kind.name(), message.text);
			if let Some(span) = message.span {
				println!("  --> at {}", span);
			}
			for note in &message.notes {
				println!("   = note: {} at {}", note.text, note.span);
			}
		}
	}
}

impl MessageKind {
	pub fn name(self) -> &'static str {
		match self {
			MessageKind::Error => "error",
			MessageKind::Warning => "warning",
		}
	}
}
