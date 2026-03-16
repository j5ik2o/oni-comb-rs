use winnow::ascii::digit1;
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::{one_of, take_while};

type WErr = winnow::error::ErrMode<ContextError>;

pub fn parse_identifier(s: &str) -> Option<String> {
    let mut input = s;
    let head: Result<char, WErr> =
        one_of(|c: char| c.is_ascii_alphabetic() || c == '_').parse_next(&mut input);
    head.ok().map(|h| {
        let tail: Result<&str, WErr> =
            take_while(0.., |c: char| c.is_ascii_alphanumeric() || c == '_')
                .parse_next(&mut input);
        let tail = tail.expect("take_while(0..) never fails");
        let mut result = String::with_capacity(1 + tail.len());
        result.push(h);
        result.push_str(tail);
        result
    })
}

pub fn parse_integer(s: &str) -> Option<u64> {
    let mut input = s;
    let digits: Result<&str, WErr> = digit1.parse_next(&mut input);
    digits.ok().and_then(|d| d.parse::<u64>().ok())
}
