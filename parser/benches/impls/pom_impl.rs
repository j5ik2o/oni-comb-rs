use pom::parser::*;

pub fn parse_identifier(s: &str) -> Option<String> {
    let input: Vec<char> = s.chars().collect();
    let head = is_a(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = is_a(|c: char| c.is_ascii_alphanumeric() || c == '_').repeat(0..);
    let ident = (head + tail).map(|(h, t)| {
        let mut result = String::with_capacity(1 + t.len());
        result.push(h);
        for c in t {
            result.push(c);
        }
        result
    });
    ident.parse(&input).ok()
}

pub fn parse_integer(s: &str) -> Option<u64> {
    let input: Vec<char> = s.chars().collect();
    let digits = is_a(|c: char| c.is_ascii_digit()).repeat(1..);
    let parser = digits.map(|d| d.iter().collect::<String>().parse::<u64>().unwrap());
    parser.parse(&input).ok()
}
