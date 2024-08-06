#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Boolean(bool),
    ArrayStart,  // [
    ArrayEnd,    // ]
    ObjectStart, // {
    ObjectEnd,   // }
    Null,
    NewLine,
    Comma, //,
    Colon, // :
}
