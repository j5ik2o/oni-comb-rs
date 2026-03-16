use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::{take_while0, take_while1};
use oni_comb_parser::str_input::StrInput;

#[test]
fn take_while0_consumes_matching_characters() {
    let mut parser = take_while0(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("123abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("123"));
    assert_eq!(input.offset(), 3);
    assert_eq!(input.remaining(), "abc");
}

#[test]
fn take_while0_returns_empty_str_on_no_match() {
    let mut parser = take_while0(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(""));
    assert_eq!(input.offset(), 0);
}

#[test]
fn take_while0_returns_empty_str_on_empty_input() {
    let mut parser = take_while0(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(""));
    assert_eq!(input.offset(), 0);
}

#[test]
fn take_while0_consumes_entire_input_when_all_match() {
    let mut parser = take_while0(|c: char| c.is_ascii_alphabetic());
    let mut input = StrInput::new("abcdef");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("abcdef"));
    assert!(input.is_eof());
}

#[test]
fn take_while0_handles_multibyte_characters() {
    let mut parser = take_while0(|c: char| !c.is_ascii());
    let mut input = StrInput::new("日本語abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("日本語"));
    assert_eq!(input.offset(), "日本語".len());
    assert_eq!(input.remaining(), "abc");
}

#[test]
fn take_while1_consumes_matching_characters() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("9999abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("9999"));
    assert_eq!(input.offset(), 4);
    assert_eq!(input.remaining(), "abc");
}

#[test]
fn take_while1_fails_on_no_match() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

#[test]
fn take_while1_fails_on_empty_input() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("");

    let result = parser.parse_next(&mut input);

    assert!(result.is_err());
    assert_eq!(input.offset(), 0);
}

#[test]
fn take_while1_consumes_entire_input_when_all_match() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("9999999");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("9999999"));
    assert!(input.is_eof());
}

#[test]
fn take_while1_succeeds_with_single_match() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("5abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("5"));
    assert_eq!(input.offset(), 1);
}

#[test]
fn take_while1_handles_multibyte_characters() {
    let mut parser = take_while1(|c: char| !c.is_ascii());
    let mut input = StrInput::new("日本語abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok("日本語"));
    assert_eq!(input.offset(), "日本語".len());
    assert_eq!(input.remaining(), "abc");
}

#[test]
fn take_while1_works_with_or_for_identifier() {
    let head = take_while1(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_');

    let mut parser = head.then(tail);
    let mut input = StrInput::new("foo_bar_123 rest");

    let result = parser.parse_next(&mut input);

    let (h, t) = result.unwrap();
    assert_eq!(h, "foo_bar_");
    assert_eq!(t, "123");
    assert_eq!(input.offset(), 11);
}

#[test]
fn take_while1_with_map_parses_integer() {
    let mut parser =
        take_while1(|c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<u64>().unwrap());
    let mut input = StrInput::new("9999999 rest");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(9999999u64));
    assert_eq!(input.offset(), 7);
}

#[test]
fn take_while0_works_with_optional() {
    let mut parser = take_while0(|c: char| c.is_ascii_digit()).optional();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(Some("")));
}

#[test]
fn take_while1_works_with_optional() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit()).optional();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(None));
    assert_eq!(input.offset(), 0);
}

#[test]
fn take_while_identifier_rejects_digit_start() {
    let mut parser = take_while1(|c: char| c.is_ascii_alphabetic() || c == '_');
    let mut input = StrInput::new("123abc");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

#[test]
fn take_while_integer_rejects_non_digit() {
    let mut parser = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("abc123");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}
