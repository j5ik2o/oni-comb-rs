use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::char;
use oni_comb_parser::str_input::StrInput;

// --- many0_into ---

#[test]
fn many0_into_vec_collects() {
  let mut parser = char('a').many0_into(Vec::new());
  let mut input = StrInput::new("aaab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'a', 'a']));
  assert_eq!(input.offset(), 3);
}

#[test]
fn many0_into_empty_returns_empty_container() {
  let mut parser = char('a').many0_into(Vec::new());
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec![]));
  assert_eq!(input.offset(), 0);
}

#[test]
fn many0_into_propagates_cut() {
  let item = char('a').zip(char('b').cut());
  let mut parser = item.many0_into(Vec::new());
  let mut input = StrInput::new("abac");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Cut(_))));
}

// --- many1_into ---

#[test]
fn many1_into_vec_collects() {
  let mut parser = char('a').many1_into(Vec::new());
  let mut input = StrInput::new("aaab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'a', 'a']));
  assert_eq!(input.offset(), 3);
}

#[test]
fn many1_into_zero_elements_is_error() {
  let mut parser = char('a').many1_into(Vec::new());
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

// --- sep_by0_into ---

#[test]
fn sep_by0_into_vec_collects() {
  let mut parser = char('a').sep_by0_into(char(','), Vec::new());
  let mut input = StrInput::new("a,a,a!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'a', 'a']));
  assert_eq!(input.offset(), 5);
}

#[test]
fn sep_by0_into_empty_returns_empty_container() {
  let mut parser = char('a').sep_by0_into(char(','), Vec::new());
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec![]));
  assert_eq!(input.offset(), 0);
}

// --- sep_by1_into ---

#[test]
fn sep_by1_into_vec_collects() {
  let mut parser = char('a').sep_by1_into(char(','), Vec::new());
  let mut input = StrInput::new("a,a,a!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(vec!['a', 'a', 'a']));
  assert_eq!(input.offset(), 5);
}

#[test]
fn sep_by1_into_zero_elements_is_error() {
  let mut parser = char('a').sep_by1_into(char(','), Vec::new());
  let mut input = StrInput::new("xyz");

  let result = parser.parse_next(&mut input);

  assert!(matches!(result, Err(Fail::Backtrack(_))));
}

// --- custom Extend type ---

#[derive(Debug, PartialEq, Clone)]
struct Counter {
  count: usize,
}

impl Extend<char> for Counter {
  fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
    for _ in iter {
      self.count += 1;
    }
  }
}

#[test]
fn many0_into_custom_extend_type() {
  let mut parser = char('a').many0_into(Counter { count: 0 });
  let mut input = StrInput::new("aaab");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(Counter { count: 3 }));
}

#[test]
fn sep_by0_into_custom_extend_type() {
  let mut parser = char('a').sep_by0_into(char(','), Counter { count: 0 });
  let mut input = StrInput::new("a,a,a!");

  let result = parser.parse_next(&mut input);

  assert_eq!(result, Ok(Counter { count: 3 }));
}
