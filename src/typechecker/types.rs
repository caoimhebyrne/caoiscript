use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    String,
    None,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "Integer"),
            Type::String => write!(f, "String"),
            Type::None => write!(f, "None"),
        }
    }
}

impl Type {
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "Integer" => Some(Type::Integer),
            "String" => Some(Type::String),
            _ => None,
        }
    }
}
