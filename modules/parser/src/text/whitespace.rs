use crate::text::take_while0::{take_while0, TakeWhile0};
use crate::text::take_while1::{take_while1, TakeWhile1};

fn is_ws(c: char) -> bool {
  c.is_ascii_whitespace()
}

pub fn whitespace0() -> TakeWhile0<fn(char) -> bool> {
  take_while0(is_ws as fn(char) -> bool)
}

pub fn whitespace1() -> TakeWhile1<fn(char) -> bool> {
  take_while1(is_ws as fn(char) -> bool)
}
