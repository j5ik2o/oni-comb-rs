pub use crate::byte_input::ByteInput;
pub use crate::parser::Parser;
pub use crate::parser_ext::ParserExt;
pub use crate::str_input::StrInput;

// StrInput 固定のプリミティブパーサーラッパー
// ジェネリック版は crate::primitive から直接インポート可能

pub fn eof<'a>() -> crate::primitive::eof::Eof<StrInput<'a>> {
  crate::primitive::eof::eof()
}

pub fn satisfy<'a, F: FnMut(char) -> bool>(f: F) -> crate::primitive::satisfy::Satisfy<F, StrInput<'a>> {
  crate::primitive::satisfy::satisfy(f)
}

pub fn take<'a>(n: usize) -> crate::primitive::take::Take<StrInput<'a>> {
  crate::primitive::take::take(n)
}

pub fn take_while0<'a, F: FnMut(char) -> bool>(f: F) -> crate::primitive::take_while0::TakeWhile0<F, StrInput<'a>> {
  crate::primitive::take_while0::take_while0(f)
}

pub fn take_while1<'a, F: FnMut(char) -> bool>(f: F) -> crate::primitive::take_while1::TakeWhile1<F, StrInput<'a>> {
  crate::primitive::take_while1::take_while1(f)
}

pub fn take_while_n_m<'a, F: FnMut(char) -> bool>(
  min: usize,
  max: usize,
  f: F,
) -> crate::primitive::take_while_n_m::TakeWhileNM<F, StrInput<'a>> {
  crate::primitive::take_while_n_m::take_while_n_m(min, max, f)
}

pub fn take_till0<'a, F: FnMut(char) -> bool>(f: F) -> crate::primitive::take_till0::TakeTill0<F, StrInput<'a>> {
  crate::primitive::take_till0::take_till0(f)
}

pub fn take_till1<'a, F: FnMut(char) -> bool>(f: F) -> crate::primitive::take_till1::TakeTill1<F, StrInput<'a>> {
  crate::primitive::take_till1::take_till1(f)
}

// text 専用パーサー
pub use crate::text::char::char;
pub use crate::text::escaped::escaped;
pub use crate::text::identifier::identifier;
pub use crate::text::integer::integer;
pub use crate::text::lexeme::lexeme;
pub use crate::text::quoted_string::quoted_string;
pub use crate::text::quoted_string_cow::quoted_string_cow;
pub use crate::text::tag::tag;
pub use crate::text::whitespace::{whitespace0, whitespace1};

pub use crate::combinator::fn_parser::fn_parser;
pub use crate::combinator::recursive::recursive;

#[cfg(feature = "regex")]
pub use crate::text::regex::{regex, RegexBuildError};

/// left, parser, right を順に実行し、parser の値だけを返す。
pub fn between<I, L, P, R>(
  left: L,
  parser: P,
  right: R,
) -> crate::combinator::zip_right::ZipRight<L, crate::combinator::zip_left::ZipLeft<P, R>>
where
  I: crate::input::Input,
  L: crate::parser::Parser<I>,
  P: crate::parser::Parser<I, Error = L::Error>,
  R: crate::parser::Parser<I, Error = L::Error>, {
  left.zip_right(parser.zip_left(right))
}
