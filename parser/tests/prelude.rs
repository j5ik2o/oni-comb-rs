use oni_comb_parser::prelude::*;

#[test]
fn char_or_with_prelude() {
    let mut p = char('a').or(char('b'));
    let mut input = StrInput::new("b");
    assert_eq!(p.parse_next(&mut input).unwrap(), 'b');
}

#[test]
fn tag_then_with_prelude() {
    let mut p = tag("hello").zip(tag(" world"));
    let mut input = StrInput::new("hello world");
    assert_eq!(p.parse_next(&mut input).unwrap(), ("hello", " world"));
}

#[test]
fn satisfy_and_take_while_with_prelude() {
    let head = satisfy(|c: char| c.is_ascii_alphabetic() || c == '_');
    let tail = take_while0(|c: char| c.is_ascii_alphanumeric() || c == '_');
    let mut p = head.zip(tail);
    let mut input = StrInput::new("foo_123");
    let (h, t) = p.parse_next(&mut input).unwrap();
    assert_eq!(h, 'f');
    assert_eq!(t, "oo_123");
}

#[test]
fn eof_with_prelude() {
    let mut p = tag("done").zip(eof());
    let mut input = StrInput::new("done");
    assert!(p.parse_next(&mut input).is_ok());
}

#[test]
fn take_while1_with_prelude() {
    let mut p = take_while1(|c: char| c.is_ascii_digit());
    let mut input = StrInput::new("42abc");
    assert_eq!(p.parse_next(&mut input).unwrap(), "42");
}
