pub mod parser;

use std::collections::HashMap;

pub enum Json {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
    Null,
}
