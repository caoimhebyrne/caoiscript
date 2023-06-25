use std::{
    fmt,
    fmt::Display,
    fmt::Formatter,
};
use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Set,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword, Location),

    Integer(u32, Location),
    String(String, Location),
    Identifier(String, Location),

    Plus(Location),
    Minus(Location),
    Slash(Location),
    Asterisk(Location),

    Equals(Location),
    Colon(Location),

    EndOfFile(Location),
}

impl Token {
    pub fn location(self) -> Location {
        match self {
            Token::Keyword(_, location) => location,

            Token::Integer(_, location) => location,
            Token::String(_, location) => location,
            Token::Identifier(_, location) => location,

            Token::Plus(location) => location,
            Token::Minus(location) => location,
            Token::Slash(location) => location,
            Token::Asterisk(location) => location,

            Token::Equals(location) => location,
            Token::Colon(location) => location,

            Token::EndOfFile(location) => location,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}