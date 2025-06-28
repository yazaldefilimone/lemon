use crate::{ast::Span, loader::SourceFile};

pub type ParseResult<T> = std::result::Result<T, ()>;

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => {
		$crate::messages::Message::error(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! warning {
	($($arg:tt)*) => {
		$crate::messages::Message::warning(format!( $($arg)* ))
	}
}

#[macro_export]
macro_rules! note {
	($span:expr, $($arg:tt)*) => {
		$crate::messages::Note::new($span, format!( $($arg)* ))
	}
}

#[derive(Debug)]
pub struct Messages<'a> {
	pub messages: Vec<Message>,
	any_errors: bool,
	module_path: &'a [String],
}

impl<'a> Messages<'a> {
	pub fn new(module_path: &'a [String]) -> Self {
		Messages { messages: Vec::new(), any_errors: false, module_path }
	}

	pub fn module_path(&self) -> &'a [String] {
		self.module_path
	}

	pub fn any_messages(&self) -> bool {
		!self.messages.is_empty()
	}

	pub fn message(&mut self, message: Message) {
		self.any_errors |= message.kind == MessageKind::Error;
		self.messages.push(message);
	}
}

#[derive(Debug)]
pub struct RootMessages<'a> {
	file_messages: Vec<Messages<'a>>,
	main_function_span: Option<Span>,
	any_errors: bool,
	sources: &'a [SourceFile],
}

impl<'a> RootMessages<'a> {
	pub fn new(sources: &'a [SourceFile]) -> Self {
		RootMessages { file_messages: Vec::new(), main_function_span: None, any_errors: false, sources }
	}

	pub fn mark_main_found(&mut self, span: Span) {
		self.main_function_span = Some(span);
	}

	pub fn print_messages(&mut self, stage: &str, check_main: bool) -> bool {
		true
	}

	pub fn reset(&mut self) {
		self.file_messages.clear();
		self.main_function_span = None;
		self.any_errors = false;
	}

	pub fn any_errors(&self) -> bool {
		self.any_errors
	}

	pub fn any_messages(&self) -> bool {
		!self.file_messages.is_empty() || self.any_errors
	}

	pub fn add_messages_if_any(&mut self, messages: Messages<'a>) {
		if messages.any_messages() {
			self.any_errors |= messages.any_errors;
			self.file_messages.push(messages);
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageKind {
	Error,
	Warning,
}

impl MessageKind {
	fn name(self) -> &'static str {
		match self {
			MessageKind::Error => "error",
			MessageKind::Warning => "warning",
		}
	}
}

#[derive(Debug)]
pub struct Message {
	kind: MessageKind,
	text: String,
	span: Option<Span>,
	notes: Vec<Note>,
}

impl Message {
	pub fn error(text: String) -> Message {
		Message { kind: MessageKind::Error, text, span: None, notes: Vec::new() }
	}

	pub fn warning(text: String) -> Message {
		Message { kind: MessageKind::Warning, text, span: None, notes: Vec::new() }
	}

	pub fn span(mut self, span: Span) -> Message {
		self.span = Some(span);
		self
	}

	pub fn span_if_some(mut self, span: Option<Span>) -> Message {
		self.span = self.span.or(span);
		self
	}

	pub fn note(mut self, note: Note) -> Message {
		self.notes.push(note);
		self
	}

	pub fn note_if_some(mut self, span: Option<Span>, text: &str) -> Message {
		if let Some(note) = Note::maybe_new(text, span) {
			self.notes.push(note);
		}
		self
	}

	pub fn print(&self, sources: &[SourceFile], stage: &str) {
		self.print_message(sources, stage);
	}

	fn print_message(&self, sources: &[SourceFile], stage: &str) {}
}

#[derive(Debug)]
pub struct Note {
	text: String,
	span: Span,
}

impl Note {
	pub fn new(span: Span, text: String) -> Note {
		Note { text, span }
	}

	pub fn maybe_new(text: &str, span: Option<Span>) -> Option<Note> {
		Some(Note { text: text.to_owned(), span: span? })
	}
}
