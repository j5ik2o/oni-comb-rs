use crate::combinator::zip_left::ZipLeft;
use crate::parser::Parser;
use crate::parser_ext::ParserExt;
use crate::primitive::take_while0::TakeWhile0;
use crate::str_input::StrInput;
use crate::text::take_while::take_while0;

fn is_ws(c: char) -> bool {
  c.is_ascii_whitespace()
}

/// `lexeme` の戻り値型。
pub type Lexeme<P> = ZipLeft<P, TakeWhile0<fn(char) -> bool, StrInput<'static>>>;

/// パーサーを実行した後に後続の空白を消費するトークンラッパー。
///
/// ```ignore
/// let lbrace = lexeme(char('{'));
/// let number = lexeme(integer());
/// ```
pub fn lexeme<'a, P>(parser: P) -> ZipLeft<P, TakeWhile0<fn(char) -> bool, StrInput<'a>>>
where
  P: Parser<StrInput<'a>, Error = crate::error::ParseError>, {
  parser.zip_left(take_while0(is_ws as fn(char) -> bool))
}
