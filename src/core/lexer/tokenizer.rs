use crate::core::errors::token_err::TokenErr;
use crate::core::tokens::misc::operator::Operator;
use crate::core::tokens::primitives::primitive::Primitive;
use crate::core::tokens::primitives::integer::Integer;

pub struct Lexer {
    pub(crate) code: String
}

#[derive(Debug)]
pub enum Token {
    Integer(Integer),
    Operator(Operator)
}

impl Primitive<Token> for Token {
    fn parse(code: &str) -> Result<Token, crate::core::errors::token_err::TokenErr> {
        if let Ok(i) = Integer::parse(code) {
            return Ok(Token::Integer(i));
        }

        if let Ok(o) = Operator::parse(code) {
            return Ok(Token::Operator(o));
        }

        Err(TokenErr::UnexpectedSequence(String::from(code)))
    }
}

impl Lexer {
    pub fn tokenize(&self) -> Result<Vec<Token>, TokenErr> {
        let mut lexemes: Vec<Token> = vec![];

        let tokens_str = self.code.split(' ').collect::<Vec<_>>();

        for token in tokens_str {
            //Tomyks Weg
            lexemes.push(Token::parse(token)?);
            //Wonders Weg
            // let t = Token::parse(token);

            // match t {
            //     Ok(t) => lexemes.push(t),
            //     Err(err) => return Err(err)
            // }
        }

        Ok(lexemes)
    }
}