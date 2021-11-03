use crate::core::Parser;
use crate::extension::parser::SkipParser;
use std::fmt::Debug;
use std::ops::Mul;

impl<'a, I, A, B> Mul<Parser<'a, I, B>> for Parser<'a, I, A>
where
  A: Clone + Debug + 'a,
  B: Clone + Debug + 'a,
{
  type Output = Parser<'a, I, B>;

  fn mul(self, rhs: Parser<'a, I, B>) -> Self::Output {
    self.skip_left(rhs)
  }
}
