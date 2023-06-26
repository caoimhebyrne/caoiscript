use anyhow::Result;
use thiserror::Error;

use crate::tokenizer::Token;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Reached unexpected end of file")]
    UnexpectedEOF,

    #[error("Unknown Token: {0}")]
    UnknownToken(Token),

    #[error("Unexpected Token: {0}")]
    UnexpectedToken(Token),

    #[error("Expected Token: {0}")]
    ExpectedToken(String),
}

impl<T> From<ParserError> for Result<T> {
    fn from(value: ParserError) -> Self {
        Err(anyhow::Error::from(value))
    }
}
