use oni_comb_parser::error::ParseError;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::*;

use crate::models::query::Query;
use crate::parsers::common::pchar;

// query = *( pchar / "/" / "?" )
// We consume the entire query string (including '&' and '=') as raw,
// then decompose into key-value pairs afterwards.
pub fn query<'a>() -> impl Parser<StrInput<'a>, Output = Query<'a>, Error = ParseError> {
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
    let raw = input.slice_since(cp);

    let params = if raw.is_empty() {
      Vec::new()
    } else {
      raw
        .split('&')
        .map(|part| match part.split_once('=') {
          Some((k, v)) => (k, Some(v)),
          None => (part, None),
        })
        .collect()
    };

    Ok(Query::new(raw, params))
  })
}
