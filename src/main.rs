use errors::ScanErr;
use tokenizer::{Tokenizer, Token};

mod errors;
mod tokenizer;

fn main() {
    match run() {
        Ok(_) => {},
        Err(err) => eprintln!("{}", err)
    }
}

fn run() -> Result<(), ScanErr> {
    let code = "266 + 3 * 44 - 5 / 222".to_string();

    let mut lexer = Tokenizer { code, start: 0, current: 0 };

    let line: Vec<Token> = lexer.scan_tokens()?;
    
    for lexeme in &line {
        println!("{:?}", lexeme);
    }

    Ok(())
}