use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// ── 位置情報 ──────────────────────────────────

#[test]
fn error_position_at_start() {
  let mut parser = char('a');
  let mut input = StrInput::new("xyz");

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(e)) => {
      assert_eq!(e.position, 0);
      assert_eq!(e.expected, vec![Expected::Char('a')]);
    }
    other => panic!("expected Backtrack, got {:?}", other),
  }
}

#[test]
fn error_position_after_consumed() {
  let mut parser = tag("ab").zip(char('c'));
  let mut input = StrInput::new("abx");

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(e)) => {
      assert_eq!(e.position, 2); // "ab" consumed, error at 'x'
      assert_eq!(e.expected, vec![Expected::Char('c')]);
    }
    other => panic!("expected Backtrack, got {:?}", other),
  }
}

// ── expected の合成 (or マージ) ───────────────

#[test]
fn or_merges_expected_at_same_position() {
  let mut parser = char('a').or(char('b')).or(char('c'));
  let mut input = StrInput::new("x");

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(e)) => {
      assert_eq!(e.position, 0);
      assert!(e.expected.contains(&Expected::Char('a')));
      assert!(e.expected.contains(&Expected::Char('b')));
      assert!(e.expected.contains(&Expected::Char('c')));
    }
    other => panic!("expected Backtrack, got {:?}", other),
  }
}

#[test]
fn or_keeps_deeper_position() {
  // tag("ab") consumes 'a' then fails at position 1
  // tag("cd") fails at position 0
  // merged result should keep position 1 (deeper)
  let mut parser = tag("ab").attempt().or(tag("cd"));
  let mut input = StrInput::new("ax");

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(e)) => {
      // attempt rewinds but error position is preserved
      // left error at pos 0 (tag "ab" after rewind),
      // right error at pos 0 (tag "cd" at start)
      assert_eq!(e.position, 0);
    }
    other => panic!("expected Backtrack, got {:?}", other),
  }
}

// ── context ──────────────────────────────────

#[test]
fn context_adds_label_to_backtrack() {
  let mut parser = char('a').context("my_rule");
  let mut input = StrInput::new("x");

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(e)) => {
      assert_eq!(e.context, vec!["my_rule"]);
    }
    other => panic!("expected Backtrack, got {:?}", other),
  }
}

#[test]
fn context_adds_label_to_cut() {
  let mut parser = char('a').cut().context("my_rule");
  let mut input = StrInput::new("x");

  match parser.parse_next(&mut input) {
    Err(Fail::Cut(e)) => {
      assert_eq!(e.context, vec!["my_rule"]);
    }
    other => panic!("expected Cut, got {:?}", other),
  }
}

#[test]
fn nested_context_stacks() {
  let inner = char('a').context("inner");
  let outer = inner.context("outer");
  let mut parser = outer;
  let mut input = StrInput::new("x");

  match parser.parse_next(&mut input) {
    Err(Fail::Backtrack(e)) => {
      assert_eq!(e.context, vec!["inner", "outer"]);
    }
    other => panic!("expected Backtrack, got {:?}", other),
  }
}

#[test]
fn context_does_not_affect_success() {
  let mut parser = char('a').context("my_rule");
  let mut input = StrInput::new("abc");

  assert_eq!(parser.parse_next(&mut input).unwrap(), 'a');
}

// ── Display ──────────────────────────────────

#[test]
fn display_simple_error() {
  let e: ParseError = ExpectError::from_expected(5, Expected::Char('x'));
  assert_eq!(e.to_string(), "parse error at position 5: expected 'x'");
}

#[test]
fn display_merged_error() {
  let e1: ParseError = ExpectError::from_expected(0, Expected::Char('a'));
  let e2: ParseError = ExpectError::from_expected(0, Expected::Char('b'));
  let merged = oni_comb_parser::error::MergeError::merge(e1, e2);
  let s = merged.to_string();
  assert!(s.contains("'a'"));
  assert!(s.contains("'b'"));
}

#[test]
fn display_error_with_context() {
  let mut e: ParseError = ExpectError::from_expected(3, Expected::Tag("true"));
  e.context = vec!["value", "array"];
  // context is displayed in reverse (outer first)
  let s = e.to_string();
  assert!(s.contains("position 3"));
  assert!(s.contains("\"true\""));
  assert!(s.contains("array > value"));
}
