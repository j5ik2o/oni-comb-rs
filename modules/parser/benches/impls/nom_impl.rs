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

/// flat_map 同一型分岐: digit → tag
pub fn parse_flat_map_same_type(s: &str) -> Option<&str> {
    use nom::bytes::tag as nom_tag;

    nom_satisfy::<_, &str, nom::error::Error<&str>>(|c: char| c.is_ascii_digit())
        .flat_map(|c: char| match c {
            '1' => nom_tag::<_, _, nom::error::Error<&str>>("one"),
            '2' => nom_tag("two"),
            '3' => nom_tag("three"),
            _ => nom_tag(""),
        })
        .parse_complete(s)
        .ok()
        .map(|(_, matched)| matched)
}

/// flat_map 異種型分岐: 先頭文字に応じて異なる型のパーサーを選択
/// nom::Parser は dyn 非互換のため、手動二段パースで実装
pub fn parse_flat_map_boxed(s: &str) -> Option<(&str, &str)> {
    use nom::bytes::{tag as nom_tag, take_while1 as nom_take_while1};

    let (rest, t) = nom_satisfy::<_, &str, nom::error::Error<&str>>(|c: char| {
        c == 'c' || c == 'i'
    })
    .parse_complete(s)
    .ok()?;

    let result = match t {
        'c' => pair(
            nom_tag::<_, _, nom::error::Error<&str>>(":"),
            nom_take_while1(|c: char| c.is_ascii_alphabetic()),
        )
        .parse_complete(rest),
        _ => pair(
            nom_tag::<_, _, nom::error::Error<&str>>(":"),
            nom_take_while1(|c: char| c.is_ascii_digit()),
        )
        .parse_complete(rest),
    };
    result.ok().map(|(_, v)| v)
}
