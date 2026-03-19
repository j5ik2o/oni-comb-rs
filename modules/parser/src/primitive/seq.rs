use core::marker::PhantomData;

use crate::error::ExpectError;
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

/// Input 型とタグスライス型の関係を定義するトレイト。
/// StrInput と &str、ByteInput と &[u8] のペアで実装される。
pub trait InputSeq<'a, T: ?Sized>: Input {
  fn starts_with(&self, tag: &T) -> bool;
  fn advance_by(&mut self, tag: &T);
  fn tag_to_expected(tag: &'static T) -> crate::error::Expected;
}

impl<'a> InputSeq<'a, str> for crate::str_input::StrInput<'a> {
  #[inline]
  fn starts_with(&self, tag: &str) -> bool {
    self.remaining().starts_with(tag)
  }

  #[inline]
  fn advance_by(&mut self, tag: &str) {
    self.advance(tag.len());
  }

  #[inline]
  fn tag_to_expected(tag: &'static str) -> crate::error::Expected {
    crate::error::Expected::Tag(tag)
  }
}

impl<'a> InputSeq<'a, [u8]> for crate::byte_input::ByteInput<'a> {
  #[inline]
  fn starts_with(&self, tag: &[u8]) -> bool {
    self.remaining().starts_with(tag)
  }

  #[inline]
  fn advance_by(&mut self, tag: &[u8]) {
    self.advance(tag.len());
  }

  #[inline]
  fn tag_to_expected(tag: &'static [u8]) -> crate::error::Expected {
    crate::error::Expected::ByteTag(tag)
  }
}

pub struct Seq<I, T: ?Sized + 'static> {
  tag: &'static T,
  _marker: PhantomData<fn(&mut I)>,
}

pub fn seq<'a, I, T: ?Sized>(tag: &'static T) -> Seq<I, T>
where
  I: InputSeq<'a, T>, {
  Seq {
    tag,
    _marker: PhantomData,
  }
}

impl<'a, I, T: ?Sized + 'static> Parser<I> for Seq<I, T>
where
  I: InputSeq<'a, T>,
{
  type Error = I::Error;
  type Output = &'static T;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<&'static T, I::Error> {
    let pos = input.offset();
    if input.starts_with(self.tag) {
      input.advance_by(self.tag);
      Ok(self.tag)
    } else {
      Err(Fail::Backtrack(I::Error::from_expected_with_location(
        pos,
        input.line(),
        input.column(),
        I::tag_to_expected(self.tag),
      )))
    }
  }
}
