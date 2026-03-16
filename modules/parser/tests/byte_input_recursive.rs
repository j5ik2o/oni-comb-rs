use oni_comb_parser::byte_input::ByteInput;
use oni_comb_parser::combinator::recursive::recursive;
use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::primitive::satisfy::satisfy;
use oni_comb_parser::primitive::take_while1::take_while1;

/// 簡易的な再帰パーサー: 括弧で囲まれたバイト列、またはアルファベット列
/// e.g. b"(abc)", b"((abc))", b"hello"
#[test]
fn byte_recursive_nested_parens() {
  let parser = recursive::<ByteInput, &[u8], ParseError, _, _>(|rec| {
    let inner = take_while1::<ByteInput, _>(|b: u8| b.is_ascii_alphabetic());
    // ( rec ) or inner
    let parens = satisfy::<ByteInput, _>(|b: u8| b == b'(')
      .zip_right(rec)
      .zip_left(satisfy::<ByteInput, _>(|b: u8| b == b')'));
    parens.or(inner)
  });

  let mut p = parser.clone();
  let mut input = ByteInput::new(b"hello");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"hello");

  let mut p = parser.clone();
  let mut input = ByteInput::new(b"(abc)");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"abc");

  let mut p = parser.clone();
  let mut input = ByteInput::new(b"((xyz))");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"xyz");
}

#[test]
fn byte_recursive_fail() {
  let parser = recursive::<ByteInput, &[u8], ParseError, _, _>(|rec| {
    let inner = take_while1::<ByteInput, _>(|b: u8| b.is_ascii_alphabetic());
    let parens = satisfy::<ByteInput, _>(|b: u8| b == b'(')
      .zip_right(rec)
      .zip_left(satisfy::<ByteInput, _>(|b: u8| b == b')'));
    parens.or(inner)
  });

  let mut p = parser;
  let mut input = ByteInput::new(b"123");
  assert!(p.parse_next(&mut input).is_err());
}
