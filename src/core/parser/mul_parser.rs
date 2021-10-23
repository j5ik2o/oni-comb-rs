use crate::core::Parser;
use std::fmt::Debug;
use std::ops::Mul;
use crate::extension::parser::SkipCombinator;

impl<'a, I, A, B> Mul<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Debug + 'a,
  B: Debug + 'a,
{
  type Output = Parser<'a, I, B>;

  fn mul(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.skip_left(rhs)
  }
}
