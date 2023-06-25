use tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::typechecker::Typechecker;

mod parser;
mod stream;
mod tokenizer;
mod typechecker;
mod location;

fn main() {
    let script = "set x: Integer = 4";
    let mut tokenizer = Tokenizer::new(script.chars().collect());

    let tokens = tokenizer.process();
    println!("Input: '{script}'");
    println!("Tokens: {:#?}", tokens);

    let mut parser = Parser::new(tokens);
    let tree = parser.try_parse().unwrap();
    println!("Parser Output: {:#?}", tree);

    let mut typechecker = Typechecker::new(tree);
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
}
