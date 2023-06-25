use anyhow::Result;

use error::*;
pub use node::*;

use crate::location::Location;
use crate::stream::ElementStream;
use crate::tokenizer::Token;
use crate::tokenizer::Token::EndOfFile;

mod error;
mod node;

pub struct Parser {
    stream: ElementStream<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            stream: ElementStream::new(tokens)
        }
    }

    pub fn try_parse(&mut self) -> Result<Vec<Node>> {
        let mut nodes = vec![];

        loop {
            let Some(token) = self.stream.peek() else {
                break;
            };

            if let EndOfFile(_) = token{
                break;
            }

            nodes.push(self.try_parse_expression()?);
            self.stream.consume();
        }

        Ok(nodes)
    }

    fn try_parse_expression(&mut self) -> Result<Node> {
        let first_node = self.try_parse_literal()?;

        // The next token decides what kind of operation we should parse.
        let token = self.try_consume()?;
        match token {
            // If the next token is a binary operator operand, we can attempt to parse a binary operator expression.
            Token::Plus(_) => self.try_parse_binary_operation_expression(first_node, token),
            _ => Ok(first_node)
        }
    }

    // (LITERAL) (OPERAND) (LITERAL)
    // NOTE: We trust that the caller has checked that `operand` is of the correct type.
    fn try_parse_binary_operation_expression(&mut self, first_literal: Node, operand: Token) -> Result<Node> {
        let second_literal = self.try_parse_expression()?;

        let operator = match operand {
            Token::Plus(_) => BinaryOperator::Plus,
            Token::Minus(_) => BinaryOperator::Minus,
            Token::Asterisk(_) => BinaryOperator::Multiply,
            Token::Slash(_) => BinaryOperator::Divide,

            _ => return ParserError::UnknownToken(operand).into()
        };

        let binary_operation = BinaryOperationNode {
            left: Box::new(first_literal),
            operator,
            right: Box::new(second_literal),
        };

        Ok(Node::BinaryOperation(binary_operation, operand.location()))
    }

    fn try_parse_literal(&mut self) -> Result<Node> {
        let token = self.try_peek()?;
        let node = match token {
            Token::Integer(value, location) => Node::IntegerLiteral(value, location),
            Token::String(value, location) => Node::StringLiteral(value, location),

            _ => return ParserError::UnknownToken(token).into()
        };

        // We have parsed that token correctly, let's consume it.
        self.stream.consume();

        Ok(node)
    }

    fn try_peek(&mut self) -> Result<Token> {
        let Some(token) = self.stream.peek() else {
            return ParserError::UnexpectedEOF.into();
        };

        Ok(token)
    }

    fn try_consume(&mut self) -> Result<Token> {
        let Some(token) = self.stream.consume() else {
            return ParserError::UnexpectedEOF.into();
        };

        Ok(token)
    }
}
