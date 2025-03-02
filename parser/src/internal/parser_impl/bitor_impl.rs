use crate::core::Parser;
use crate::extension::parser::OperatorParser;
use std::fmt::Debug;
use std::ops::BitOr;

impl<'a, I: Clone, A> BitOr for Parser<'a, I, A>
where
  A: Debug + 'a,
{
  type Output = Self;

  fn bitor(self, rhs: Parser<'a, I, A>) -> Self::Output {
    self.or(rhs)
  }
}
