use std::fs;

use tests::TestRunner;

mod parser;
mod stream;
mod tokenizer;
mod typechecker;
mod location;
mod tests;

fn main() {
    let test_files = fs::read_dir("tests").unwrap();
    for file in test_files {
        let entry = file.unwrap();
        if !entry.file_type().unwrap().is_file() {
            continue
        }

        let path = entry.path();
        let test = TestRunner::new(path.file_name().unwrap().to_str().unwrap().into(), fs::read_to_string(path).unwrap());
        test.run();
    }
}

// let script = "let x: Integer = 4";
// let mut tokenizer = Tokenizer::new(script.chars().collect());
//
// let tokens = tokenizer.process();
// println!("Input: '{script}'");
// println!("Tokens: {:#?}", tokens);
//
// let mut parser = Parser::new(tokens);
// let tree = parser.try_parse().unwrap();
// println!("Parser Output: {:#?}", tree);
//
// let mut typechecker = Typechecker::new(tree);
// let errors = typechecker.check();
// if errors.is_empty() {
//     println!("Typechecker is happy!");
// } else {
//     println!("Typechecker is sad :(");
//     for error in errors {
//         let line = error.location.line;
//         let column = error.location.column;
//
//         println!("====================");
//         println!("Error at line {} column {}:", line, column);
//         println!("{}", script.lines().nth(line).unwrap());
//         println!("{}^", " ".repeat(column));
//         println!("{}{}", " ".repeat(column), error.message);
//     }
// }
