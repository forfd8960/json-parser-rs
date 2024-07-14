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
}
