use crate::core::ParserRunner;
use std::fmt::Debug;

pub trait OperatorParser<'a>: ParserRunner<'a> {
  fn and_then<B>(self, other: Self::P<'a, Self::Input, B>) -> Self::P<'a, Self::Input, (Self::Output, B)>
  where
    Self::Output: Clone + Debug + 'a,
    B: Clone + Debug + 'a;

  fn or(self, other: Self::P<'a, Self::Input, Self::Output>) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a;

  fn exists(self) -> Self::P<'a, Self::Input, bool>
  where
    Self::Output: Debug + 'a;

  fn not(self) -> Self::P<'a, Self::Input, ()>
  where
    Self::Output: Debug + 'a;

  fn opt(self) -> Self::P<'a, Self::Input, Option<Self::Output>>
  where
    Self::Output: Clone + Debug + 'a;

  fn attempt(self) -> Self::P<'a, Self::Input, Self::Output>
  where
    Self::Output: Debug + 'a;

  fn scan_right1<BOP>(self, op: Self::P<'a, Self::Input, BOP>) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;

  fn chain_right0<BOP>(
    self,
    op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;

  fn chain_left0<BOP>(
    self,
    op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;

  fn chain_right1<BOP>(self, op: Self::P<'a, Self::Input, BOP>) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;

  fn chain_left1<BOP>(self, op: Self::P<'a, Self::Input, BOP>) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;

  fn rest_right1<BOP>(
    self,
    op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;

  fn rest_left1<BOP>(
    self,
    op: Self::P<'a, Self::Input, BOP>,
    x: Self::Output,
  ) -> Self::P<'a, Self::Input, Self::Output>
  where
    BOP: Fn(Self::Output, Self::Output) -> Self::Output + 'a,
    Self::Output: Clone + Debug + 'a;
}
