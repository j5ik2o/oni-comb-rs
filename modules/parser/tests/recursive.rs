use oni_comb_parser::fail::Fail;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// ── 基本テスト ────────────────────────────────

#[test]
fn recursive_simple_parentheses() {
  // value = "x" | "(" value ")"
  let value = recursive(|value| tag("x").or(between(tag("("), value, tag(")"))));

  let mut input = StrInput::new("x");
  assert_eq!(value.clone().parse_next(&mut input).unwrap(), "x");

  let mut input = StrInput::new("(x)");
  assert_eq!(value.clone().parse_next(&mut input).unwrap(), "x");

  let mut input = StrInput::new("((x))");
  assert_eq!(value.clone().parse_next(&mut input).unwrap(), "x");

  let mut input = StrInput::new("(((x)))");
  assert_eq!(value.clone().parse_next(&mut input).unwrap(), "x");
}

#[test]
fn recursive_fail_propagation() {
  let value = recursive(|value| tag("x").or(between(tag("("), value, tag(")"))));

  let mut input = StrInput::new("y");
  assert!(matches!(value.clone().parse_next(&mut input), Err(Fail::Backtrack(_))));
  assert_eq!(input.offset(), 0);
}

#[test]
fn recursive_unclosed_paren() {
  let value = recursive(|value| tag("x").or(tag("(").zip_right(value).zip_left(tag(")").cut())));

  let mut input = StrInput::new("(x");
  // "(" 消費 → "x" 成功 → ")" が見つからず Cut
  assert!(matches!(value.clone().parse_next(&mut input), Err(Fail::Cut(_))));
}

#[test]
fn recursive_with_map() {
  // depth counter: "x" = 0, "(" value ")" = value + 1
  let depth = recursive(|depth| {
    let base = tag("x").map(|_| 0i64);
    let nested = between(tag("("), depth, tag(")")).map(|d: i64| d + 1);
    base.or(nested)
  });

  let mut input = StrInput::new("x");
  assert_eq!(depth.clone().parse_next(&mut input).unwrap(), 0);

  let mut input = StrInput::new("(x)");
  assert_eq!(depth.clone().parse_next(&mut input).unwrap(), 1);

  let mut input = StrInput::new("(((x)))");
  assert_eq!(depth.clone().parse_next(&mut input).unwrap(), 3);
}

// ── リスト構造 ────────────────────────────────

#[test]
fn recursive_nested_list() {
  // list = "[" (value ("," value)*)? "]"
  // value = integer | list
  // ただし integer は簡易的に1桁
  let value = recursive(|value| {
    let int = satisfy(|c: char| c.is_ascii_digit()).map(|c: char| c.to_string());
    let list = between(char('['), value.sep_by0(char(',')), char(']'))
      .map(|items: Vec<String>| format!("[{}]", items.join(",")));
    int.or(list)
  });

  let mut input = StrInput::new("[1,[2,3],4]");
  assert_eq!(value.clone().parse_next(&mut input).unwrap(), "[1,[2,3],4]");
}
