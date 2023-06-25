use crate::location::Location;
use crate::parser::{Literal, Node};
use crate::typechecker::types::Type;

#[derive(Debug, Clone)]
pub struct TypecheckerError {
    pub location: Location,
    pub message: String,
}

impl TypecheckerError {
    pub fn mismatched_types(left: &Type, right: &Type, location: &Location) -> Self {
        Self {
            location: location.clone(),
            message: format!("Mismatched types: {} and {}", left.to_string(), right.to_string()),
        }
    }

    pub fn invalid_type(type_identifier: &str, location: &Location) -> Self {
        Self {
            location: location.clone(),
            message: format!("Invalid type: {}", type_identifier),
        }
    }
}

impl<T> Into<Result<T, TypecheckerError>> for TypecheckerError {
    fn into(self) -> Result<T, TypecheckerError> {
        Err(self)
    }
}

fn node_to_string(node: &Node) -> String {
    match node {
        Node::Literal(literal, _) => {
            match literal {
                Literal::Integer(value) => value.to_string(),
                Literal::String(value) => value.clone(),
            }
        },
        Node::BinaryOperation(_, _) => "BinaryOperation".into(),
        Node::SetOperation(_, _) => "SetOperation".into(),
    }
}
