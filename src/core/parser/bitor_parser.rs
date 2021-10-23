use crate::core::Parser;
use std::fmt::Debug;
use std::ops::BitOr;
use crate::extension::parser::BasicCombinator;

impl<'a, I, A> BitOr for Parser<'a, I, A>
where
  A: Debug + 'a,
{
  type Output = Self;

  fn bitor(self, rhs: Parser<'a, I, A>) -> Self::Output {
    self.or(rhs)
  }
}
