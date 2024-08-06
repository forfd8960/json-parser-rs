use errors::ParserError;
use parser::parser::Parser;
use parser::Json;

mod errors;
mod parser;
mod scanner;
mod token;

pub fn parse_json(json: &str) -> Result<Json, ParserError> {
    match scanner::Scanner::new(json.to_string()).scan() {
        Ok(tokens) => Parser::new(tokens).parse(),
        Err(e) => Err(ParserError::InvalidJson(e.to_string())),
    }
}
