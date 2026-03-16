use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// 4.1: flat_map succeeds when both parsers succeed
#[test]
fn flat_map_succeeds_when_both_succeed() {
    // Parse a digit, then dynamically choose a tag based on the digit
    let mut parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|c| match c {
        '1' => tag("one"),
        '2' => tag("two"),
        _ => tag("other"),
    });
    let mut input = StrInput::new("1one");
    assert_eq!(parser.parse_next(&mut input).unwrap(), "one");
    assert_eq!(input.offset(), 4);
}

// 4.2: flat_map fails when the first parser fails (Backtrack)
#[test]
fn flat_map_fails_when_first_backtracks() {
    let mut parser = char('x').flat_map(|_| tag("rest"));
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);
    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

// 4.3: flat_map propagates Cut from the first parser
#[test]
fn flat_map_propagates_cut_from_first() {
    let mut parser = char('x').cut().flat_map(|_| tag("rest"));
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);
    assert!(matches!(result, Err(Fail::Cut(_))));
}

// 4.4: flat_map propagates failure from the dynamically chosen parser
#[test]
fn flat_map_propagates_failure_from_second() {
    let mut parser = char('a').flat_map(|_| tag("xyz"));
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);
    assert!(matches!(result, Err(Fail::Backtrack(_))));
    // first parser consumed 'a', second failed
    assert_eq!(input.offset(), 1);
}

#[test]
fn flat_map_propagates_cut_from_second() {
    let mut parser = char('a').flat_map(|_| tag("xyz").cut());
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);
    assert!(matches!(result, Err(Fail::Cut(_))));
}

// 4.5: flat_map with same-type branches (no Box needed)
#[test]
fn flat_map_same_type_branches_no_box() {
    let mut parser = satisfy(|c: char| c.is_ascii_digit()).flat_map(|c| match c {
        '1' => tag("one"),
        '2' => tag("two"),
        _ => tag("?"),
    });

    let mut input = StrInput::new("2two");
    assert_eq!(parser.parse_next(&mut input).unwrap(), "two");
    assert_eq!(input.offset(), 4);
}

// 4.6: flat_map with Box<dyn Parser> for heterogeneous branches
#[test]
fn flat_map_box_dyn_heterogeneous_branches() {
    let mut parser = satisfy(|c: char| c == 'c' || c == 't').flat_map(
        |c| -> Box<
            dyn Parser<oni_comb_parser::str_input::StrInput<'_>, Output = &str, Error = ParseError>,
        > {
            match c {
                'c' => Box::new(tag("har")),
                _ => Box::new(take_while1(|c: char| c.is_ascii_digit())),
            }
        },
    );

    let mut input1 = StrInput::new("char");
    assert_eq!(parser.parse_next(&mut input1).unwrap(), "har");

    let mut input2 = StrInput::new("t123");
    assert_eq!(parser.parse_next(&mut input2).unwrap(), "123");
}

// 4.7: flat_map inside attempt downgrades Cut to Backtrack
#[test]
fn flat_map_inside_attempt_downgrades_cut() {
    let inner = char('a').flat_map(|_| tag("xyz").cut());
    let mut parser = inner.attempt();
    let mut input = StrInput::new("abc");

    let result = parser.parse_next(&mut input);
    assert!(matches!(result, Err(Fail::Backtrack(_))));
    assert_eq!(input.offset(), 0);
}

// 4.8: flat_map result can be mapped
#[test]
fn flat_map_result_can_be_mapped() {
    let mut parser = char('a')
        .flat_map(|_| tag("bc"))
        .map(|s: &str| s.to_uppercase());
    let mut input = StrInput::new("abc");

    assert_eq!(parser.parse_next(&mut input).unwrap(), "BC");
}

// 4.9: flat_map can be used inside or
#[test]
fn flat_map_inside_or() {
    let left = char('x').flat_map(|_| tag("yy"));
    let right = tag("abc");
    let mut parser = left.or(right);
    let mut input = StrInput::new("abc");

    assert_eq!(parser.parse_next(&mut input).unwrap(), "abc");
    assert_eq!(input.offset(), 3);
}
