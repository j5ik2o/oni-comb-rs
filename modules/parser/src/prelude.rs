pub use crate::byte_input_stream::ByteInputStream;
pub use crate::parser::Parser;
pub use crate::parser_ext::ParserExt;
pub use crate::str_input_stream::StrInputStream;

// ジェネリックプリミティブパーサー（StrInput / ByteInput 両対応）
pub use crate::primitive::any::any;
pub use crate::primitive::eof::eof;
pub use crate::primitive::none_of::none_of;
pub use crate::primitive::not_a::not_a;
pub use crate::primitive::one_of::one_of;
pub use crate::primitive::satisfy::satisfy;
pub use crate::primitive::seq::seq;
pub use crate::primitive::sym::sym;
pub use crate::primitive::take::take;
pub use crate::primitive::take_till0::take_till0;
pub use crate::primitive::take_till1::take_till1;
pub use crate::primitive::take_while0::take_while0;
pub use crate::primitive::take_while1::take_while1;
pub use crate::primitive::take_while_n_m::take_while_n_m;

// text 専用パーサー（StrInput のみ）
pub use crate::text::char::char;
pub use crate::text::float::float;
pub use crate::text::identifier::identifier;
pub use crate::text::integer::integer;
pub use crate::text::lexeme::lexeme;
pub use crate::text::tag::tag;
pub use crate::text::whitespace::{whitespace0, whitespace1};

#[cfg(feature = "alloc")]
pub use crate::text::escaped::escaped;
#[cfg(feature = "alloc")]
pub use crate::text::quoted_string::quoted_string;

pub use crate::combinator::fn_parser::fn_parser;
pub use crate::combinator::guard::guard;
pub use crate::combinator::position::position;
#[cfg(feature = "alloc")]
pub use crate::combinator::predictive_choice::predictive_choice;
#[cfg(feature = "alloc")]
pub use crate::combinator::recursive::recursive;
pub use crate::ops::Ops;

#[cfg(feature = "regex")]
pub use crate::text::regex::{regex, RegexBuildError};

/// left, parser, right を順に実行し、parser の値だけを返す。
pub fn between<I, L, P, R>(
  left: L,
  parser: P,
  right: R,
) -> crate::combinator::zip_right::ZipRight<L, crate::combinator::zip_left::ZipLeft<P, R>>
where
  I: crate::input_stream::InputStream,
  L: crate::parser::Parser<I>,
  P: crate::parser::Parser<I, Error = L::Error>,
  R: crate::parser::Parser<I, Error = L::Error>, {
  left.zip_right(parser.zip_left(right))
}
