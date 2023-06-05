use rust_monkey::{environment::Environment, evaluator::Evaluator, lexer::Lexer, parser::Parser};
use rustyline::DefaultEditor;
use std::{cell::RefCell, rc::Rc};

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");

    let env = Rc::new(RefCell::new(Environment::new()));
    let mut evaluator = Evaluator::new(env);

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
