use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

fn single_quote_string(
) -> impl Parser<oni_comb_parser::str_input_stream::StrInputStream<'static>, Output = String, Error = ParseError> {
  escaped('\'', '\'', '\\', |c| match c {
    '\'' => Some('\''),
    '\\' => Some('\\'),
    'n' => Some('\n'),
    't' => Some('\t'),
    _ => None,
  })
}

#[test]
fn escaped_simple() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new("'hello'");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.offset(), 7);
}

#[test]
fn escaped_empty() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new("''");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "");
}

#[test]
fn escaped_with_escape_sequences() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new(r"'it\'s a\ntest'");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "it's a\ntest");
}

#[test]
fn escaped_backslash() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new(r"'a\\b'");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a\\b");
}

#[test]
fn escaped_no_open_delimiter() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new("hello");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn escaped_unterminated_is_cut() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new("'hello");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn escaped_invalid_escape_is_cut() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new(r"'hello\x'");

  assert!(matches!(parser.parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn escaped_custom_delimiters() {
  // バッククォートで囲み、$ をエスケープ文字とする
  let mut parser = escaped('`', '`', '$', |c| match c {
    '$' => Some('$'),
    '`' => Some('`'),
    _ => None,
  });
  let mut input = StrInputStream::new("`a$`b$$c`");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "a`b$c");
}

#[test]
fn escaped_with_remaining_input() {
  let mut parser = single_quote_string();
  let mut input = StrInputStream::new("'hello' world");

  assert_eq!(parser.parse_next(&mut input).unwrap(), "hello");
  assert_eq!(input.remaining(), " world");
}
