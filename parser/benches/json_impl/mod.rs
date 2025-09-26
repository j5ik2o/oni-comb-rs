pub mod nom;
pub mod oni_comb;
pub mod pom;

use serde_json::Value;

pub fn read_fixture(name: &str) -> &'static [u8] {
    match name {
        "heavy" => include_bytes!("../data/heavy.json"),
        _ => panic!("unknown fixture {name}"),
    }
}

pub fn read_fail_fixture(name: &str) -> &'static [u8] {
    match name {
        "missing_comma" => include_bytes!("../data/fail/missing_comma.json"),
        "unclosed_brace" => include_bytes!("../data/fail/unclosed_brace.json"),
        _ => panic!("unknown fail fixture {name}"),
    }
}

pub fn parse_with_serde(input: &[u8]) -> Result<Value, String> {
    serde_json::from_slice(input).map_err(|e| e.to_string())
}
