use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::str_input::StrInput;
use oni_comb_parser::text::satisfy::Satisfy;
use oni_comb_parser::text::take_while::{TakeWhile0, TakeWhile1};

pub fn parse_identifier(s: &str) -> Option<String> {
    let head = Satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = TakeWhile0(|c: char| c.is_ascii_alphanumeric() || c == '_');
    let mut parser = head.then(tail).map(|(h, t): (char, &str)| {
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
        TakeWhile1(|c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<u64>().unwrap());
    let mut input = StrInput::new(s);
    parser.parse_next(&mut input).ok()
}
