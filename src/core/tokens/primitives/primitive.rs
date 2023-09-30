use crate::core::errors::token_err::TokenErr;

pub trait Primitive<Token> {
    fn parse(code: &str) -> Result<Token, TokenErr>;

    //fn peek(code: &str) -> Integer;
}