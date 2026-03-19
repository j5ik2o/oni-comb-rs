use core::marker::PhantomData;

use crate::fail::PResult;
use crate::input_stream::InputStream;
use crate::parser::Parser;

pub struct TakeWhile0<F, I: InputStream>(F, PhantomData<fn(&mut I)>);

pub fn take_while0<I: InputStream, F: FnMut(I::Token) -> bool>(f: F) -> TakeWhile0<F, I> {
  TakeWhile0(f, PhantomData)
}

impl<I: InputStream, F> Parser<I> for TakeWhile0<F, I>
where
  F: FnMut(I::Token) -> bool,
{
  type Error = I::Error;
  type Output = I::Slice;

  #[inline]
  fn parse_next(&mut self, input: &mut I) -> PResult<I::Slice, I::Error> {
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
    Ok(input.slice_since(cp))
  }
}
