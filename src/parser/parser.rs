use std::collections::HashMap;

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
        let mut result = Json::Null;
        while !self.is_end() {
            let current = self.current();

            match current {
                Token::ObjectStart => {
                    println!("object start...");
                    result = self.parse_object()?;
                }
                Token::ArrayStart => {
                    println!("array start...");
                    result = self.parse_array()?;
                }
                _ => {
                    return Err(ParserError::InvalidToken(format!(
                        "invalid token: {:?}",
                        self.tokens[self.index]
                    )));
                }
            }
        }

        Ok(result)
    }

    fn parse_object(&mut self) -> Result<Json, ParserError> {
        self.consume(Token::ObjectStart, "expected object start: {".to_string())?;

        let mut object: HashMap<String, Json> = HashMap::new();
        while !self.check(Token::ObjectEnd) {
            let key = self.consume(Token::String("".to_string()), "expected string".to_string())?;

            self.consume(Token::Colon, "expected colon".to_string())?;

            let value = self.parse_value()?;

            if let Token::String(key_val) = key {
                object.insert(key_val, value);
            } else {
                return Err(ParserError::InvalidToken(format!(
                    "expected string, got {:?}",
                    key
                )));
            }

            if self.check(Token::ObjectEnd) {
                break;
            }
            self.consume(Token::Comma, "expected comma".to_string())?;
        }

        self.consume(Token::ObjectEnd, "expected object end:}".to_string())?;
        Ok(Json::Object(object))
    }

    fn parse_array(&mut self) -> Result<Json, ParserError> {
        self.consume(Token::ArrayStart, "expected array start: [".to_string())?;

        let mut array = Vec::new();
        while !self.check(Token::ArrayEnd) {
            let value = self.parse_value()?;
            array.push(value);

            if self.check(Token::ArrayEnd) {
                break;
            }

            self.consume(Token::Comma, "expected comma".to_string())?;
        }

        self.consume(Token::ArrayEnd, "expected array end: ]".to_string())?;
        Ok(Json::Array(array))
    }

    fn parse_value(&mut self) -> Result<Json, ParserError> {
        let current = self.current();
        println!("current: {:?}", current);
        match current {
            Token::String(s) => {
                self.advance();
                Ok(Json::String(s))
            }
            Token::Number(n) => {
                self.advance();
                Ok(Json::Number(n))
            }
            Token::Boolean(b) => {
                self.advance();
                Ok(Json::Boolean(b))
            }
            Token::Null => {
                self.advance();
                Ok(Json::Null)
            }
            Token::ObjectStart => self.parse_object(),
            Token::ArrayStart => self.parse_array(),
            _ => Err(ParserError::InvalidToken(format!(
                "invalid token: {:?}",
                self.tokens[self.index]
            ))),
        }
    }

    fn consume(&mut self, t: Token, msg: String) -> Result<Token, ParserError> {
        if self.check(t) {
            let t = self.advance().unwrap();
            return Ok(t);
        }

        Err(ParserError::InvalidToken(msg))
    }

    fn check(&self, t: Token) -> bool {
        if self.is_end() {
            return false;
        }

        let current = self.tokens[self.index].clone();
        match current {
            Token::Number(_) => match t {
                Token::Number(_) => true,
                _ => false,
            },
            Token::String(_) => match t {
                Token::String(_) => true,
                _ => false,
            },
            Token::Boolean(_) => match t {
                Token::Boolean(_) => true,
                _ => false,
            },
            _ => t == current,
        }
    }

    fn is_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn current(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_end() {
            return None;
        }

        self.index += 1;
        Some(self.tokens[self.index - 1].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_array() -> Result<()> {
        let tokens = vec![Token::ArrayStart, Token::Number(1.0), Token::ArrayEnd];
        let mut parser = Parser::new(tokens);
        let obj = parser.parse()?;
        assert_eq!(obj, Json::Array(vec![Json::Number(1.0)]));
        Ok(())
    }

    #[test]
    fn test_parse_object() -> Result<()> {
        let tokens = vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Number(1.0),
            Token::ObjectEnd,
        ];
        let mut parser = Parser::new(tokens);
        let obj = parser.parse()?;

        assert_eq!(
            obj,
            Json::Object(HashMap::from([("key".to_string(), Json::Number(1.0))]))
        );

        Ok(())
    }

    #[test]
    fn test_parse_object1() -> Result<()> {
        let tokens = vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::ArrayStart,
            Token::Number(1.0),
            Token::Comma,
            Token::String("data".to_string()),
            Token::Comma,
            Token::Boolean(true),
            Token::ArrayEnd,
            Token::ObjectEnd,
        ];
        let mut parser = Parser::new(tokens);
        let obj = parser.parse()?;

        assert_eq!(
            obj,
            Json::Object(HashMap::from([(
                "key".to_string(),
                Json::Array(vec![
                    Json::Number(1.0),
                    Json::String("data".to_string()),
                    Json::Boolean(true)
                ])
            )]))
        );

        Ok(())
    }

    #[test]
    fn test_parse_object2() -> Result<()> {
        let tokens = vec![
            Token::ObjectStart,
            Token::String("key".to_string()),
            Token::Colon,
            Token::ObjectStart,
            Token::String("k1".to_string()),
            Token::Colon,
            Token::String("v1".to_string()),
            Token::Comma,
            Token::String("k2".to_string()),
            Token::Colon,
            Token::Boolean(true),
            Token::ObjectEnd,
            Token::ObjectEnd,
        ];
        let mut parser = Parser::new(tokens);
        let obj = parser.parse()?;

        assert_eq!(
            obj,
            Json::Object(HashMap::from([(
                "key".to_string(),
                Json::Object(HashMap::from([
                    ("k1".to_string(), Json::String("v1".to_string())),
                    ("k2".to_string(), Json::Boolean(true)),
                ]))
            )]))
        );

        Ok(())
    }
}
