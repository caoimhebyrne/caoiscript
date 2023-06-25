pub use token::*;

use crate::location::Location;
use crate::stream::ElementStream;

pub mod token;

pub struct Tokenizer {
    stream: ElementStream<char>,
    new_lines: usize,
}

impl Tokenizer {
    pub fn new(characters: Vec<char>) -> Self {
        Self {
            stream: ElementStream::new(characters),
            new_lines: 0,
        }
    }

    pub fn process(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        loop {
            let Some(character) = self.stream.peek() else {
                break;
            };

            let location = Location {
                line: self.new_lines,
                column: self.stream.index,
            };

            let mut should_consume = true;
            let token = match character {
                '+' => Some(Token::Plus(location)),
                '-' => Some(Token::Minus(location)),
                '*' => Some(Token::Asterisk(location)),
                '/' => Some(Token::Slash(location)),

                '"' => {
                    self.stream.consume();
                    should_consume = false;

                    self.parse_string(location)
                }

                '\n' => {
                    self.stream.consume();
                    self.new_lines += 1;

                    continue;
                }

                _ => {
                    if character.is_numeric() {
                        self.parse_integer(location)
                    } else {
                        None
                    }
                }
            };

            match token {
                Some(value) => tokens.push(value),
                None => {}
            };

            if should_consume {
                self.stream.consume();
            }
        }

        let location = Location {
            line: self.new_lines,
            column: self.stream.index,
        };

        tokens.push(Token::EndOfFile(location));
        return tokens;
    }

    fn parse_integer(&mut self, location: Location) -> Option<Token> {
        let mut characters: Vec<char> = vec![];

        loop {
            let Some(character) = self.stream.peek() else {
                break;
            };

            if !character.is_numeric() {
                break;
            }

            characters.push(character);
            self.stream.consume();
        }

        let parsed_value = characters
            .into_iter()
            .map(|char| char.to_digit(10))
            .try_fold(0, |ans, i| i.map(|i| ans * 10 + i));

        match parsed_value {
            Some(value) => Some(Token::Integer(value, location)),
            None => None,
        }
    }

    fn parse_string(&mut self, location: Location) -> Option<Token> {
        let mut characters: Vec<char> = vec![];

        loop {
            let Some(character) = self.stream.consume() else {
                break;
            };

            if character == '"' {
                break;
            }

            characters.push(character);
        }

        let value = characters.into_iter().collect();
        Some(Token::String(value, location))
    }
}
