use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

#[test]
fn many1_collects_one_element() {
    let mut parser = char('a').many1();
    let mut input = StrInput::new("ab");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a']);
    assert_eq!(input.offset(), 1);
}

#[test]
fn many1_collects_multiple_elements() {
    let mut parser = char('a').many1();
    let mut input = StrInput::new("aaab");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a', 'a', 'a']);
    assert_eq!(input.offset(), 3);
}

#[test]
fn many1_fails_on_zero_elements() {
    let mut parser = char('a').many1();
    let mut input = StrInput::new("bbb");

    assert!(matches!(
        parser.parse_next(&mut input),
        Err(Fail::Backtrack(_))
    ));
    assert_eq!(input.offset(), 0);
}

#[test]
fn many1_propagates_cut() {
    let mut parser = char('a').cut().many1();
    let mut input = StrInput::new("bbb");

    assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn many1_with_tag() {
    let mut parser = tag("ab").many1();
    let mut input = StrInput::new("ababc");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!["ab", "ab"]);
    assert_eq!(input.offset(), 4);
}
