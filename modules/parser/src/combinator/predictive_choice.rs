use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::byte_input_stream::ByteInputStream;
use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input_stream::InputStream;
use crate::parser::Parser;
use crate::str_input_stream::StrInputStream;

trait PredictiveByteInput: InputStream {
  fn peek_byte_for_choice(&self) -> Option<u8>;
}

impl<'a> PredictiveByteInput for StrInputStream<'a> {
  #[inline]
  fn peek_byte_for_choice(&self) -> Option<u8> {
    self.peek_byte()
  }
}

impl<'a> PredictiveByteInput for ByteInputStream<'a> {
  #[inline]
  fn peek_byte_for_choice(&self) -> Option<u8> {
    self.peek_byte()
  }
}

enum ByteMatcher {
  Exact(u8),
  Predicate(fn(u8) -> bool),
}

impl ByteMatcher {
  #[inline]
  fn matches(&self, byte: u8) -> bool {
    match self {
      ByteMatcher::Exact(expected) => *expected == byte,
      ByteMatcher::Predicate(pred) => pred(byte),
    }
  }
}

struct ByteBranch<'a, I: InputStream, O> {
  matcher: ByteMatcher,
  parser: Box<dyn Parser<I, Output = O, Error = I::Error> + 'a>,
}

pub struct PredictiveChoice<'a, I: InputStream, O> {
  branches: Vec<ByteBranch<'a, I, O>>,
  _marker: PhantomData<fn(&'a mut I) -> O>,
}

pub fn predictive_choice<'a, I: InputStream, O>() -> PredictiveChoice<'a, I, O> {
  PredictiveChoice {
    branches: Vec::new(),
    _marker: PhantomData,
  }
}

impl<'a, I: InputStream, O> PredictiveChoice<'a, I, O> {
  pub fn when_byte<P>(mut self, byte: u8, parser: P) -> Self
  where
    P: Parser<I, Output = O, Error = I::Error> + 'a,
  {
    self.branches.push(ByteBranch {
      matcher: ByteMatcher::Exact(byte),
      parser: Box::new(parser),
    });
    self
  }

  pub fn when_predicate<P>(mut self, predicate: fn(u8) -> bool, parser: P) -> Self
  where
    P: Parser<I, Output = O, Error = I::Error> + 'a,
  {
    self.branches.push(ByteBranch {
      matcher: ByteMatcher::Predicate(predicate),
      parser: Box::new(parser),
    });
    self
  }
}

impl<'a, I, O> Parser<I> for PredictiveChoice<'a, I, O>
where
  I: PredictiveByteInput,
  I::Error: ExpectError,
{
  type Error = I::Error;
  type Output = O;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<O, I::Error> {
    let Some(byte) = input.peek_byte_for_choice() else {
      return Err(Fail::Backtrack(I::Error::from_position(
        input.position(),
        Expected::Description("predictive choice branch"),
      )));
    };

    for branch in &mut self.branches {
      if branch.matcher.matches(byte) {
        return branch.parser.parse_next(input);
      }
    }

    Err(Fail::Backtrack(I::Error::from_position(
      input.position(),
      Expected::Description("predictive choice branch"),
    )))
  }
}
