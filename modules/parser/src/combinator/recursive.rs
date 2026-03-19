use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::UnsafeCell;

use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

type DynParser<'a, I, O, E> = dyn Parser<I, Output = O, Error = E> + 'a;

struct RecursiveInner<'a, I: InputStream, O, E> {
  inner: UnsafeCell<Option<Box<DynParser<'a, I, O, E>>>>,
}

/// 再帰パーサー。`recursive()` で構築する。
///
/// 再帰の結び目だけ `Box<dyn Parser>` + `Rc` で型消去し、
/// 非再帰部分は具象型を維持する。
pub struct Recursive<'a, I: InputStream, O, E> {
  shared: Rc<RecursiveInner<'a, I, O, E>>,
}

impl<'a, I: InputStream, O, E> Clone for Recursive<'a, I, O, E> {
  fn clone(&self) -> Self {
    Recursive {
      shared: Rc::clone(&self.shared),
    }
  }
}

impl<'a, I: InputStream, O, E> Parser<I> for Recursive<'a, I, O, E> {
  type Error = E;
  type Output = O;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<O, E> {
    // SAFETY: Recursive は Rc を使うため !Send + !Sync（単一スレッド）。
    // 再帰呼び出しは同一コールスタック上で順次実行され、
    // 外側の parse_next は内側の完了まで中断しているため、
    // 内部パーサーの状態に対する並行アクセスは発生しない。
    unsafe {
      (*self.shared.inner.get())
        .as_mut()
        .expect("recursive parser not initialized")
        .parse_next(input)
    }
  }
}

/// 再帰パーサーを構築する。
///
/// クロージャ `f` は再帰参照（`Recursive`）を受け取り、パーサーを組み立てて返す。
/// 返されたパーサーは `Box<dyn Parser>` として内部に格納される。
///
/// ```ignore
/// let expr = recursive(|expr| {
///     let atom = integer().or(between(tag("("), expr, tag(")")));
///     let term = atom.chainl1(mul_op());
///     term.chainl1(add_op())
/// });
/// ```
pub fn recursive<'a, I, O, E, F, P>(f: F) -> Recursive<'a, I, O, E>
where
  I: InputStream,
  F: FnOnce(Recursive<'a, I, O, E>) -> P,
  P: Parser<I, Output = O, Error = E> + 'a, {
  let rec = Recursive {
    shared: Rc::new(RecursiveInner {
      inner: UnsafeCell::new(None),
    }),
  };
  let parser = f(rec.clone());
  unsafe {
    *rec.shared.inner.get() = Some(Box::new(parser));
  }
  rec
}
