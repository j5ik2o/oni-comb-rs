use alloc::vec::Vec;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct Repeat<P> {
  pub(crate) parser: P,
  pub(crate) min: usize,
  pub(crate) max: Option<usize>, // None = unlimited
}

impl<I, P> Parser<I> for Repeat<P>
where
  I: Input,
  P: Parser<I>,
  P::Error: crate::error::ExpectError,
{
  type Error = P::Error;
  type Output = Vec<P::Output>;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<Self::Output, Self::Error> {
    let mut items = Vec::new();
    let start_cp = input.checkpoint();
    let start_pos = input.offset();

    loop {
      // Check max limit
      if let Some(max) = self.max {
        if items.len() >= max {
          break;
        }
      }

      let cp = input.checkpoint();
      match self.parser.parse_next(input) {
        Ok(v) => {
          if input.checkpoint() == cp {
            return Err(Fail::ZeroProgress);
          }
          items.push(v);
        }
        Err(Fail::Backtrack(_)) => {
          input.reset(cp);
          break;
        }
        Err(Fail::Cut(e)) => return Err(Fail::Cut(e)),
        Err(Fail::Incomplete) => return Err(Fail::Incomplete),
        Err(Fail::ZeroProgress) => return Err(Fail::ZeroProgress),
      }
    }

    if items.len() < self.min {
      input.reset(start_cp);
      Err(Fail::Backtrack(P::Error::from_expected(
        start_pos,
        Expected::Description("repeat minimum"),
      )))
    } else {
      Ok(items)
    }
  }
}

/// RangeArgument を使って min/max を抽出するためのトレイト。
pub trait RepeatRange {
  fn min_count(&self) -> usize;
  fn max_count(&self) -> Option<usize>;
}

impl RepeatRange for core::ops::RangeFull {
  fn min_count(&self) -> usize {
    0
  }

  fn max_count(&self) -> Option<usize> {
    None
  }
}

impl RepeatRange for core::ops::RangeFrom<usize> {
  fn min_count(&self) -> usize {
    self.start
  }

  fn max_count(&self) -> Option<usize> {
    None
  }
}

impl RepeatRange for core::ops::Range<usize> {
  fn min_count(&self) -> usize {
    self.start
  }

  fn max_count(&self) -> Option<usize> {
    Some(if self.end > 0 { self.end - 1 } else { 0 })
  }
}

impl RepeatRange for core::ops::RangeInclusive<usize> {
  fn min_count(&self) -> usize {
    *self.start()
  }

  fn max_count(&self) -> Option<usize> {
    Some(*self.end())
  }
}

impl RepeatRange for core::ops::RangeTo<usize> {
  fn min_count(&self) -> usize {
    0
  }

  fn max_count(&self) -> Option<usize> {
    Some(if self.end > 0 { self.end - 1 } else { 0 })
  }
}

impl RepeatRange for core::ops::RangeToInclusive<usize> {
  fn min_count(&self) -> usize {
    0
  }

  fn max_count(&self) -> Option<usize> {
    Some(self.end)
  }
}

impl RepeatRange for usize {
  fn min_count(&self) -> usize {
    *self
  }

  fn max_count(&self) -> Option<usize> {
    Some(*self)
  }
}
