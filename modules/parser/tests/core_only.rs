//! Tests for core-only (no alloc) functionality.
//! These tests verify that the core parsers work correctly with
//! the default error type (ParseError when alloc is enabled,
//! MinimalError when not). The parsers themselves are alloc-free.

use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

// --- char (core-only text parser) ---

#[test]
fn core_char_matches() {
  let mut parser = char('a');
  let mut input = StrInputStream::new("abc");
  assert_eq!(parser.parse_next(&mut input).unwrap(), 'a');
  assert_eq!(input.offset(), 1);
}

#[test]
fn core_char_fails() {
  let mut parser = char('x');
  let mut input = StrInputStream::new("abc");
  assert!(parser.parse_next(&mut input).is_err());
  assert_eq!(input.offset(), 0);
}

// --- tag (core-only text parser) ---

#[test]
fn core_tag_matches() {
  let mut parser = tag("AT+");
  let mut input = StrInputStream::new("AT+CMD");
  assert_eq!(parser.parse_next(&mut input).unwrap(), "AT+");
  assert_eq!(input.offset(), 3);
}

// --- identifier (core-only text parser) ---

#[test]
fn core_identifier_matches() {
  let mut parser = identifier();
  let mut input = StrInputStream::new("foo_123 ");
  assert_eq!(parser.parse_next(&mut input).unwrap(), "foo_123");
}

// --- integer (core-only text parser) ---

#[test]
fn core_integer_matches() {
  let mut parser = integer();
  let mut input = StrInputStream::new("42");
  assert_eq!(parser.parse_next(&mut input).unwrap(), 42);
}

#[test]
fn core_integer_negative() {
  let mut parser = integer();
  let mut input = StrInputStream::new("-7");
  assert_eq!(parser.parse_next(&mut input).unwrap(), -7);
}

// --- satisfy (core-only primitive parser) ---

#[test]
fn core_satisfy_matches() {
  let mut parser = satisfy(|c: char| c.is_ascii_digit());
  let mut input = StrInputStream::new("9x");
  assert_eq!(parser.parse_next(&mut input).unwrap(), '9');
}

// --- fold combinators (core-only, zero-allocation) ---

#[test]
fn core_many0_fold_counts() {
  let mut parser = char('a').many0_fold(|| 0usize, |n, _| n + 1);
  let mut input = StrInputStream::new("aaab");
  assert_eq!(parser.parse_next(&mut input).unwrap(), 3);
  assert_eq!(input.offset(), 3);
}

#[test]
fn core_sep_by0_fold_sums_digits() {
  let digit = satisfy(|c: char| c.is_ascii_digit()).map(|c: char| (c as u64) - ('0' as u64));
  let mut parser = digit.sep_by0_fold(char(','), || 0u64, |sum, d| sum + d);
  let mut input = StrInputStream::new("1,2,3!");
  assert_eq!(parser.parse_next(&mut input).unwrap(), 6);
  assert_eq!(input.offset(), 5);
}

// --- combinators (core-only) ---

#[test]
fn core_or_works() {
  let mut parser = char('a').or(char('b'));
  let mut input = StrInputStream::new("b");
  assert_eq!(parser.parse_next(&mut input).unwrap(), 'b');
}

#[test]
fn core_zip_works() {
  let mut parser = tag("AT+").zip_right(identifier());
  let mut input = StrInputStream::new("AT+CMD");
  assert_eq!(parser.parse_next(&mut input).unwrap(), "CMD");
}

#[test]
fn core_optional_works() {
  let mut parser = char('-').optional().zip(integer());
  let mut input = StrInputStream::new("42");
  let (sign, val) = parser.parse_next(&mut input).unwrap();
  assert_eq!(sign, None);
  assert_eq!(val, 42);
}

#[test]
fn core_map_res_works() {
  let mut parser = take_while1(|c: char| c.is_ascii_digit()).map_res(|s: &str| s.parse::<u32>(), "u32");
  let mut input = StrInputStream::new("999");
  assert_eq!(parser.parse_next(&mut input).unwrap(), 999);
}
