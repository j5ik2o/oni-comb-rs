use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::str_input::StrInput;
use oni_comb_parser::text::char::Char;
use oni_comb_parser::text::tag::Tag;

#[test]
fn map_transforms_success_value() {
    let mut parser = Char('a').map(|c: char| c.to_ascii_uppercase());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok('A'));
    assert_eq!(input.offset(), 1);
}

#[test]
fn map_preserves_failure() {
    let mut parser = Char('a').map(|c: char| c.to_ascii_uppercase());
    let mut input = StrInput::new("xyz");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

#[test]
fn map_preserves_cut_failure() {
    let mut parser = Char('x').cut().map(|c: char| c.to_ascii_uppercase());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn map_chains_multiple_transforms() {
    let mut parser = Char('a')
        .map(|c: char| c.to_ascii_uppercase())
        .map(|c: char| c.to_string());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(String::from("A")));
}

#[test]
fn then_sequences_two_parsers() {
    let mut parser = Char('a').then(Char('b'));
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(('a', 'b')));
    assert_eq!(input.offset(), 2);
    assert_eq!(input.remaining(), "c");
}

#[test]
fn then_fails_if_first_fails() {
    let mut parser = Char('a').then(Char('b'));
    let mut input = StrInput::new("xyz");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

#[test]
fn then_fails_if_second_fails() {
    let mut parser = Char('a').then(Char('b'));
    let mut input = StrInput::new("acd");

    let result = parser.parse_next(&mut input);

    assert!(result.is_err());
}

#[test]
fn then_chains_three_parsers() {
    let mut parser = Char('a').then(Char('b')).then(Char('c'));
    let mut input = StrInput::new("abcdef");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok((('a', 'b'), 'c')));
    assert_eq!(input.offset(), 3);
}

#[test]
fn then_with_tags_sequences_string_slices() {
    let mut parser = Tag("hello").then(Tag(" ")).then(Tag("world"));
    let mut input = StrInput::new("hello world!");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok((("hello", " "), "world")));
    assert_eq!(input.offset(), 11);
}

#[test]
fn then_propagates_cut_from_second() {
    let mut parser = Char('a').then(Char('b').cut());
    let mut input = StrInput::new("ac");

    let result = parser.parse_next(&mut input);

    assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn map_over_then_result() {
    let mut parser = Char('a')
        .then(Char('b'))
        .map(|(a, b): (char, char)| format!("{}{}", a, b));
    let mut input = StrInput::new("ab");

    let result = parser.parse_next(&mut input);

    assert_eq!(result, Ok(String::from("ab")));
}
