pub mod parser;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
