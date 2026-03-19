use oni_comb_parser::error::ParseError;
use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::parsers::common::pchar;

// fragment = *( pchar / "/" / "?" )
pub fn fragment<'a>() -> impl Parser<StrInputStream<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInputStream<'a>| {
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
