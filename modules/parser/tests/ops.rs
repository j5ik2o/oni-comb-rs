use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

#[test]
fn add_is_zip() {
  let mut input = StrInputStream::new("ab");
  let result = (sym('a').ops() + sym('b').ops()).parse_next(&mut input).unwrap();
  assert_eq!(result, ('a', 'b'));
}

#[test]
fn sub_is_zip_left() {
  let mut input = StrInputStream::new("ab");
  let result = (sym('a').ops() - sym('b').ops()).parse_next(&mut input).unwrap();
  assert_eq!(result, 'a');
  assert_eq!(input.remaining(), "");
}

#[test]
fn mul_is_zip_right() {
  let mut input = StrInputStream::new("ab");
  let result = (sym('a').ops() * sym('b').ops()).parse_next(&mut input).unwrap();
  assert_eq!(result, 'b');
}

#[test]
fn bitor_is_or() {
  let mut input = StrInputStream::new("bc");
  let result = (sym('a').ops() | sym('b').ops()).parse_next(&mut input).unwrap();
  assert_eq!(result, 'b');
}

#[test]
fn not_op_is_negative_lookahead() {
  let mut input = StrInputStream::new("bc");
  let result = (!sym('a').ops()).parse_next(&mut input).unwrap();
  assert_eq!(result, ());
  assert_eq!(input.remaining(), "bc");
}

#[test]
fn neg_op_is_peek() {
  let mut input = StrInputStream::new("abc");
  let _result = (-sym('a').ops()).parse_next(&mut input).unwrap();
  assert_eq!(input.remaining(), "abc"); // didn't consume
}

#[test]
fn shr_is_flat_map() {
  let mut input = StrInputStream::new("ab");
  let result = (sym('a').ops() >> |_: char| sym('b')).parse_next(&mut input).unwrap();
  assert_eq!(result, 'b');
}

#[test]
fn chained_ops_work() {
  // pom style: sym('(') * sym('x') - sym(')')
  let mut input = StrInputStream::new("(x)");
  let result = (sym('(').ops() * sym('x').ops() - sym(')').ops())
    .parse_next(&mut input)
    .unwrap();
  assert_eq!(result, 'x');
  assert_eq!(input.remaining(), "");
}

#[test]
fn or_chain_works() {
  let mut input = StrInputStream::new("c");
  let result = (sym('a').ops() | sym('b').ops() | sym('c').ops())
    .parse_next(&mut input)
    .unwrap();
  assert_eq!(result, 'c');
}
