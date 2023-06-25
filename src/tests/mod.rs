mod requirements;

use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use crate::typechecker::Typechecker;

pub struct TestRunner {
    script: String
}

impl TestRunner {
    pub fn new(script: String) -> Self {
        Self { script }
    }

    pub fn run(&self) {
        let mut tokenizer = Tokenizer::new(self.script.chars().collect());
        let tokens = tokenizer.process();

        let mut parser = Parser::new(tokens);
        let tree = parser.try_parse().unwrap();

        let mut typechecker = Typechecker::new(tree);
        let errors = typechecker.check();


    }
}
