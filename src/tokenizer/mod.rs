pub use token::*;

use crate::location::Location;
use crate::stream::ElementStream;

pub mod token;

pub struct Tokenizer {
    stream: ElementStream<char>,
    new_lines: usize,
    last_line_length: usize,
}

impl Tokenizer {
    pub fn new(script: String) -> Self {
        let lines: Vec<&str> = script
            .split("\n")
            .filter(|line| !line.starts_with("#"))
            .collect();

        let characters = lines.join("\n").chars().collect();
        Self {
            stream: ElementStream::new(characters),
            new_lines: 0,
            last_line_length: 0,
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
                column: self.stream.index - self.last_line_length,
            };

            let mut should_consume = true;
            let token = match character {
                '+' => Some(Token::Plus(location)),
                '-' => Some(Token::Minus(location)),
                '*' => Some(Token::Asterisk(location)),
                '/' => Some(Token::Slash(location)),
                ':' => Some(Token::Colon(location)),
                '=' => Some(Token::Equals(location)),

                '"' => {
                    self.stream.consume();
                    should_consume = false;

                    self.parse_string(location)
                }

                '\n' => {
                    self.stream.consume();
                    self.new_lines += 1;
                    self.last_line_length = self.stream.index;

                    continue;
                }

                _ => {
                    if character.is_numeric() {
                        should_consume = false;
                        self.parse_integer(location)
                    } else if character.is_alphabetic() || character == '_' {
                        let identifier = self.read_string(|c| !c.is_alphabetic() && c != '_');
                        should_consume = false;

                        match Self::parse_keyword(&identifier, &location) {
                            Some(value) => Some(value),
                            None => Some(Token::Identifier(identifier, location)),
                        }
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

            self.stream.consume();
            characters.push(character);
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
        let value = self.read_string(|c| c == '"');
        Some(Token::String(value, location))
    }

    fn read_string(&mut self, end_predicate: fn(char) -> bool) -> String {
        let mut characters: Vec<char> = vec![];

        loop {
            let Some(character) = self.stream.peek() else {
                break;
            };

            if end_predicate(character) {
                break;
            }

            self.stream.consume();
            characters.push(character);
        }

        characters.into_iter().collect()
    }

    fn parse_keyword(identifier: &String, location: &Location) -> Option<Token> {
        let keyword = match identifier.as_str() {
            "let" => Keyword::Let,
            _ => return None,
        };

        Some(Token::Keyword(keyword, location.clone()))
    }
}
