use rust_monkey::{evaluator::Evaluator, lexer::Lexer, parser::Parser};
use rustyline::DefaultEditor;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");

    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let lexer = Lexer::new(&line);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                if !parser.errors().is_empty() {
                    println!("Woops! We ran into some monkey business here!");
                    println!(" parse error:");
                    for err in parser.errors() {
                        println!("\t{}", err);
                    }
                    continue;
                }
                let mut evaluator = Evaluator::new();
                if let Some(evaluated) = evaluator.eval(program) {
                    println!("{}", evaluated);
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
