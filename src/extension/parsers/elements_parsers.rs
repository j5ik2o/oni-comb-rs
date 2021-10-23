use crate::core::Parsers;
use regex::Regex;
use std::fmt::Debug;

pub trait ElementsParsers: Parsers {
  fn seq<'a, 'b, I>(tag: &'b [I]) -> Self::P<'a, I, &'a [I]>
  where
    I: PartialEq + Debug + 'a,
    'b: 'a;

  fn tag<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a;

  fn tag_no_case<'a, 'b>(tag: &'b str) -> Self::P<'a, char, &'a str>
  where
    'b: 'a;

  fn regex<'a>(regex: Regex) -> Self::P<'a, char, String>;
}
