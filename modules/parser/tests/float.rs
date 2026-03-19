use oni_comb_parser::fail::Fail;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

#[test]
fn float_integer() {
  let mut input = StrInputStream::new("42");
  assert_eq!(float().parse_next(&mut input).unwrap(), 42.0);
}

#[test]
fn float_negative() {
  let mut input = StrInputStream::new("-7");
  assert_eq!(float().parse_next(&mut input).unwrap(), -7.0);
}

#[test]
fn float_decimal() {
  let mut input = StrInputStream::new("3.14");
  assert_eq!(float().parse_next(&mut input).unwrap(), 3.14);
}

#[test]
fn float_exponent() {
  let mut input = StrInputStream::new("1.5e10");
  assert_eq!(float().parse_next(&mut input).unwrap(), 1.5e10);
}

#[test]
fn float_negative_exponent() {
  let mut input = StrInputStream::new("2.5E-3");
  assert_eq!(float().parse_next(&mut input).unwrap(), 2.5e-3);
}

#[test]
fn float_zero() {
  let mut input = StrInputStream::new("0");
  assert_eq!(float().parse_next(&mut input).unwrap(), 0.0);
}

#[test]
fn float_leading_zero_stops() {
  let mut input = StrInputStream::new("007");
  assert_eq!(float().parse_next(&mut input).unwrap(), 0.0);
  assert_eq!(input.remaining(), "07");
}

#[test]
fn float_dot_only_fails() {
  let mut input = StrInputStream::new(".5");
  assert!(matches!(float().parse_next(&mut input), Err(Fail::Backtrack(_))));
}

#[test]
fn float_positive_exponent() {
  let mut input = StrInputStream::new("1e+2");
  assert_eq!(float().parse_next(&mut input).unwrap(), 1e+2);
}
