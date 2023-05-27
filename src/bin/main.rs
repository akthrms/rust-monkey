use rust_monkey::{lexer::Lexer, token::Token};
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let mut lexer = Lexer::new(&line);

                loop {
                    match lexer.next_token() {
                        Token::EOF => break,
                        token => println!("{:?}", token),
                    }
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
