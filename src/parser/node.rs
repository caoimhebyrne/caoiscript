use crate::location::Location;

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(u32),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Node {
    Literal(Literal, Location),

    // A binary operation (e.g. 1 + 3)
    BinaryOperation(BinaryOperationNode, Location),

    // A set operation (e.g. set x: Integer = 1)
    SetOperation(SetOperationNode, Location),
}

impl Node {
    pub fn location(&self) -> &Location {
        match self {
            Node::Literal(_, location) => location,
            Node::BinaryOperation(_, location) => location,
            Node::SetOperation(_, location) => location,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct BinaryOperationNode {
    pub left: Box<Node>,
    pub operator: BinaryOperator,
    pub right: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct SetOperationNode {
    pub name_identifier: String,
    pub type_identifier: Option<String>,
    pub expression: Box<Node>,
}
