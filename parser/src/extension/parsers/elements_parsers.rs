use crate::core::Parsers;
use std::fmt::Debug;

pub trait ElementsParsers: Parsers {
  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, Vec<I>>
  where
    I: PartialEq + Debug + Clone + 'a,
    'b: 'a;

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a;

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, String>
  where
    'b: 'a;

  fn regex<'a>(pattern: &str) -> Self::P<'a, char, String>;
}
