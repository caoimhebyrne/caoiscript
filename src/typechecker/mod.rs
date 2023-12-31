use std::ops::Deref;

pub use error::*;
pub use types::*;

use crate::location::Location;
use crate::parser::{BinaryOperationNode, LetOperationNode, Literal, Node};
use crate::stream::ElementStream;

pub mod error;
pub mod types;

pub struct Typechecker {
    stream: ElementStream<Node>,
}

impl Typechecker {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            stream: ElementStream::new(nodes),
        }
    }

    pub fn check(&mut self) -> Vec<TypecheckerError> {
        let mut errors = vec![];

        loop {
            let Some(node) = self.stream.consume() else {
                break;
            };

            let result = Self::typecheck_node(&node);
            if let Err(value) = result {
                errors.push(value)
            }
        }

        errors
    }

    pub fn typecheck_node(node: &Node) -> Result<Type, TypecheckerError> {
        match node {
            Node::Literal(literal, _) => Self::typecheck_literal(literal),

            Node::BinaryOperation(operation, location) => {
                Self::typecheck_binary_operation(operation, location)
            }

            Node::LetOperation(operation, location) => {
                Self::typecheck_let_operation(operation, location)
            }

            Node::AssignmentOperation(operation, _) => {
                Self::typecheck_node(operation.expression.deref())
            }

            Node::Reference(_, _) => Ok(Type::None),
        }
    }

    // All literals are valid.
    // `<literal>`
    pub fn typecheck_literal(literal: &Literal) -> Result<Type, TypecheckerError> {
        Ok(match literal {
            Literal::Integer(_) => Type::Integer,
            Literal::String(_) => Type::String,
        })
    }

    // Binary operations are only valid if the left and right operands are the same type.
    // `<left> + <right>`
    pub fn typecheck_binary_operation(
        operation: &BinaryOperationNode,
        location: &Location,
    ) -> Result<Type, TypecheckerError> {
        let left_type = Self::typecheck_node(operation.left.deref())?;
        let right_type = Self::typecheck_node(operation.right.deref())?;

        if left_type != right_type {
            return TypecheckerError::mismatched_types(&left_type, &right_type, location).into();
        }

        Ok(left_type)
    }

    // Let operations are only valid if the expression is the same type as the declared type.
    // The declared type is optional, so we need to check if it exists.
    // `let <name>: <type> = <expression>`
    pub fn typecheck_let_operation(
        operation: &LetOperationNode,
        location: &Location,
    ) -> Result<Type, TypecheckerError> {
        let expression = operation.expression.deref();
        let expression_type = Self::typecheck_node(expression)?;

        let Some(type_identifier) = &operation.type_identifier else {
            return Ok(expression_type);
        };

        let declared_type = Type::from_string(type_identifier);
        match declared_type {
            Some(value) => {
                if value != expression_type {
                    return TypecheckerError::mismatched_types(
                        &value,
                        &expression_type,
                        expression.location(),
                    )
                    .into();
                }

                Ok(value)
            }
            None => TypecheckerError::invalid_type(type_identifier, location).into(),
        }
    }
}
