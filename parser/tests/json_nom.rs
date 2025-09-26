#[path = "../benches/json_impl/mod.rs"]
mod json_impl;

#[test]
fn nom_parses_heavy_fixture() {
    let data = json_impl::read_fixture("heavy");
    let value = json_impl::nom::parse_json_value(data).expect("nom parser failed");
    assert!(value.is_object());
}

#[test]
fn nom_parses_string_value() {
    let value = json_impl::nom::parse_json_value(br#""test""#).expect("string parse failed");
    assert_eq!(value, serde_json::Value::String("test".into()));
}

#[test]
fn nom_parses_simple_object() {
    let value = json_impl::nom::parse_json_value(br#"{"key": 1}"#).expect("object parse failed");
    assert!(value.is_object());
}

#[test]
fn nom_parses_number() {
    let value = json_impl::nom::parse_json_value(br"123").expect("number parse failed");
    assert!(value.is_number());
}
