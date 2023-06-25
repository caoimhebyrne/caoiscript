use crate::location::Location;
use crate::parser::Node;

#[derive(Debug, Clone)]
pub struct TypecheckerError {
    pub location: Location,
    pub message: String,
}

impl TypecheckerError {
    pub fn from_mismatched_nodes(left: &Node, right: &Node, location: &Location) -> Self {
        let left_type = node_to_string(left);
        let right_type = node_to_string(right);

        Self {
            location: location.clone(),
            message: format!("Mismatched types: {} and {}", left_type, right_type),
        }
    }
}

fn node_to_string(node: &Node) -> String {
    match node {
        Node::StringLiteral(_, _) => "String".into(),
        Node::IntegerLiteral(_, _) => "Integer".into(),
        Node::BinaryOperation(_, _) => "BinaryOperation".into(),
        Node::SetOperation(_, _) => "SetOperation".into(),
    }
}
