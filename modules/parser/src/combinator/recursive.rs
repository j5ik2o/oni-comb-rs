use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

type ParseThunk<I, O, E> = unsafe fn(*mut (), &mut I) -> PResult<O, E>;
type DropThunk = unsafe fn(*mut ());

struct RecursiveRuntime<I: InputStream, O, E> {
  data: *mut (),
  parse_fn: ParseThunk<I, O, E>,
  drop_fn: DropThunk,
}

unsafe fn uninitialized_parse<I: InputStream, O, E>(_: *mut (), _: &mut I) -> PResult<O, E> {
  panic!("recursive parser not initialized")
}

unsafe fn no_drop(_: *mut ()) {}

unsafe fn parse_boxed<I: InputStream, O, E, P>(data: *mut (), input: &mut I) -> PResult<O, E>
where
  P: Parser<I, Output = O, Error = E>, {
  // SAFETY: `data` points to the boxed parser stored in `RecursiveInner`.
  // Re-entrant recursive calls intentionally go through the same parser object,
  // matching the previous `UnsafeCell<Box<dyn Parser>>` design.
  unsafe { (&mut *(data as *mut P)).parse_next(input) }
}

unsafe fn drop_boxed<P>(data: *mut ()) {
  if !data.is_null() {
    // SAFETY: `data` was created by `Box::into_raw(Box<P>)` in `initialize`.
    unsafe {
      drop(Box::from_raw(data as *mut P));
    }
  }
}

struct RecursiveOwner<I: InputStream, O, E> {
  runtime: UnsafeCell<RecursiveRuntime<I, O, E>>,
}

impl<I: InputStream, O, E> RecursiveOwner<I, O, E> {
  fn new() -> Self {
    Self {
      runtime: UnsafeCell::new(RecursiveRuntime {
        data: core::ptr::null_mut(),
        parse_fn: uninitialized_parse::<I, O, E>,
        drop_fn: no_drop,
      }),
    }
  }

  unsafe fn initialize<P>(&self, parser: P)
  where
    P: Parser<I, Output = O, Error = E>, {
    let runtime = unsafe { &mut *self.runtime.get() };
    let boxed = Box::new(parser);
    runtime.data = Box::into_raw(boxed) as *mut ();
    runtime.parse_fn = parse_boxed::<I, O, E, P>;
    runtime.drop_fn = drop_boxed::<P>;
  }
}

impl<I: InputStream, O, E> Drop for RecursiveOwner<I, O, E> {
  fn drop(&mut self) {
    let runtime = self.runtime.get_mut();
    unsafe {
      (runtime.drop_fn)(runtime.data);
    }
    runtime.data = core::ptr::null_mut();
    runtime.drop_fn = no_drop;
    runtime.parse_fn = uninitialized_parse::<I, O, E>;
  }
}

/// 再帰パーサー。`recursive()` で構築する。
///
/// root parser だけがランタイム allocation を所有し、
/// parser graph 内の自己参照は non-owning handle として保持される。
/// steady-state では `Box<dyn Parser>` を経由せず thunk dispatch で具象 parser に到達する。
pub struct Recursive<'a, I: InputStream, O, E> {
  owner: Option<Rc<RecursiveOwner<I, O, E>>>,
  runtime_ptr: NonNull<RecursiveRuntime<I, O, E>>,
  marker: PhantomData<&'a ()>,
}

impl<'a, I: InputStream, O, E> Recursive<'a, I, O, E> {
  fn new_owner(owner: Rc<RecursiveOwner<I, O, E>>, runtime_ptr: NonNull<RecursiveRuntime<I, O, E>>) -> Self {
    Self {
      runtime_ptr,
      owner: Some(owner),
      marker: PhantomData,
    }
  }

  fn new_ref(runtime_ptr: NonNull<RecursiveRuntime<I, O, E>>) -> Self {
    Self {
      owner: None,
      runtime_ptr,
      marker: PhantomData,
    }
  }
}

impl<'a, I: InputStream, O, E> Clone for Recursive<'a, I, O, E> {
  fn clone(&self) -> Self {
    Self {
      owner: self.owner.as_ref().map(Rc::clone),
      runtime_ptr: self.runtime_ptr,
      marker: PhantomData,
    }
  }
}

impl<'a, I: InputStream, O, E> Parser<I> for Recursive<'a, I, O, E> {
  type Error = E;
  type Output = O;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<O, E> {
    // SAFETY: `runtime_ptr` points to the runtime slot owned by the root `Recursive`.
    // Graph-internal references are non-owning and only used while some root owner
    // is alive during parsing.
    let runtime = unsafe { self.runtime_ptr.as_ref() };
    unsafe { (runtime.parse_fn)(runtime.data, input) }
  }
}

/// 再帰パーサーを構築する。
///
/// クロージャ `f` は non-owning な再帰参照（`Recursive`）を受け取り、
/// 具象 parser graph を組み立てて返す。返却される root parser のみが
/// ランタイム allocation を所有し、graph 内の再帰参照は所有権を持たない。
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
  let owner = Rc::new(RecursiveOwner::new());
  let runtime_ptr = unsafe { NonNull::new_unchecked(owner.runtime.get()) };
  let self_ref = Recursive::new_ref(runtime_ptr);
  let parser = f(self_ref);
  unsafe {
    owner.initialize(parser);
  }
  Recursive::new_owner(owner, runtime_ptr)
}
