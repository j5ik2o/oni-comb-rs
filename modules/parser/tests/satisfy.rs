use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::satisfy;
use oni_comb_parser::str_input::StrInput;

#[test]
fn satisfy_matches_when_predicate_returns_true() {
    let mut parser = satisfy(|c: char| c.is_ascii_lowercase());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok('a'));
    assert_eq!(input.offset(), 1);
    assert_eq!(input.remaining(), "bc");
}

#[test]
fn satisfy_fails_when_predicate_returns_false() {
    let mut parser = satisfy(|c: char| c.is_ascii_lowercase());
    let mut input = StrInput::new("123");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

#[test]
fn satisfy_fails_on_empty_input() {
    let mut parser = satisfy(|c: char| c.is_ascii_lowercase());
    let mut input = StrInput::new("");

    let result = parser.parse_next(&mut input);

    assert!(result.is_err());
    assert_eq!(input.offset(), 0);
}

#[test]
fn satisfy_handles_multibyte_character() {
    let mut parser = satisfy(|c: char| c == '日');
    let mut input = StrInput::new("日本語");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok('日'));
    assert_eq!(input.offset(), '日'.len_utf8());
}

#[test]
fn satisfy_does_not_consume_on_multibyte_mismatch() {
    let mut parser = satisfy(|c: char| c == '本');
    let mut input = StrInput::new("日本語");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

#[test]
fn satisfy_works_with_many0() {
    let mut parser = satisfy(|c: char| c.is_ascii_digit()).many0();
    let mut input = StrInput::new("123abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(vec!['1', '2', '3']));
    assert_eq!(input.offset(), 3);
    assert_eq!(input.remaining(), "abc");
}

#[test]
fn satisfy_works_with_or() {
    let mut parser = satisfy(|c: char| c.is_ascii_lowercase())
        .or(satisfy(|c: char| c == '_'));
    let mut input = StrInput::new("_x");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok('_'));
    assert_eq!(input.offset(), 1);
}

#[test]
fn satisfy_works_with_optional() {
    let mut parser = satisfy(|c: char| c == '+').optional();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(None));
    assert_eq!(input.offset(), 0);
}

#[test]
fn satisfy_chain_parses_identifier_start() {
    let letter_or_underscore = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let alnum_or_underscore = satisfy(|c: char| c.is_ascii_alphanumeric() || c == '_');

    let mut parser = letter_or_underscore.zip(alnum_or_underscore.many0());
    let mut input = StrInput::new("foo_bar_123!");

    let result = parser.parse_next(&mut input);

    let (first, rest) = result.unwrap();
    assert_eq!(first, 'f');
    assert_eq!(rest, vec!['o', 'o', '_', 'b', 'a', 'r', '_', '1', '2', '3']);
    assert_eq!(input.offset(), 11);
    assert_eq!(input.remaining(), "!");
}

#[test]
fn satisfy_rejects_digit_as_identifier_start() {
    let mut parser = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let mut input = StrInput::new("123abc");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}
