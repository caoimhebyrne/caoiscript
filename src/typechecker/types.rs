#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    String,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Integer => "Integer".into(),
            Type::String => "String".into(),
        }
    }

    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "Integer" => Some(Type::Integer),
            "String" => Some(Type::String),
            _ => None,
        }
    }
}
