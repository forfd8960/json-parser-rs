use super::Json;
use crate::{errors::ParserError, token::Token};
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Json, ParserError> {
        todo!()
    }

    fn parse_object(&mut self) -> Result<Json, ParserError> {
        todo!()
    }

    fn parse_array(&mut self) -> Result<Json, ParserError> {
        todo!()
    }

    fn consume(&mut self, t: Token, msg: String) -> Result<Token, ParserError> {
        if self.check(t) {
            return Ok(self.advance());
        }

        Err(ParserError::InvalidToken(msg))
    }

    fn check(&self, t: Token) -> bool {
        if self.is_end() {
            return false;
        }

        self.tokens[self.index] == t
    }

    fn is_end(&self) -> bool {
        self.tokens[self.index] == Token::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.index += 1;
        }

        self.tokens[self.index - 1].clone()
    }
}
