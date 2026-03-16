use oni_comb_parser::error::ParseError;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::parsers::common::pchar;

// fragment = *( pchar / "/" / "?" )
pub fn fragment<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    loop {
      if pchar().attempt().parse_next(input).is_ok() {
        continue;
      }
      if satisfy(|c: char| c == '/' || c == '?')
        .attempt()
        .parse_next(input)
        .is_ok()
      {
        continue;
      }
      break;
    }
    Ok(input.slice_since(cp))
  })
}
