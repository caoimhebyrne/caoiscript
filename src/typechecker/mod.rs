use std::ops::Deref;
pub use error::*;

use crate::location::Location;
use crate::parser::{BinaryOperationNode, Literal, Node, SetOperationNode};
use crate::stream::ElementStream;
use crate::typechecker::types::Type;

mod error;
mod types;

pub struct Typechecker {
    stream: ElementStream<Node>,
}

impl Typechecker {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            stream: ElementStream::new(nodes)
        }
    }

    pub fn check(&mut self) -> Vec<TypecheckerError> {
        let mut errors = vec![];

        loop {
            let Some(node) = self.stream.consume() else {
                break;
            };

            let result = Self::typecheck_node(&node);
            match result {
                Err(value) => errors.push(value),
                _ => {}
            }
        }

        errors
    }

    pub fn typecheck_node(node: &Node) -> Result<Type, TypecheckerError> {
        match node {
            Node::Literal(literal, _) => {
                Ok(
                    match literal {
                        Literal::Integer(_) => Type::Integer,
                        Literal::String(_) => Type::String,
                    }
                )
            }

            Node::BinaryOperation(operation, location) =>
                Self::typecheck_binary_operation(operation, location),

            Node::SetOperation(operation, location) =>
                Self::typecheck_set_operation(operation, location)
        }
    }

    pub fn typecheck_binary_operation(operation: &BinaryOperationNode, location: &Location) -> Result<Type, TypecheckerError> {
        let left_type = Self::typecheck_node(operation.left.deref())?;
        let right_type = Self::typecheck_node(operation.right.deref())?;

        if left_type != right_type {
            return TypecheckerError::mismatched_types(&left_type, &right_type, &location).into();
        }

        Ok(left_type)
    }

    pub fn typecheck_set_operation(operation: &SetOperationNode, location: &Location) -> Result<Type, TypecheckerError> {
        let expression = operation.expression.deref();

        let declared_type = Type::from_string(&operation.type_identifier);
        let expression_type = Self::typecheck_node(&expression)?;

        match declared_type {
            Some(value) => {
                if value != expression_type {
                    return TypecheckerError::mismatched_types(&value, &expression_type, expression.location()).into();
                }
            }
            None => return TypecheckerError::invalid_type(&operation.type_identifier, &location).into()
        }

        Ok(expression_type)
    }
}
