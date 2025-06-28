mod lexer;
pub mod token;
mod token_kind;
pub use lexer::Lexer;
pub use token_kind::TokenKind;
pub type LexResult<T> = std::result::Result<T, ()>;
