# json-parser-rs

## Example

```rust
use json_parser_rs::parse_json;

fn main() {
    let my_json = r#"{"name":"Alex", "age": 30, "job":"Software Engineer"}"#;
    let json = parse_json(my_json).unwrap();
    // parsed json: Object({"name": String("Alex"), "job": String("Software Engineer"), "age": Number(30.0)})
    println!("parsed json: {:?}", json);

    let my_json1 = r#"{"list":[1,3,6,9,100]}"#;
    let json = parse_json(my_json1).unwrap();
    // parsed json: Object({"list": Array([Number(1.0), Number(3.0), Number(6.0), Number(9.0), Number(100.0)])})
    println!("parsed json: {:?}", json);

    let my_json2 = r#"[1,3,6,9,100]"#;
    let json = parse_json(my_json2).unwrap();
    // parsed json: Array([Number(1.0), Number(3.0), Number(6.0), Number(9.0), Number(100.0)])
    println!("parsed json: {:?}", json);

    let idea_list = r#"{"ideas":[{"id":1,"title":"idea1","content":"content1","created_at":"2023-07-01T00:00:00Z","updated_at":"2023-07-01T00:00:00Z"},{"id":2,"title":"idea2","content":"content2","created_at":"2023-07-01T00:00:00Z","updated_at":"2023-07-01T00:00:00Z"}]}"#;
    let idea_list_json = parse_json(idea_list).unwrap();

    // parsed json: Object({"ideas": Array([Object({"created_at": String("2023-07-01T00:00:00Z"), "content": String("content1"), "title": String("idea1"), "updated_at": String("2023-07-01T00:00:00Z"), "id": Number(1.0)}), Object({"updated_at": String("2023-07-01T00:00:00Z"), "title": String("idea2"), "id": Number(2.0), "content": String("content2"), "created_at": String("2023-07-01T00:00:00Z")})])})
    println!("parsed json: {:?}", idea_list_json);
}

```
