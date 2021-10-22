use crate::core::Parser;
use crate::extension::SkipCombinator;
use std::fmt::Debug;
use std::ops::Sub;

impl<'a, I, A, B> Sub<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Debug + 'a,
  B: Debug + 'a,
{
  type Output = Self;

  fn sub(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.skip_right(rhs)
  }
}
