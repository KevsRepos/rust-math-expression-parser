use crate::core::errors::token_err::TokenErr;

use crate::core::lexer::tokenizer::Lexer;

mod core;

fn main() {
    match run() {
        Ok(_) => {},
        Err(err) => eprintln!("{}", err)
    }
}

fn run() -> Result<(), TokenErr> {
    let code = "1 + g1 - 2 * 5 + 88".to_string();

    let lexer = Lexer { code };

    let line = lexer.tokenize()?;

    for lexeme in &line {
        println!("{:?}", lexeme);
    }

    Ok(())
}