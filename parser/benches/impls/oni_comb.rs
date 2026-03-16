use oni_comb_parser::prelude::*;

pub fn parse_identifier(s: &str) -> Option<String> {
    let head = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_');
    let mut parser = head.zip(tail).map(|(h, t): (char, &str)| {
        let mut result = String::with_capacity(1 + t.len());
        result.push(h);
        result.push_str(t);
        result
    });
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}

pub fn parse_integer(s: &str) -> Option<u64> {
    let mut parser =
        take_while1(|c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<u64>().unwrap());
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}
