use std::borrow::Cow;

use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[test]
fn empty_string() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new("\"\"");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "");
  assert_eq!(input.offset(), 2);
}

#[test]
fn simple_string() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new("\"hello\"");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
}

#[test]
fn simple_string_is_borrowed() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new("\"hello\"");

  assert!(matches!(parser.parse_next(&mut input).unwrap(), Cow::Borrowed("hello")));
}

#[test]
fn string_with_remaining() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new("\"hello\" world");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.offset(), 7);
}

// ── escape sequences ─────────────────────────

#[test]
fn escape_quote() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""say \"hi\"""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "say \"hi\"");
}

#[test]
fn escaped_string_is_owned() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""say \"hi\"""#);

  assert!(matches!(
    parser.parse_next(&mut input).unwrap(),
    Cow::Owned(ref s) if s == "say \"hi\""
  ));
}

#[test]
fn escape_backslash() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""a\\b""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\\b");
}

#[test]
fn escape_slash() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""a\/b""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a/b");
}

#[test]
fn escape_newline_tab() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""a\n\tb""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\n\tb");
}

#[test]
fn escape_carriage_return() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""a\rb""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\rb");
}

#[test]
fn escape_backspace_formfeed() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""a\b\fb""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\u{0008}\u{000C}b");
}

#[test]
fn escape_unicode() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\u0041""#); // U+0041 = 'A'

  assert_eq!(parser.parse_next(&mut input).unwrap(), "A");
}

#[test]
fn escape_unicode_japanese() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\u3042""#); // U+3042 = 'あ'

  assert_eq!(parser.parse_next(&mut input).unwrap(), "あ");
}

#[test]
fn escape_unicode_mixed() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""hello\u0020world""#);

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello world");
}

// ── error cases ──────────────────────────────

#[test]
fn not_a_string() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new("hello");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn unterminated_string_is_cut() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new("\"hello");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn invalid_escape_is_cut() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\x""#);

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn incomplete_unicode_escape_is_cut() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\u00""#);

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn surrogate_pair_emoji() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\uD83D\uDE00""#);
  let result = parser.parse_next(&mut input).unwrap();
  assert_eq!(result, Cow::<str>::Owned("😀".to_string()));
}

#[test]
fn surrogate_pair_musical_symbol() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\uD834\uDD1E""#);
  let result = parser.parse_next(&mut input).unwrap();
  assert_eq!(result, Cow::<str>::Owned("𝄞".to_string()));
}

#[test]
fn lone_high_surrogate_is_error() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\uD83D""#);
  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn lone_low_surrogate_is_error() {
  let mut parser = quoted_string();
  let mut input = StrInputStream::new(r#""\uDE00""#);
  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}
