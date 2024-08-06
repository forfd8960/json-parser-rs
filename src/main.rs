use json_parser_rs::parse_json;

fn main() {
    let my_json = r#"{"name":"Alex", "age": 30, "job":"Software Engineer"}"#;
    let json = parse_json(my_json).unwrap();
    println!("parsed json: {:?}", json);
}
