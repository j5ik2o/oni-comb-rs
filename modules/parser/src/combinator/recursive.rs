use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::UnsafeCell;

use crate::fail::PResult;
use crate::parser::Parser;
use crate::str_input::StrInput;

struct RecursiveInner<'a, O, E> {
  inner: UnsafeCell<Option<Box<dyn Parser<StrInput<'a>, Output = O, Error = E> + 'a>>>,
}

/// 再帰パーサー。`recursive()` で構築する。
///
/// 再帰の結び目だけ `Box<dyn Parser>` + `Rc` で型消去し、
/// 非再帰部分は具象型を維持する。
pub struct Recursive<'a, O, E> {
  shared: Rc<RecursiveInner<'a, O, E>>,
}

impl<'a, O, E> Clone for Recursive<'a, O, E> {
  fn clone(&self) -> Self {
    Recursive {
      shared: Rc::clone(&self.shared),
    }
  }
}

impl<'a, O, E> Parser<StrInput<'a>> for Recursive<'a, O, E> {
  type Error = E;
  type Output = O;

  #[inline]
  fn parse_next(&mut self, input: &mut StrInput<'a>) -> PResult<O, E> {
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
pub fn recursive<'a, O, E, F, P>(f: F) -> Recursive<'a, O, E>
where
  F: FnOnce(Recursive<'a, O, E>) -> P,
  P: Parser<StrInput<'a>, Output = O, Error = E> + 'a, {
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
