use oni_comb_parser::byte_input_stream::ByteInputStream;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::primitive::eof::eof;
use oni_comb_parser::primitive::satisfy::satisfy;
use oni_comb_parser::primitive::take::take;
use oni_comb_parser::primitive::take_while0::take_while0;
use oni_comb_parser::primitive::take_while1::take_while1;

#[test]
fn byte_map_to_uppercase() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b.is_ascii_lowercase()).map(|b: u8| b - b'a' + b'A');
  let mut input = ByteInputStream::new(b"hello");
  assert_eq!(p.parse_next(&mut input).unwrap(), b'H');
}

#[test]
fn byte_zip_two_takes() {
  let mut p = take::<ByteInputStream>(2).zip(take::<ByteInputStream>(3));
  let mut input = ByteInputStream::new(b"abcde");
  let (a, b) = p.parse_next(&mut input).unwrap();
  assert_eq!(a, b"ab");
  assert_eq!(b, b"cde");
}

#[test]
fn byte_zip_left() {
  let mut p = take::<ByteInputStream>(3).zip_left(take::<ByteInputStream>(2));
  let mut input = ByteInputStream::new(b"abcde");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"abc");
  assert!(input.is_eof());
}

#[test]
fn byte_zip_right() {
  let mut p = take::<ByteInputStream>(2).zip_right(take::<ByteInputStream>(3));
  let mut input = ByteInputStream::new(b"abcde");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"cde");
  assert!(input.is_eof());
}

#[test]
fn byte_or_first_success() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b == b'a').or(satisfy::<ByteInputStream, _>(|b: u8| b == b'b'));
  let mut input = ByteInputStream::new(b"a");
  assert_eq!(p.parse_next(&mut input).unwrap(), b'a');
}

#[test]
fn byte_or_second_success() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b == b'a').or(satisfy::<ByteInputStream, _>(|b: u8| b == b'b'));
  let mut input = ByteInputStream::new(b"b");
  assert_eq!(p.parse_next(&mut input).unwrap(), b'b');
}

#[test]
fn byte_or_both_fail() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b == b'a').or(satisfy::<ByteInputStream, _>(|b: u8| b == b'b'));
  let mut input = ByteInputStream::new(b"c");
  assert!(p.parse_next(&mut input).is_err());
}

#[test]
fn byte_many0_collect() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b.is_ascii_digit()).many0();
  let mut input = ByteInputStream::new(b"123abc");
  let result = p.parse_next(&mut input).unwrap();
  assert_eq!(result, vec![b'1', b'2', b'3']);
  assert_eq!(input.remaining(), b"abc");
}

#[test]
fn byte_many0_empty() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b.is_ascii_digit()).many0();
  let mut input = ByteInputStream::new(b"abc");
  let result = p.parse_next(&mut input).unwrap();
  assert!(result.is_empty());
}

#[test]
fn byte_optional_some() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b == b'x').optional();
  let mut input = ByteInputStream::new(b"x");
  assert_eq!(p.parse_next(&mut input).unwrap(), Some(b'x'));
}

#[test]
fn byte_optional_none() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b == b'x').optional();
  let mut input = ByteInputStream::new(b"y");
  assert_eq!(p.parse_next(&mut input).unwrap(), None);
}

#[test]
fn byte_take_while0_then_eof() {
  let mut p = take_while0::<ByteInputStream, _>(|b: u8| b != b'\n').zip(eof::<ByteInputStream>());
  let mut input = ByteInputStream::new(b"hello");
  let (line, _) = p.parse_next(&mut input).unwrap();
  assert_eq!(line, b"hello");
}

#[test]
fn byte_take_while1_then_take() {
  let mut p = take_while1::<ByteInputStream, _>(|b: u8| b.is_ascii_alphabetic()).zip(take::<ByteInputStream>(1));
  let mut input = ByteInputStream::new(b"abc!");
  let (letters, bang) = p.parse_next(&mut input).unwrap();
  assert_eq!(letters, b"abc");
  assert_eq!(bang, b"!");
}

#[test]
fn byte_attempt_rewinds() {
  let mut p = take::<ByteInputStream>(3)
    .zip(satisfy::<ByteInputStream, _>(|b: u8| b == b'X'))
    .map(|(s, _)| s)
    .attempt()
    .or(take::<ByteInputStream>(2));
  let mut input = ByteInputStream::new(b"abcd");
  // First branch: take(3)="abc", satisfy('X') fails, attempt rewinds
  // Second branch: take(2)="ab"
  assert_eq!(p.parse_next(&mut input).unwrap(), b"ab");
}

#[test]
fn byte_sep_by0() {
  let mut p = satisfy::<ByteInputStream, _>(|b: u8| b.is_ascii_digit()).sep_by0(satisfy::<ByteInputStream, _>(|b: u8| b == b','));
  let mut input = ByteInputStream::new(b"1,2,3!");
  let result = p.parse_next(&mut input).unwrap();
  assert_eq!(result, vec![b'1', b'2', b'3']);
  assert_eq!(input.remaining(), b"!");
}
