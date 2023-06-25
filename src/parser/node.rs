use crate::location::Location;

#[derive(Debug, Clone)]
pub enum Node {
    // An integer literal, e.g. 1
    IntegerLiteral(u32, Location),

    // A string literal, e.g. Hello, World!
    StringLiteral(String, Location),

    // A binary operation (e.g. 1 + 3)
    BinaryOperation(BinaryOperationNode, Location),
}

// impl Node {
//     pub fn location(&self) -> &Location {
//         match self {
//             Node::IntegerLiteral(_, location) => location,
//             Node::StringLiteral(_, location) => location,
//             Node::BinaryOperation(_, location) => location,
//         }
//     }
// }

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
