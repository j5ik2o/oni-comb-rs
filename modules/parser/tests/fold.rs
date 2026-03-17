use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::{char, tag};
use oni_comb_parser::str_input::StrInput;

// --- many0_fold ---

#[test]
fn many0_fold_zero_elements_returns_init() {
  let mut parser = char('a').many0_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(0));
  assert_eq!(input.offset(), 0);
}

#[test]
fn many0_fold_multiple_elements() {
  let mut parser = char('a').many0_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("aaab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(3));
  assert_eq!(input.offset(), 3);
}

#[test]
fn many0_fold_accumulates_values() {
  let mut parser = char('a')
    .or(char('b'))
    .many0_fold(String::new, |mut acc, c| { acc.push(c); acc });
  let mut input = StrInput::new("abba!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok("abba".to_string()));
  assert_eq!(input.offset(), 4);
}

#[test]
fn many0_fold_propagates_cut() {
  let item = char('a').zip(char('b').cut());
  let mut parser = item.many0_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("abac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

#[test]
fn many0_fold_detects_zero_progress() {
  let mut parser = tag("").many0_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("anything");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::ZeroProgress)));
}

#[test]
fn many0_fold_empty_input() {
  let mut parser = char('a').many0_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(0));
}

// --- many1_fold ---

#[test]
fn many1_fold_one_element() {
  let mut parser = char('a').many1_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("ab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(1));
  assert_eq!(input.offset(), 1);
}

#[test]
fn many1_fold_multiple_elements() {
  let mut parser = char('a').many1_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("aaab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(3));
  assert_eq!(input.offset(), 3);
}

#[test]
fn many1_fold_zero_elements_is_error() {
  let mut parser = char('a').many1_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

#[test]
fn many1_fold_propagates_cut() {
  let item = char('a').zip(char('b').cut());
  let mut parser = item.many1_fold(|| 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("abac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

// --- sep_by0_fold ---

#[test]
fn sep_by0_fold_zero_elements_returns_init() {
  let mut parser = char('a').sep_by0_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(0));
  assert_eq!(input.offset(), 0);
}

#[test]
fn sep_by0_fold_multiple_elements() {
  let mut parser = char('a').sep_by0_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("a,a,a!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(3));
  assert_eq!(input.offset(), 5);
}

#[test]
fn sep_by0_fold_rejects_trailing_separator() {
  let mut parser = char('a').sep_by0_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("a,a,");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(2));
  assert_eq!(input.offset(), 3); // 末尾の , は巻き戻し
}

#[test]
fn sep_by0_fold_propagates_cut() {
  let item = char('a').zip(char('b').cut()).map(|(a, b)| (a, b));
  let mut parser = item.sep_by0_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("ab,ac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

// --- sep_by1_fold ---

#[test]
fn sep_by1_fold_zero_elements_is_error() {
  let mut parser = char('a').sep_by1_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

#[test]
fn sep_by1_fold_one_element() {
  let mut parser = char('a').sep_by1_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("a!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(1));
  assert_eq!(input.offset(), 1);
}

#[test]
fn sep_by1_fold_multiple_elements() {
  let mut parser = char('a').sep_by1_fold(char(','), || 0usize, |acc, _| acc + 1);
  let mut input = StrInput::new("a,a,a!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(3));
  assert_eq!(input.offset(), 5);
}
