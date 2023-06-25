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
            Node::Literal(literal, _) =>
                Self::typecheck_literal(literal),

            Node::BinaryOperation(operation, location) =>
                Self::typecheck_binary_operation(operation, location),

            Node::SetOperation(operation, location) =>
                Self::typecheck_set_operation(operation, location)
        }
    }

    // All literals are valid.
    // `<literal>`
    pub fn typecheck_literal(literal: &Literal) -> Result<Type, TypecheckerError> {
        Ok(
            match literal {
                Literal::Integer(_) => Type::Integer,
                Literal::String(_) => Type::String,
            }
        )
    }

    // Binary operations are only valid if the left and right operands are the same type.
    // `<left> + <right>`
    pub fn typecheck_binary_operation(operation: &BinaryOperationNode, location: &Location) -> Result<Type, TypecheckerError> {
        let left_type = Self::typecheck_node(operation.left.deref())?;
        let right_type = Self::typecheck_node(operation.right.deref())?;

        if left_type != right_type {
            return TypecheckerError::mismatched_types(&left_type, &right_type, &location).into();
        }

        Ok(left_type)
    }

    // Set operations are only valid if the expression is the same type as the declared type.
    // The declared type is optional, so we need to check if it exists.
    // `set <name>: <type> = <expression>`
    pub fn typecheck_set_operation(operation: &SetOperationNode, location: &Location) -> Result<Type, TypecheckerError> {
        let expression = operation.expression.deref();
        let expression_type = Self::typecheck_node(&expression)?;

        let Some(type_identifier) = &operation.type_identifier else {
            return Ok(expression_type);
        };

        let declared_type = Type::from_string(&type_identifier);
        match declared_type {
            Some(value) => {
                if value != expression_type {
                    return TypecheckerError::mismatched_types(&value, &expression_type, expression.location()).into();
                }

                Ok(value)
            }
            None => return TypecheckerError::invalid_type(type_identifier, &location).into()
        }
    }
}
