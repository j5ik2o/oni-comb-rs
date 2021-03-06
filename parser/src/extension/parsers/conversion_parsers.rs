use std::fmt::Debug;
use std::str::FromStr;

use crate::core::Parsers;

pub trait ConversionParsers: Parsers {
  fn map_res<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Result<B, E> + 'a,
    E: Debug,
    A: Debug + 'a,
    B: Debug + 'a;

  fn map_opt<'a, I, A, B, E, F>(parser: Self::P<'a, I, A>, f: F) -> Self::P<'a, I, B>
  where
    F: Fn(A) -> Option<B> + 'a,
    A: Debug + 'a,
    B: Debug + 'a;

  fn convert_from_bytes_to_str<'a, I>(parser: Self::P<'a, I, &'a [u8]>) -> Self::P<'a, I, &'a str> {
    Self::map_res(parser, std::str::from_utf8)
  }

  fn convert_from_str_to_f64<'a, I>(parser: Self::P<'a, I, &'a str>) -> Self::P<'a, I, f64> {
    Self::map_res(parser, f64::from_str)
  }
}
