use rust_monkey::{lexer::Lexer, parser::Parser};
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let lexer = Lexer::new(&line);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                println!("{:?}", program)
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
