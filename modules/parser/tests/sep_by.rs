use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// ── sep_by0 ───────────────────────────────────

#[test]
fn sep_by0_empty_input() {
    let mut parser = char('a').sep_by0(char(','));
    let mut input = StrInput::new("");

    assert_eq!(parser.parse_next(&mut input).unwrap(), Vec::<char>::new());
}

#[test]
fn sep_by0_no_match() {
    let mut parser = char('a').sep_by0(char(','));
    let mut input = StrInput::new("xyz");

    assert_eq!(parser.parse_next(&mut input).unwrap(), Vec::<char>::new());
    assert_eq!(input.offset(), 0);
}

#[test]
fn sep_by0_single_element() {
    let mut parser = char('a').sep_by0(char(','));
    let mut input = StrInput::new("a");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a']);
    assert_eq!(input.offset(), 1);
}

#[test]
fn sep_by0_multiple_elements() {
    let mut parser = char('a').sep_by0(char(','));
    let mut input = StrInput::new("a,a,a");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a', 'a', 'a']);
    assert_eq!(input.offset(), 5);
}

#[test]
fn sep_by0_rejects_trailing_separator() {
    let mut parser = char('a').sep_by0(char(','));
    let mut input = StrInput::new("a,a,");

    // trailing comma の後の要素が見つからない → カンマ前に巻き戻し
    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a', 'a']);
    assert_eq!(input.offset(), 3);
}

#[test]
fn sep_by0_with_tag() {
    let mut parser = tag("ab").sep_by0(tag(", "));
    let mut input = StrInput::new("ab, ab, ab!");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!["ab", "ab", "ab"]);
    assert_eq!(input.offset(), 10);
}

#[test]
fn sep_by0_propagates_cut() {
    let mut parser = char('a').cut().sep_by0(char(','));
    let mut input = StrInput::new("xyz");

    assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

// ── sep_by1 ───────────────────────────────────

#[test]
fn sep_by1_fails_on_empty() {
    let mut parser = char('a').sep_by1(char(','));
    let mut input = StrInput::new("");

    assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
}

#[test]
fn sep_by1_single_element() {
    let mut parser = char('a').sep_by1(char(','));
    let mut input = StrInput::new("a");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a']);
}

#[test]
fn sep_by1_multiple_elements() {
    let mut parser = char('a').sep_by1(char(','));
    let mut input = StrInput::new("a,a,a");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a', 'a', 'a']);
}

#[test]
fn sep_by1_rejects_trailing_separator() {
    let mut parser = char('a').sep_by1(char(','));
    let mut input = StrInput::new("a,a,");

    assert_eq!(parser.parse_next(&mut input).unwrap(), vec!['a', 'a']);
    assert_eq!(input.offset(), 3);
}
