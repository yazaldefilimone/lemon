#[macro_export]
macro_rules! error {
	($msg:expr) => {
		$crate::messages::Message::error($msg)
	};
}

#[macro_export]
macro_rules! warning {
	($msg:expr) => {
		$crate::messages::Message::warning($msg)
	};
}

#[macro_export]
macro_rules! note {
	($span:expr, $msg:expr) => {
		$crate::messages::Note::new($span, $msg)
	};
}

#[macro_export]
macro_rules! usage_error {
	($($arg:tt)*) => {{
		eprint!("{}usage error:{} ", crate::color::BOLD_RED, crate::color::RESET);
		eprintln!($( $arg )*);
		std::process::exit(-1);
	}}
}
