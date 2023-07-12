//! oxttl parsing toolkit.
//!
//! Provides the basic code to write plain Rust lexers and parsers able to read files chunk by chunk.

mod lexer;
mod parser;

pub use self::lexer::{Lexer, LexerError, TokenRecognizer, TokenRecognizerError};
#[cfg(feature = "async-tokio")]
pub use self::parser::FromTokioAsyncReadIterator;
pub use self::parser::{
    FromReadIterator, ParseError, Parser, RuleRecognizer, RuleRecognizerError, SyntaxError,
};