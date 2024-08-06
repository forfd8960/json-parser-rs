pub mod parser;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Json {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
    Null,
}
