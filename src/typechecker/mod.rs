use std::mem::discriminant;
use std::ops::Deref;

pub use error::*;
use crate::location::Location;

use crate::parser::{BinaryOperationNode, Node};
use crate::stream::ElementStream;

mod error;

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

            let error = match node {
                Node::BinaryOperation(operation, location) => Self::typecheck_binary_operation(operation, &location),
                _ => None,
            };

            match error {
                Some(value) => errors.push(value),
                None => {}
            }
        }

        errors
    }

    pub fn typecheck_binary_operation(operation: BinaryOperationNode, location: &Location) -> Option<TypecheckerError> {
        let left = operation.left.deref();
        let right = operation.right.deref();

        let left_discriminant = discriminant(left);
        let right_discriminant = discriminant(right);

        if left_discriminant != right_discriminant {
            return Some(TypecheckerError::from_mismatched_nodes(left, right, location));
        }

        None
    }
}
