use anyhow::Result;
use thiserror::Error;

use crate::tokenizer::Token;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Reached unexpected end of file")]
    UnexpectedEOF,

    #[error("Unknown Token: {0}")]
    UnknownToken(Token),
}

impl<T> Into<Result<T>> for ParserError {
    fn into(self) -> Result<T> {
        Err(anyhow::Error::from(self))
    }
}
