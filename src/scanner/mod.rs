use anyhow::{bail, Result};

use crate::{errors::LexerError, token::Token};

pub struct Scanner {
    pub chars: Vec<char>,
    pub start: usize,
    pub current: usize,
}

impl Scanner {
    pub fn new(json_text: String) -> Self {
        Self {
            chars: json_text.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            println!(
                "start: {}, current: {}, scanned: {:?}",
                self.start, self.current, token
            );

            if token == Token::NewLine {
                continue;
            }

            tokens.push(token);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let cur = self.advance();

        // let mut tokens: Vec<Token> = vec![];
        match cur {
            '{' => Ok(Token::ObjectStart),
            '}' => Ok(Token::ObjectEnd),
            '[' => Ok(Token::ArrayStart),
            ']' => Ok(Token::ArrayEnd),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '"' => Ok(self.scan_string()?),
            '\n' | '\t' | '\r' | ' ' => Ok(Token::NewLine),
            _ => {
                if cur.is_numeric() {
                    return Ok(self.scan_number()?);
                } else if cur.is_alphabetic() {
                    return Ok(self.scan_identifier()?);
                } else {
                    return Err(LexerError::InvalidChar);
                }
            }
        }
    }

    fn scan_string(&mut self) -> Result<Token, LexerError> {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c != '"' && !self.is_at_end() {
                self.advance();
            } else {
                break;
            }
        }

        if self.is_at_end() {
            return Err(LexerError::InvalidString("unterminated string".to_string()));
        }

        self.advance();
        for c in self.chars[self.start + 1..self.current - 1].iter() {
            s.push(*c);
        }
        Ok(Token::String(s))
    }

    fn scan_number(&mut self) -> Result<Token, LexerError> {
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if c == '.' {
                if let Some(c_n) = self.peek_next() {
                    if c_n.is_numeric() {
                        self.advance();

                        while let Some(cc) = self.peek() {
                            if cc.is_numeric() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        let s: String = self.chars[self.start..self.current].iter().collect();
        match s.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(LexerError::InvalidNumber(s)),
        }
    }

    fn scan_identifier(&mut self) -> Result<Token, LexerError> {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.chars[self.start..self.current].iter().collect();
        match text.as_str() {
            "null" => Ok(Token::Null),
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            _ => Err(LexerError::InvalidIdent(text)),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        Some(self.chars[self.current])
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.chars.len() {
            return None;
        }
        Some(self.chars[self.current + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_string() -> anyhow::Result<()> {
        let mut scanner = Scanner::new(r#"{"key":"value"}"#.to_string());
        let tokens = scanner.scan()?;
        assert_eq!(
            tokens,
            vec![
                Token::ObjectStart,
                Token::String("key".to_string()),
                Token::Colon,
                Token::String("value".to_string()),
                Token::ObjectEnd,
                Token::Eof,
            ]
        );
        Ok(())
    }

    #[test]
    fn test_scan_number() -> anyhow::Result<()> {
        let mut scanner = Scanner::new(r#"{"weight":100}"#.to_string());
        let tokens = scanner.scan()?;
        assert_eq!(
            tokens,
            vec![
                Token::ObjectStart,
                Token::String("weight".to_string()),
                Token::Colon,
                Token::Number(100 as f64),
                Token::ObjectEnd,
                Token::Eof,
            ]
        );
        Ok(())
    }

    #[test]
    fn test_scan_number1() -> anyhow::Result<()> {
        let mut scanner = Scanner::new(r#"[10.28]"#.to_string());
        let tokens = scanner.scan()?;
        assert_eq!(
            tokens,
            vec![
                Token::ArrayStart,
                Token::Number(10.28 as f64),
                Token::Colon,
                Token::ArrayEnd,
                Token::Eof,
            ]
        );
        Ok(())
    }

    #[test]
    fn test_scan_boolean() -> anyhow::Result<()> {
        let mut scanner = Scanner::new(r#"{"has_more":true, "contain":false}"#.to_string());
        let tokens = scanner.scan()?;
        assert_eq!(
            tokens,
            vec![
                Token::ObjectStart,
                Token::String("has_more".to_string()),
                Token::Colon,
                Token::Boolean(true),
                Token::Comma,
                Token::String("contain".to_string()),
                Token::Colon,
                Token::Boolean(false),
                Token::ObjectEnd,
                Token::Eof,
            ]
        );
        Ok(())
    }
    #[test]
    fn test_scan_array() -> anyhow::Result<()> {
        let mut scanner =
            Scanner::new(r#"["abc", true, false, 10, 10.24, {"key":"value"}]"#.to_string());
        let tokens = scanner.scan()?;
        assert_eq!(
            tokens,
            vec![
                Token::ArrayStart,
                Token::String("abc".to_string()),
                Token::Comma,
                Token::Boolean(true),
                Token::Comma,
                Token::Boolean(false),
                Token::Comma,
                Token::Number(10 as f64),
                Token::Comma,
                Token::Number(10.24 as f64),
                Token::Comma,
                Token::ObjectStart,
                Token::String("key".to_string()),
                Token::Colon,
                Token::String("value".to_string()),
                Token::ObjectEnd,
                Token::ArrayEnd,
                Token::Eof,
            ]
        );
        Ok(())
    }
}
