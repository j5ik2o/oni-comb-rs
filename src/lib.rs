#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]

mod basic_parsers;
pub mod combinator;
pub mod combinators;
pub mod location;
pub mod parse_error;
pub mod parse_result;
pub mod parse_state;
pub mod parser;
pub mod parsers;
pub mod range;
pub mod simple_parser;
pub mod simple_parsers;

use std::fmt::Debug;

// https://github.com/com-lihaoyi/fastparse
// https://github.com/fpinscala/fpinscala/blob/first-edition/answers/src/main/scala/fpinscala/parsing
// https://github.com/Geal/nom
// https://hazm.at/mox/lang/rust/nom/index.html
// https://github.com/J-F-Liu/pom

fn first_nonmatching_index(s1: String, s2: &str, offset: usize) -> Option<usize> {
  let mut i = 0usize;
  while i < s1.len() && i < s2.len() {
    if s1.chars().nth(i + offset) != s2.chars().nth(i) {
      return Some(i);
    }
    i += 1;
  }
  if s1.len() - offset >= s2.len() {
    None
  } else {
    Some(s1.len() - offset)
  }
}

#[derive(Debug)]
pub struct Tuple<A, B> {
  a: A,
  b: B,
}

impl<A: Clone + Debug, B: Clone + Debug> Clone for Tuple<A, B> {
  fn clone(&self) -> Self {
    Self {
      a: self.a.clone(),
      b: self.b.clone(),
    }
  }
}

impl<A, B> Tuple<A, B> {
  pub fn new(a: A, b: B) -> Self {
    Self { a, b }
  }
}
