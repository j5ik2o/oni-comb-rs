use crate::combinator::attempt::Attempt;
use crate::combinator::chainl1::ChainL1;
use crate::combinator::chainr1::ChainR1;
use crate::combinator::context::Context;
use crate::combinator::cut::Cut;
use crate::combinator::flat_map::FlatMap;
use crate::combinator::many::Many;
use crate::combinator::many1::Many1;
use crate::combinator::map::Map;
use crate::combinator::optional::Optional;
use crate::combinator::or::Or;
use crate::combinator::sep_by::{SepBy0, SepBy1};
use crate::combinator::zip::Zip;
use crate::combinator::zip_left::ZipLeft;
use crate::combinator::zip_right::ZipRight;
use crate::input::Input;
use crate::parser::Parser;

pub trait ParserExt<I: Input>: Parser<I> + Sized {
  fn map<F, O2>(self, f: F) -> Map<Self, F>
  where
    F: FnMut(Self::Output) -> O2, {
    Map { parser: self, f }
  }

  fn zip<P2>(self, rhs: P2) -> Zip<Self, P2>
  where
    P2: Parser<I, Error = Self::Error>, {
    Zip {
      first: self,
      second: rhs,
    }
  }

  /// 両方実行し、左（self）の値だけを返す（= terminated）。
  fn zip_left<P2>(self, rhs: P2) -> ZipLeft<Self, P2>
  where
    P2: Parser<I, Error = Self::Error>, {
    ZipLeft {
      first: self,
      second: rhs,
    }
  }

  /// 両方実行し、右（rhs）の値だけを返す（= preceded）。
  fn zip_right<P2>(self, rhs: P2) -> ZipRight<Self, P2>
  where
    P2: Parser<I, Error = Self::Error>, {
    ZipRight {
      first: self,
      second: rhs,
    }
  }

  fn or<P2>(self, rhs: P2) -> Or<Self, P2>
  where
    Self::Error: crate::error::MergeError,
    P2: Parser<I, Output = Self::Output, Error = Self::Error>, {
    Or { left: self, right: rhs }
  }

  fn attempt(self) -> Attempt<Self> {
    Attempt { parser: self }
  }

  fn cut(self) -> Cut<Self> {
    Cut { parser: self }
  }

  fn optional(self) -> Optional<Self> {
    Optional { parser: self }
  }

  fn many0(self) -> Many<Self> {
    Many { parser: self }
  }

  fn many1(self) -> Many1<Self> {
    Many1 { parser: self }
  }

  /// エラーにコンテキストラベルを付与する。
  fn context(self, label: &'static str) -> Context<Self>
  where
    Self::Error: crate::error::ContextError, {
    Context { parser: self, label }
  }

  /// 区切り付き 0 個以上の繰り返し。
  fn sep_by0<S>(self, sep: S) -> SepBy0<Self, S>
  where
    S: Parser<I, Error = Self::Error>, {
    SepBy0 { parser: self, sep }
  }

  /// 区切り付き 1 個以上の繰り返し。
  fn sep_by1<S>(self, sep: S) -> SepBy1<Self, S>
  where
    S: Parser<I, Error = Self::Error>, {
    SepBy1 { parser: self, sep }
  }

  /// 左結合の二項演算子チェーン。operand (op operand)* を左から畳む。
  fn chainl1<Op, F>(self, op: Op) -> ChainL1<Self, Op>
  where
    Op: Parser<I, Output = F, Error = Self::Error>,
    F: FnMut(Self::Output, Self::Output) -> Self::Output, {
    ChainL1 {
      operand: self,
      operator: op,
    }
  }

  /// 右結合の二項演算子チェーン。operand (op operand)* を右から畳む。
  fn chainr1<Op, F>(self, op: Op) -> ChainR1<Self, Op>
  where
    Op: Parser<I, Output = F, Error = Self::Error>,
    F: FnMut(Self::Output, Self::Output) -> Self::Output, {
    ChainR1 {
      operand: self,
      operator: op,
    }
  }

  fn flat_map<F, P2>(self, f: F) -> FlatMap<Self, F>
  where
    P2: Parser<I, Error = Self::Error>,
    F: FnMut(Self::Output) -> P2, {
    FlatMap { parser: self, f }
  }

  fn and_then<F, P2>(self, f: F) -> FlatMap<Self, F>
  where
    P2: Parser<I, Error = Self::Error>,
    F: FnMut(Self::Output) -> P2, {
    FlatMap { parser: self, f }
  }
}

impl<I: Input, P: Parser<I> + Sized> ParserExt<I> for P {}
