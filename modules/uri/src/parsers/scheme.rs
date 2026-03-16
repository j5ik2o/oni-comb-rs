use oni_comb_parser::error::ParseError;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

// scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
pub fn scheme<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    let mut head = satisfy(|c: char| c.is_ascii_alphabetic());
    head.parse_next(input)?;
    let mut tail = take_while0(|c: char| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
    tail.parse_next(input)?;
    Ok(input.slice_since(cp))
  })
}
