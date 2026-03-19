//! Parser の演算子オーバーロード。
//!
//! `P` (任意の Parser) を `p.ops()` で `Ops<P, I>` にラップすると、
//! `+`, `-`, `*`, `|`, `!`, 単項`-`, `>>` の演算子が使える。

use core::marker::PhantomData;
use core::ops;

use crate::combinator::flat_map::FlatMap;
use crate::combinator::not::Not;
use crate::combinator::or::Or;
use crate::combinator::peek::Peek;
use crate::combinator::zip::Zip;
use crate::combinator::zip_left::ZipLeft;
use crate::combinator::zip_right::ZipRight;
use crate::error::{ExpectError, MergeError};
use crate::input::Input;
use crate::parser::Parser;

/// 演算子オーバーロードを有効にするラッパー型。
pub struct Ops<P, I: Input>(pub P, PhantomData<fn(&mut I)>);

impl<P, I: Input> Ops<P, I> {
  pub fn new(parser: P) -> Self {
    Ops(parser, PhantomData)
  }
}

impl<I: Input, P: Parser<I>> Parser<I> for Ops<P, I> {
  type Error = P::Error;
  type Output = P::Output;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> crate::fail::PResult<Self::Output, Self::Error> {
    self.0.parse_next(input)
  }
}

// Add: Ops<P1> + Ops<P2> → Ops<Zip<P1, P2>>
impl<I, P1, P2> ops::Add<Ops<P2, I>> for Ops<P1, I>
where
  I: Input,
  P1: Parser<I>,
  P2: Parser<I, Error = P1::Error>,
{
  type Output = Ops<Zip<P1, P2>, I>;

  fn add(self, rhs: Ops<P2, I>) -> Self::Output {
    Ops::new(Zip {
      first: self.0,
      second: rhs.0,
    })
  }
}

// Sub: Ops<P1> - Ops<P2> → Ops<ZipLeft<P1, P2>>
impl<I, P1, P2> ops::Sub<Ops<P2, I>> for Ops<P1, I>
where
  I: Input,
  P1: Parser<I>,
  P2: Parser<I, Error = P1::Error>,
{
  type Output = Ops<ZipLeft<P1, P2>, I>;

  fn sub(self, rhs: Ops<P2, I>) -> Self::Output {
    Ops::new(ZipLeft {
      first: self.0,
      second: rhs.0,
    })
  }
}

// Mul: Ops<P1> * Ops<P2> → Ops<ZipRight<P1, P2>>
impl<I, P1, P2> ops::Mul<Ops<P2, I>> for Ops<P1, I>
where
  I: Input,
  P1: Parser<I>,
  P2: Parser<I, Error = P1::Error>,
{
  type Output = Ops<ZipRight<P1, P2>, I>;

  fn mul(self, rhs: Ops<P2, I>) -> Self::Output {
    Ops::new(ZipRight {
      first: self.0,
      second: rhs.0,
    })
  }
}

// BitOr: Ops<P1> | Ops<P2> → Ops<Or<P1, P2>>
impl<I, P1, P2> ops::BitOr<Ops<P2, I>> for Ops<P1, I>
where
  I: Input,
  P1: Parser<I>,
  P1::Error: MergeError,
  P2: Parser<I, Output = P1::Output, Error = P1::Error>,
{
  type Output = Ops<Or<P1, P2>, I>;

  fn bitor(self, rhs: Ops<P2, I>) -> Self::Output {
    Ops::new(Or {
      left: self.0,
      right: rhs.0,
    })
  }
}

// Not: !Ops<P> → Ops<Not<P>>
impl<I, P> ops::Not for Ops<P, I>
where
  I: Input,
  P: Parser<I>,
  P::Error: ExpectError,
{
  type Output = Ops<Not<P>, I>;

  fn not(self) -> Self::Output {
    Ops::new(Not { parser: self.0 })
  }
}

// Neg: -Ops<P> → Ops<Peek<P>>
impl<I, P> ops::Neg for Ops<P, I>
where
  I: Input,
  P: Parser<I>,
{
  type Output = Ops<Peek<P>, I>;

  fn neg(self) -> Self::Output {
    Ops::new(Peek { parser: self.0 })
  }
}

// Shr: Ops<P> >> F → Ops<FlatMap<P, F>>
impl<I, P, F2, P2> ops::Shr<F2> for Ops<P, I>
where
  I: Input,
  P: Parser<I>,
  P2: Parser<I, Error = P::Error>,
  F2: FnMut(P::Output) -> P2,
{
  type Output = Ops<FlatMap<P, F2>, I>;

  fn shr(self, f: F2) -> Self::Output {
    Ops::new(FlatMap { parser: self.0, f })
  }
}
