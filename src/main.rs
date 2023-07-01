use std::fs;

use interpreter::Interpreter;
use parser::Parser;
use tokenizer::Tokenizer;
use typechecker::Typechecker;

mod interpreter;
mod location;
mod parser;
mod stream;
// mod tests;
mod tokenizer;
mod typechecker;

fn main() {
    let script = fs::read_to_string("examples/addition.caoi").unwrap();
    let mut tokenizer = Tokenizer::new(script.chars().collect());

    let tokens = tokenizer.process();
    println!("Tokens: {:#?}", tokens);

    let mut parser = Parser::new(tokens);
    let tree = parser.try_parse().unwrap();
    println!("Parser Output: {:#?}", tree);

    let mut typechecker = Typechecker::new(tree.clone());
    let errors = typechecker.check();
    if errors.is_empty() {
        println!("Typechecker is happy!");
    } else {
        println!("Typechecker is sad :(");
        for error in errors {
            let line = error.location.line;
            let column = error.location.column;

            println!("====================");
            println!("Error at line {} column {}:", line, column);
            println!("{}", script.lines().nth(line).unwrap());
            println!("{}^", " ".repeat(column));
            println!("{}{}", " ".repeat(column), error.message);
        }
    }

    println!("Executing interpreter...");

    let mut interpreter = Interpreter::new(tree);
    interpreter.run();
}
