use crate::core::Parser;
use crate::extension::parser::SkipParser;
use std::fmt::Debug;
use std::ops::Sub;

impl<'a, I, A, B> Sub<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a,
{
  type Output = Self;

  fn sub(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.skip_right(rhs)
  }
}
