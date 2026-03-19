pub use crate::primitive::take::Take;
pub use crate::primitive::take_while0::TakeWhile0;
pub use crate::primitive::take_while1::TakeWhile1;
pub use crate::primitive::take_while_n_m::TakeWhileNM;

use crate::str_input_stream::StrInputStream;

pub fn take<'a>(n: usize) -> Take<StrInputStream<'a>> {
  crate::primitive::take::take(n)
}

pub fn take_while0<'a, F: FnMut(char) -> bool>(f: F) -> TakeWhile0<F, StrInputStream<'a>> {
  crate::primitive::take_while0::take_while0(f)
}

pub fn take_while1<'a, F: FnMut(char) -> bool>(f: F) -> TakeWhile1<F, StrInputStream<'a>> {
  crate::primitive::take_while1::take_while1(f)
}

pub fn take_while_n_m<'a, F: FnMut(char) -> bool>(min: usize, max: usize, f: F) -> TakeWhileNM<F, StrInputStream<'a>> {
  crate::primitive::take_while_n_m::take_while_n_m(min, max, f)
}
