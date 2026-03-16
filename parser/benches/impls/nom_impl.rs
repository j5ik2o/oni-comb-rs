use nom::character::{digit1, satisfy as nom_satisfy};
use nom::combinator::recognize;
use nom::multi::many0_count;
use nom::sequence::pair;
use nom::Parser;

pub fn parse_identifier(s: &str) -> Option<String> {
    recognize::<_, nom::error::Error<&str>, _>(pair(
        nom_satisfy(|c: char| c.is_ascii_alphabetic() || c == '_'),
        many0_count(nom_satisfy(|c: char| c.is_ascii_alphanumeric() || c == '_')),
    ))
    .parse_complete(s)
    .ok()
    .map(|(_, matched)| matched.to_string())
}

pub fn parse_integer(s: &str) -> Option<u64> {
    digit1::<&str, nom::error::Error<&str>>()
        .parse_complete(s)
        .ok()
        .and_then(|(_, digits)| digits.parse::<u64>().ok())
}
