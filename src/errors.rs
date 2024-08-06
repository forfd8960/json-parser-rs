use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ParserError {
    #[error("invalid number")]
    InvalidNumber,
    #[error("invalid string")]
    InvalidString,
    #[error("invalid array")]
    InvalidArray,
    #[error("invalid object")]
    InvalidObject,
    #[error("invalid token")]
    InvalidToken(String),
    #[error("invalid json")]
    InvalidJson(String),
}

#[derive(Debug, Error)]
pub(crate) enum LexerError {
    #[error("invalid number: {0}")]
    InvalidNumber(String),
    #[error("invalid string: {0}")]
    InvalidString(String),
    #[error("invalid array")]
    InvalidArray,
    #[error("invalid object")]
    InvalidObject,
    #[error("invalid char")]
    InvalidChar,
    #[error("invalid identifier: {0}")]
    InvalidIdent(String),
}
