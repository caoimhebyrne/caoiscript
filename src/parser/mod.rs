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
        }

        Ok(nodes)
    }

    fn try_parse_expression(&mut self) -> Result<Node> {
        let first_node = self.try_parse_literal()?;

        // The next token decides what kind of operation we should parse.
        let token = self.try_peek();
        if let Err(_) = token {
            return Ok(first_node);
        }

        let token = token.unwrap();

        let result = match token {
            // If the next token is a binary operator operand, we can attempt to parse a binary operator expression.
            Token::Plus(_) |
            Token::Asterisk(_) |
            Token::Slash(_) |
            Token::Minus(_) => {
                self.try_consume()?;
                self.try_parse_binary_operation_expression(first_node, token)
            },

            // If we don't recognize the next token, we can assume that the expression is complete.
            _ => Ok(first_node)
        };

        result
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
            Token::Integer(value, location) =>
                Node::Literal(Literal::Integer(value), location),

            Token::String(value, location) =>
                Node::Literal(Literal::String(value), location),

            Token::Keyword(keyword, location) => match keyword {
                Keyword::Let => self.try_parse_let_expression(location)?,
            }

            _ => return ParserError::UnknownToken(token).into()
        };

        Ok(node)
    }

    // let <identifier>(: <type>)= <expression>
    fn try_parse_let_expression(&mut self, location: Location) -> Result<Node> {
        let name_identifier = self.try_consume_identifier()?;

        // If the next token is an equals sign, we can parse the expression.
        // If the next token is a colon, we can parse the type and then the expression.
        let token = self.try_consume()?;
        return match token {
            Token::Equals(_) => self.try_parse_inferred_let_expression(name_identifier, location),
            Token::Colon(_) => self.try_parse_typed_let_expression(name_identifier, location),

            _ => ParserError::UnexpectedToken(token).into()
        };
    }

    // let <identifier> = <expression>
    fn try_parse_inferred_let_expression(&mut self, name_identifier: String, location: Location) -> Result<Node> {
        let expression = self.try_parse_expression()?;

        let let_operation = LetOperationNode {
            name_identifier,
            type_identifier: None,
            expression: Box::new(expression),
        };

        Ok(Node::LetOperation(let_operation, location))
    }

    // let <identifier>: <type> = <expression>
    fn try_parse_typed_let_expression(&mut self, name_identifier: String, location: Location) -> Result<Node> {
        // The identifier denotes what type the expression result should be.
        let type_identifier = self.try_consume_identifier()?;

        // Equals indicates that an expression is next.
        let token = self.try_consume()?;
        let Token::Equals(_) = token else {
            return ParserError::ExpectedToken("=".into()).into();
        };

        let expression = self.try_parse_expression()?;

        let let_operation = LetOperationNode {
            name_identifier,
            type_identifier: Some(type_identifier),
            expression: Box::new(expression),
        };

        Ok(Node::LetOperation(let_operation, location))
    }

    // Attempts to consume and parse an identifier token.
    fn try_consume_identifier(&mut self) -> Result<String> {
        let token = self.try_consume()?;
        let identifier = match token {
            Token::Identifier(value, _) => value,
            _ => return ParserError::UnexpectedToken(token).into()
        };

        Ok(identifier)
    }

    fn try_consume(&mut self) -> Result<Token> {
        let Some(token) = self.stream.consume() else {
            return ParserError::UnexpectedEOF.into();
        };

        Ok(token)
    }

    fn try_peek(&mut self) -> Result<Token> {
        let Some(token) = self.stream.peek() else {
            return ParserError::UnexpectedEOF.into();
        };

        Ok(token)
    }
}
