use crate::primitive::take_while0::TakeWhile0;
use crate::primitive::take_while1::TakeWhile1;
use crate::str_input_stream::StrInputStream;
use crate::text::take_while::{take_while0, take_while1};

fn is_ws(c: char) -> bool {
  c.is_ascii_whitespace()
}

pub fn whitespace0<'a>() -> TakeWhile0<fn(char) -> bool, StrInputStream<'a>> {
  take_while0(is_ws as fn(char) -> bool)
}

pub fn whitespace1<'a>() -> TakeWhile1<fn(char) -> bool, StrInputStream<'a>> {
  take_while1(is_ws as fn(char) -> bool)
}
