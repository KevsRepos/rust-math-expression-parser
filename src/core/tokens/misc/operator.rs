use crate::core::{tokens::primitives::primitive::Primitive, errors::token_err::TokenErr};

#[derive(Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division
}

impl Primitive<Operator> for Operator {
    fn parse(code: &str) -> Result<Operator, crate::core::errors::token_err::TokenErr> {
        match code {
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),
            sequence => Err(TokenErr::UnexpectedSequence(String::from(sequence)))
        }
    }
}