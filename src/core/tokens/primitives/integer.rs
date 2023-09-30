use crate::core::{tokens::misc::whitespace::WhiteSpace, errors::token_err::TokenErr};

use super::primitive::Primitive;

#[derive(Debug)]
pub struct Integer {
    _value: i32
}

impl Primitive<Integer> for Integer {
    // fn parse(token) -> bool {
    //     return token.is_digit(10);
    // }

    fn parse(code: &str) -> Result<Integer, TokenErr> {
        let mut parsed_int = String::new();

        for c in code.chars() {
            if WhiteSpace::parse(&c) {
                break;
            }

            if c.is_ascii_digit() {
                parsed_int.push(c);
                continue;
            } else {
                return Err(TokenErr::UnexpectedSequence(String::from(c)));
            }
        }

        if let Ok(int) = parsed_int.parse::<i32>() {
            Ok(Integer { _value: int })
        } else {
            Err(TokenErr::UnexpectedSequence(String::from(code)))
        }
    }
}