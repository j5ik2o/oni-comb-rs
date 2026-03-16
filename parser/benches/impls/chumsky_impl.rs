use chumsky::prelude::*;

pub fn parse_identifier(s: &str) -> Option<String> {
    let head = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_alphabetic() || *c == '_');
    let tail =
        filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated();
    let ident = head.then(tail).map(|(h, t): (char, Vec<char>)| {
        let mut result = String::with_capacity(1 + t.len());
        result.push(h);
        for c in t {
            result.push(c);
        }
        result
    });
    ident.parse(s).ok()
}

pub fn parse_integer(s: &str) -> Option<u64> {
    let digits = filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .collect::<String>();
    let parser = digits.map(|d| d.parse::<u64>().unwrap());
    parser.parse(s).ok()
}
