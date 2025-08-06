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

#[macro_export]
macro_rules! usage_error {
	($($arg:tt)*) => {{
		eprint!("{}usage error:{} ", crate::color::BOLD_RED, crate::color::RESET);
		eprintln!($( $arg )*);
		std::process::exit(-1);
	}}
}
