use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[test]
fn empty_string() {
  let mut parser = quoted_string();
  let mut input = StrInput::new("\"\"");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "");
  assert_eq!(input.offset(), 2);
}

#[test]
fn simple_string() {
  let mut parser = quoted_string();
  let mut input = StrInput::new("\"hello\"");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
}

#[test]
fn string_with_remaining() {
  let mut parser = quoted_string();
  let mut input = StrInput::new("\"hello\" world");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.offset(), 7);
}

// ── escape sequences ─────────────────────────

#[test]
fn escape_quote() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""say \"hi\"""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "say \"hi\"");
}

#[test]
fn escape_backslash() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""a\\b""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\\b");
}

#[test]
fn escape_slash() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""a\/b""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a/b");
}

#[test]
fn escape_newline_tab() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""a\n\tb""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\n\tb");
}

#[test]
fn escape_carriage_return() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""a\rb""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\rb");
}

#[test]
fn escape_backspace_formfeed() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""a\b\fb""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\u{0008}\u{000C}b");
}

#[test]
fn escape_unicode() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""\u0041""#); // U+0041 = 'A'

  assert_eq!(parser.parse_next(&mut input).unwrap(), "A");
}

#[test]
fn escape_unicode_japanese() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""\u3042""#); // U+3042 = 'あ'

  assert_eq!(parser.parse_next(&mut input).unwrap(), "あ");
}

#[test]
fn escape_unicode_mixed() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""hello\u0020world""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello world");
}

// ── error cases ──────────────────────────────

#[test]
fn not_a_string() {
  let mut parser = quoted_string();
  let mut input = StrInput::new("hello");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn unterminated_string_is_cut() {
  let mut parser = quoted_string();
  let mut input = StrInput::new("\"hello");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn invalid_escape_is_cut() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""\x""#);

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn incomplete_unicode_escape_is_cut() {
  let mut parser = quoted_string();
  let mut input = StrInput::new(r#""\u00""#);

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}
