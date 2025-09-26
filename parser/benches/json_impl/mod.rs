pub mod nom;
pub mod oni_comb;
pub mod pom;

pub fn read_fixture(name: &str) -> &'static [u8] {
    match name {
        "heavy" => include_bytes!("../data/heavy.json"),
        _ => panic!("unknown fixture {name}"),
    }
}

#[allow(dead_code)]
pub fn read_fail_fixture(name: &str) -> &'static [u8] {
    match name {
        "missing_comma" => include_bytes!("../data/fail/missing_comma.json"),
        "unclosed_brace" => include_bytes!("../data/fail/unclosed_brace.json"),
        _ => panic!("unknown fail fixture {name}"),
    }
}
