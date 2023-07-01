mod context;
mod value;

use context::Context;
use core::panic;
use std::ops::Deref;

use crate::{
    parser::{AssignmentOperationNode, LetOperationNode, Literal, Node, VariableReference},
    stream::ElementStream,
};

use self::value::Value;

pub struct Interpreter {
    stream: ElementStream<Node>,
}

impl Interpreter {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            stream: ElementStream::new(nodes),
        }
    }

    pub fn run(&mut self) {
        let mut context = Context::new("Root");

        loop {
            let Some(node) = self.stream.consume() else {
                break;
            };

            self.interpret_node(&node, &mut context);
        }

        println!("{}", context);
    }

    fn interpret_node(&mut self, node: &Node, context: &mut Context) -> Value {
        match node {
            Node::Literal(value, _) => self.interpret_literal(value),
            Node::LetOperation(operation, _) => self.interpret_let_operation(operation, context),
            Node::AssignmentOperation(operation, _) => {
                self.interpret_assignment_operation(operation, context)
            }
            Node::Reference(identifier, _) => context.get_variable(identifier).unwrap(),

            _ => panic!("Unable to interpret node: {:#?}", node),
        }
    }

    fn interpret_literal(&mut self, literal: &Literal) -> Value {
        match literal {
            Literal::String(value) => Value::String(value.clone()),
            Literal::Integer(value) => Value::Integer(*value),
        }
    }

    fn interpret_let_operation(
        &mut self,
        operation: &LetOperationNode,
        context: &mut Context,
    ) -> Value {
        let value = self.interpret_node(operation.expression.deref(), context);

        context.set_variable(&operation.name_identifier, value.clone());
        value
    }

    fn interpret_assignment_operation(
        &mut self,
        operation: &AssignmentOperationNode,
        context: &mut Context,
    ) -> Value {
        let identifier = match operation.identifier.clone() {
            VariableReference::Unresolved(identifier) => identifier,
            VariableReference::Typed(identifier, _) => identifier,
        };

        if let None = context.get_variable(&identifier) {
            panic!("Unknown variable: {}", identifier);
        }

        let value = self.interpret_node(operation.expression.deref(), context);
        context.set_variable(&identifier, value.clone());

        value
    }
}
