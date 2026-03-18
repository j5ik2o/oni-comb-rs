use crate::combinator::attempt::Attempt;
use crate::combinator::chainl1::ChainL1;
#[cfg(feature = "alloc")]
use crate::combinator::chainr1::ChainR1;
use crate::combinator::collect::Collect;
use crate::combinator::context::Context;
use crate::combinator::cut::Cut;
use crate::combinator::discard::Discard;
use crate::combinator::flat_map::FlatMap;
#[cfg(feature = "alloc")]
use crate::combinator::many::Many;
#[cfg(feature = "alloc")]
use crate::combinator::many1::Many1;
use crate::combinator::many1_fold::Many1Fold;
use crate::combinator::many_fold::ManyFold;
use crate::combinator::map::Map;
use crate::combinator::map_res::MapRes;
use crate::combinator::not::Not;
use crate::combinator::optional::Optional;
use crate::combinator::or::Or;
use crate::combinator::peek::Peek;
#[cfg(feature = "alloc")]
use crate::combinator::repeat::Repeat;
#[cfg(feature = "alloc")]
use crate::combinator::sep_by::{SepBy0, SepBy1};
use crate::combinator::sep_by_fold::{SepByFold0, SepByFold1};
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

  #[cfg(feature = "alloc")]
  fn many0(self) -> Many<Self> {
    Many { parser: self }
  }

  #[cfg(feature = "alloc")]
  fn many1(self) -> Many1<Self> {
    Many1 { parser: self }
  }

  /// ゼロ個以上の要素を畳み込む（ゼロアロケーション）。
  fn many0_fold<B, F, Acc>(self, init: B, f: F) -> ManyFold<Self, B, F>
  where
    B: FnMut() -> Acc,
    F: FnMut(Acc, Self::Output) -> Acc, {
    ManyFold { parser: self, init, f }
  }

  /// 1個以上の要素を畳み込む（ゼロアロケーション）。
  fn many1_fold<B, F, Acc>(self, init: B, f: F) -> Many1Fold<Self, B, F>
  where
    B: FnMut() -> Acc,
    F: FnMut(Acc, Self::Output) -> Acc, {
    Many1Fold { parser: self, init, f }
  }

  /// ゼロ個以上の要素をユーザー指定のコンテナに収集する。
  fn many0_into<C>(self, container: C) -> ManyFold<Self, impl FnMut() -> C, impl FnMut(C, Self::Output) -> C>
  where
    C: Extend<Self::Output> + Clone, {
    self.many0_fold(
      move || container.clone(),
      |mut acc, item| {
        acc.extend(core::iter::once(item));
        acc
      },
    )
  }

  /// 1個以上の要素をユーザー指定のコンテナに収集する。
  fn many1_into<C>(self, container: C) -> Many1Fold<Self, impl FnMut() -> C, impl FnMut(C, Self::Output) -> C>
  where
    C: Extend<Self::Output> + Clone, {
    self.many1_fold(
      move || container.clone(),
      |mut acc, item| {
        acc.extend(core::iter::once(item));
        acc
      },
    )
  }

  /// エラーにコンテキストラベルを付与する。
  fn context(self, label: &'static str) -> Context<Self>
  where
    Self::Error: crate::error::ContextError, {
    Context { parser: self, label }
  }

  /// 区切り付き 0 個以上の繰り返し。
  #[cfg(feature = "alloc")]
  fn sep_by0<S>(self, sep: S) -> SepBy0<Self, S>
  where
    S: Parser<I, Error = Self::Error>, {
    SepBy0 { parser: self, sep }
  }

  /// 区切り付き 1 個以上の繰り返し。
  #[cfg(feature = "alloc")]
  fn sep_by1<S>(self, sep: S) -> SepBy1<Self, S>
  where
    S: Parser<I, Error = Self::Error>, {
    SepBy1 { parser: self, sep }
  }

  /// 区切り付き 0 個以上の要素を畳み込む（ゼロアロケーション）。
  fn sep_by0_fold<S, B, F, Acc>(self, sep: S, init: B, f: F) -> SepByFold0<Self, S, B, F>
  where
    S: Parser<I, Error = Self::Error>,
    B: FnMut() -> Acc,
    F: FnMut(Acc, Self::Output) -> Acc, {
    SepByFold0 {
      parser: self,
      sep,
      init,
      f,
    }
  }

  /// 区切り付き 1 個以上の要素を畳み込む（ゼロアロケーション）。
  fn sep_by1_fold<S, B, F, Acc>(self, sep: S, init: B, f: F) -> SepByFold1<Self, S, B, F>
  where
    S: Parser<I, Error = Self::Error>,
    B: FnMut() -> Acc,
    F: FnMut(Acc, Self::Output) -> Acc, {
    SepByFold1 {
      parser: self,
      sep,
      init,
      f,
    }
  }

  /// 区切り付き 0 個以上の要素をユーザー指定のコンテナに収集する。
  fn sep_by0_into<S, C>(
    self,
    sep: S,
    container: C,
  ) -> SepByFold0<Self, S, impl FnMut() -> C, impl FnMut(C, Self::Output) -> C>
  where
    S: Parser<I, Error = Self::Error>,
    C: Extend<Self::Output> + Clone, {
    self.sep_by0_fold(
      sep,
      move || container.clone(),
      |mut acc, item| {
        acc.extend(core::iter::once(item));
        acc
      },
    )
  }

  /// 区切り付き 1 個以上の要素をユーザー指定のコンテナに収集する。
  fn sep_by1_into<S, C>(
    self,
    sep: S,
    container: C,
  ) -> SepByFold1<Self, S, impl FnMut() -> C, impl FnMut(C, Self::Output) -> C>
  where
    S: Parser<I, Error = Self::Error>,
    C: Extend<Self::Output> + Clone, {
    self.sep_by1_fold(
      sep,
      move || container.clone(),
      |mut acc, item| {
        acc.extend(core::iter::once(item));
        acc
      },
    )
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
  #[cfg(feature = "alloc")]
  fn chainr1<Op, F>(self, op: Op) -> ChainR1<Self, Op>
  where
    Op: Parser<I, Output = F, Error = Self::Error>,
    F: FnMut(Self::Output, Self::Output) -> Self::Output, {
    ChainR1 {
      operand: self,
      operator: op,
    }
  }

  /// パーサーの結果を失敗しうる関数で変換する。
  fn map_res<F, O2, E2>(self, f: F, label: &'static str) -> MapRes<Self, F>
  where
    Self::Error: crate::error::ExpectError,
    F: FnMut(Self::Output) -> Result<O2, E2>, {
    MapRes { parser: self, f, label }
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

  /// 否定先読み。内部パーサーが失敗すれば Ok(()), 成功すれば Backtrack。入力を消費しない。
  fn not(self) -> Not<Self> {
    Not { parser: self }
  }

  /// 正先読み。成功時は出力を返すが入力を消費しない。
  fn peek(self) -> Peek<Self> {
    Peek { parser: self }
  }

  /// 回数指定の繰り返し。Range 引数 (0.., 1.., 2..=4 等) をサポート。
  #[cfg(feature = "alloc")]
  fn repeat<R: crate::combinator::repeat::RepeatRange>(self, range: R) -> Repeat<Self> {
    Repeat {
      parser: self,
      min: range.min_count(),
      max: range.max_count(),
    }
  }

  /// パース範囲の入力 Slice を返す（出力を捨てる）。
  fn collect(self) -> Collect<Self> {
    Collect { parser: self }
  }

  /// 出力を () に変換する。
  fn discard(self) -> Discard<Self> {
    Discard { parser: self }
  }

  /// 演算子オーバーロードを有効にするラッパーで包む。
  /// `sym('a').ops() + sym('b').ops()` のように使う。
  fn ops(self) -> crate::ops::Ops<Self, I> {
    crate::ops::Ops::new(self)
  }
}

impl<I: Input, P: Parser<I> + Sized> ParserExt<I> for P {}
