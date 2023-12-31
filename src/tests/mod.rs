use std::process::exit;

use requirements::TestRequirement;

use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use crate::typechecker::Typechecker;

mod requirements;

pub struct TestRunner {
    name: String,
    script: String,
    requirements: Vec<TestRequirement>,
}

impl TestRunner {
    pub fn new(name: String, script: String) -> Self {
        Self {
            name,
            script: script.clone(),
            requirements: TestRequirement::parse(script),
        }
    }

    pub fn run(&self) {
        if self.requirements.is_empty() {
            println!("❌ `{}` has no requirements!", self.name);
            return;
        }

        println!("🏃‍♀️ Running test `{}`", self.name);

        let mut tokenizer = Tokenizer::new(self.script.chars().collect());
        let tokens = tokenizer.process();

        println!("🎫 Tokens: {:#?}", tokens);

        let mut parser = Parser::new(tokens);
        let tree = parser.try_parse().unwrap();

        println!("🌳 AST: {:#?}", tree);

        let mut typechecker = Typechecker::new(tree);
        let errors = typechecker.check();

        for requirement in &self.requirements {
            match requirement {
                TestRequirement::TypecheckerPass => {
                    if errors.is_empty() {
                        println!("✅ `{}` passed!", self.name);
                    } else {
                        println!("❌ `{}` failed!", self.name);
                        for error in &errors {
                            let line = error.location.line;
                            let column = error.location.column;

                            println!("====================");
                            println!("Error at line {} column {}:", line + 1, column.saturating_sub(0));
                            println!("{}", self.script.lines().nth(line).unwrap());
                            println!("{}^", " ".repeat(column));
                            println!("{}{}", " ".repeat(column), error.message);
                            exit(-1);
                        }
                    }
                }

                TestRequirement::TypecheckerFail => {
                    if errors.is_empty() {
                        println!("❌ `{}` failed!", self.name);
                        exit(-1);
                    } else {
                        println!("✅ `{}` passed!", self.name);
                    }
                }
            }
        }
    }
}
