use std::fmt::{Debug, Display};

use regex::Regex;

use crate::core::element::Element;
use crate::core::Parsers;
use crate::utils::Set;

pub trait BasicParsers: Parsers {
  fn begin<'a, I>() -> Self::P<'a, I, ()> {
    Self::empty()
  }

  fn end<'a, I>() -> Self::P<'a, I, ()>
  where
    I: Debug + Display + 'a;

  fn empty<'a, I>() -> Self::P<'a, I, ()>;
}
