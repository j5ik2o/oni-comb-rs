use oni_comb_parser::byte_input::ByteInput;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::primitive::eof::eof;
use oni_comb_parser::primitive::satisfy::satisfy;
use oni_comb_parser::primitive::take::take;
use oni_comb_parser::primitive::take_while0::take_while0;
use oni_comb_parser::primitive::take_while1::take_while1;
use oni_comb_parser::primitive::take_while_n_m::take_while_n_m;

#[test]
fn byte_input_take_3() {
  let mut p = take::<ByteInput>(3);
  let mut input = ByteInput::new(b"abcdef");
  let result = p.parse_next(&mut input).unwrap();
  assert_eq!(result, b"abc");
  assert_eq!(input.remaining(), b"def");
}

#[test]
fn byte_input_take_not_enough() {
  let mut p = take::<ByteInput>(5);
  let mut input = ByteInput::new(b"ab");
  assert!(p.parse_next(&mut input).is_err());
  assert_eq!(input.offset(), 0); // reset to start
}

#[test]
fn byte_input_satisfy_match() {
  let mut p = satisfy::<ByteInput, _>(|b: u8| b.is_ascii_uppercase());
  let mut input = ByteInput::new(b"Hello");
  assert_eq!(p.parse_next(&mut input).unwrap(), b'H');
  assert_eq!(input.remaining(), b"ello");
}

#[test]
fn byte_input_satisfy_no_match() {
  let mut p = satisfy::<ByteInput, _>(|b: u8| b.is_ascii_uppercase());
  let mut input = ByteInput::new(b"hello");
  assert!(p.parse_next(&mut input).is_err());
}

#[test]
fn byte_input_satisfy_empty() {
  let mut p = satisfy::<ByteInput, _>(|_: u8| true);
  let mut input = ByteInput::new(b"");
  assert!(p.parse_next(&mut input).is_err());
}

#[test]
fn byte_input_take_while0_digits() {
  let mut p = take_while0::<ByteInput, _>(|b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"123abc");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"123");
  assert_eq!(input.remaining(), b"abc");
}

#[test]
fn byte_input_take_while0_no_match() {
  let mut p = take_while0::<ByteInput, _>(|b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"abc");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"");
}

#[test]
fn byte_input_take_while0_empty() {
  let mut p = take_while0::<ByteInput, _>(|b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"");
}

#[test]
fn byte_input_take_while1_digits() {
  let mut p = take_while1::<ByteInput, _>(|b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"42abc");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"42");
}

#[test]
fn byte_input_take_while1_no_match() {
  let mut p = take_while1::<ByteInput, _>(|b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"abc");
  assert!(p.parse_next(&mut input).is_err());
}

#[test]
fn byte_input_take_while_n_m_bounded() {
  let mut p = take_while_n_m::<ByteInput, _>(2, 4, |b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"12345");
  assert_eq!(p.parse_next(&mut input).unwrap(), b"1234");
  assert_eq!(input.remaining(), b"5");
}

#[test]
fn byte_input_take_while_n_m_below_min() {
  let mut p = take_while_n_m::<ByteInput, _>(3, 5, |b: u8| b.is_ascii_digit());
  let mut input = ByteInput::new(b"12abc");
  assert!(p.parse_next(&mut input).is_err());
  assert_eq!(input.offset(), 0); // reset
}

#[test]
fn byte_input_eof_at_end() {
  let mut p = eof::<ByteInput>();
  let mut input = ByteInput::new(b"");
  assert!(p.parse_next(&mut input).is_ok());
}

#[test]
fn byte_input_eof_not_at_end() {
  let mut p = eof::<ByteInput>();
  let mut input = ByteInput::new(b"a");
  assert!(p.parse_next(&mut input).is_err());
}

#[test]
fn byte_input_checkpoint_reset() {
  let mut input = ByteInput::new(b"abc");
  let cp = input.checkpoint();
  assert_eq!(input.next_token(), Some(b'a'));
  assert_eq!(input.next_token(), Some(b'b'));
  assert_eq!(input.offset(), 2);
  input.reset(cp);
  assert_eq!(input.offset(), 0);
  assert_eq!(input.next_token(), Some(b'a'));
}

#[test]
fn byte_input_peek_token() {
  let mut input = ByteInput::new(b"ab");
  assert_eq!(input.peek_token(), Some(b'a'));
  assert_eq!(input.peek_token(), Some(b'a')); // does not consume
  input.next_token();
  assert_eq!(input.peek_token(), Some(b'b'));
}

#[test]
fn byte_input_slice_since() {
  let mut input = ByteInput::new(b"hello");
  let cp = input.checkpoint();
  input.next_token(); // h
  input.next_token(); // e
  input.next_token(); // l
  assert_eq!(input.slice_since(cp), b"hel");
}
