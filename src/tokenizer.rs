use crate::errors::ScanErr;

pub struct Tokenizer {
    pub(crate) code: String,
    pub(crate) start: usize,
    pub(crate) current: usize
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Integer(i32),
    
    AdditionOperator,
    SubtractionOperator,
    DivisionOperator,
    MultiplicationOperator,

    EndOfFile
}

impl Tokenizer {
    fn advance(&mut self) {
        self.current += 1;
    }

    fn get_current(&self) -> Option<char> {
        self.code.chars().nth(self.current)
    }

    fn peek(&self) -> Option<char> {
        self.code.chars().nth(self.current + 1)
    }

    fn number(&mut self, current: char) -> Result<Token, ScanErr> {
        let mut parsed: String = String::from(current);

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                parsed.push_str(&c.to_string());
                self.advance();
            } else {
                break;
            }
        }
        
        if let Ok(int) = parsed.parse::<i32>() {
            return Ok(Token::Integer(int));
        }

        Err(ScanErr::UnexpectedSequence(parsed))
    }

    fn scan_token(&mut self) -> Result<Token, ScanErr> {
        let c = match self.get_current() {
            Some(c) => c,
            None => return Ok(Token::EndOfFile)
        };

        match c {
            ' ' => {self.advance(); self.scan_token()},
            '+' => Ok(Token::AdditionOperator),
            '-' => Ok(Token::SubtractionOperator),
            '/' => Ok(Token::DivisionOperator),
            '*' => Ok(Token::MultiplicationOperator),
            '0'..='9' => self.number(c),
            _ => Err(ScanErr::UnexpectedSequence(c.to_string()))
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScanErr> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            self.start = self.current;

            let result = self.scan_token()?;

            self.advance();

            if result == Token::EndOfFile {
                tokens.push(result);
                break;
            }

            tokens.push(result);
        }

        Ok(tokens)
    }
}