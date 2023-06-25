use anyhow::Result;

use error::*;
pub use node::*;

use crate::location::Location;
use crate::stream::ElementStream;
use crate::tokenizer::{Keyword, Token};
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

            if let EndOfFile(_) = token {
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
        let token = self.try_consume();
        match token {
            Ok(value) => match value {
                // If the next token is a binary operator operand, we can attempt to parse a binary operator expression.
                Token::Plus(_) => self.try_parse_binary_operation_expression(first_node, value),
                _ => Ok(first_node)
            }

            // If there are no more tokens, we can return the first node.
            Err(_) => Ok(first_node)
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
        let token = self.try_consume()?;

        let node = match token {
            Token::Integer(value, location) => Node::Literal(Literal::Integer(value), location),
            Token::String(value, location) => Node::Literal(Literal::String(value), location),

            Token::Keyword(keyword, location) => match keyword {
                Keyword::Set => self.try_parse_set_expression(location)?,
            }

            _ => return ParserError::UnknownToken(token).into()
        };

        Ok(node)
    }

    // set <identifier>: <type> = <expression>
    fn try_parse_set_expression(&mut self, location: Location) -> Result<Node> {
        let token = self.try_consume()?;
        let name_identifier = match token {
            Token::Identifier(value, _) => value,
            _ => return ParserError::UnexpectedToken(token).into()
        };

        let token = self.try_consume()?;
        let Token::Colon(_) = token else {
            return ParserError::ExpectedToken(":".into()).into();
        };

        let token = self.try_consume()?;
        let type_identifier = match token {
            Token::Identifier(value, _) => value,
            _ => return ParserError::UnexpectedToken(token).into()
        };

        let token = self.try_consume()?;
        let Token::Equals(_) = token else {
            return ParserError::ExpectedToken("=".into()).into();
        };

        let expression = self.try_parse_expression()?;

        let set_operation = SetOperationNode {
            name_identifier,
            type_identifier,
            expression: Box::new(expression),
        };

        Ok(Node::SetOperation(set_operation, location))
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
