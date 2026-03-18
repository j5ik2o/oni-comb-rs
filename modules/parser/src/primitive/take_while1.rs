use core::marker::PhantomData;

use crate::error::{ExpectError, Expected};
use crate::fail::{Fail, PResult};
use crate::input::Input;
use crate::parser::Parser;

pub struct TakeWhile1<F, I: Input>(F, PhantomData<fn(&mut I)>);

pub fn take_while1<I: Input, F: FnMut(I::Token) -> bool>(f: F) -> TakeWhile1<F, I> {
  TakeWhile1(f, PhantomData)
}

impl<I: Input, F> Parser<I> for TakeWhile1<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
    let pos = input.offset();
    let cp = input.checkpoint();
    loop {
      let item_cp = input.checkpoint();
      match input.next_token() {
        Some(t) if (self.0)(t) => {}
        Some(_) => {
          input.reset(item_cp);
          break;
        }
        None => break,
      }
    }
    if input.checkpoint() == cp {
      return Err(Fail::Backtrack(I::Error::from_expected(
        pos,
        Expected::Description("at least one matching token"),
      )));
    }
    Ok(input.slice_since(cp))
  }
}
