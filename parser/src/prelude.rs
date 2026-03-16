pub use crate::parser::Parser;
pub use crate::parser_ext::ParserExt;
pub use crate::str_input::StrInput;

pub use crate::text::char::char;
pub use crate::text::eof::eof;
pub use crate::text::escaped::escaped;
pub use crate::text::identifier::identifier;
pub use crate::text::integer::integer;
pub use crate::text::lexeme::lexeme;
pub use crate::text::quoted_string::quoted_string;
pub use crate::text::satisfy::satisfy;
pub use crate::text::tag::tag;
pub use crate::text::take_while::{take_while0, take_while1};
pub use crate::text::whitespace::{whitespace0, whitespace1};

pub use crate::combinator::recursive::recursive;

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
    R: crate::parser::Parser<I, Error = L::Error>,
{
    left.zip_right(parser.zip_left(right))
}
