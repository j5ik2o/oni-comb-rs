use oni_comb_parser::error::ParseError;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::models::path::Path;
use crate::models::uri::Uri;
use crate::parsers::authority::authority;
use crate::parsers::fragment::fragment;
use crate::parsers::path::{path_abempty, path_absolute, path_noscheme, path_rootless};
use crate::parsers::query::query;
use crate::parsers::scheme::scheme;

// hier-part = "//" authority path-abempty / path-absolute / path-rootless / path-empty
// URI = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
fn uri_parser<'a>() -> impl Parser<StrInputStream<'a>, Output = Uri<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInputStream<'a>| {
    // scheme ":" (optional: attempt to parse scheme + colon)
    let s = fn_parser(|input: &mut StrInputStream<'a>| {
      let s = scheme().parse_next(input)?;
      tag(":").parse_next(input)?;
      Ok(s)
    })
    .attempt()
    .parse_next(input)
    .ok();

    // hier-part
    let (auth, path) = if tag("//").attempt().parse_next(input).is_ok() {
      let a = authority().parse_next(input)?;
      let p = path_abempty().parse_next(input)?;
      (Some(a), p)
    } else if let Ok(p) = path_absolute().attempt().parse_next(input) {
      (None, p)
    } else if s.is_some() {
      // With scheme: path-rootless allowed
      if let Ok(p) = path_rootless().attempt().parse_next(input) {
        (None, p)
      } else {
        (None, Path::Empty)
      }
    } else {
      // Without scheme: path-noscheme
      if let Ok(p) = path_noscheme().attempt().parse_next(input) {
        (None, p)
      } else {
        (None, Path::Empty)
      }
    };

    // [ "?" query ]
    let q = if tag("?").attempt().parse_next(input).is_ok() {
      Some(query().parse_next(input)?)
    } else {
      None
    };

    // [ "#" fragment ]
    let f = if tag("#").attempt().parse_next(input).is_ok() {
      Some(fragment().parse_next(input)?)
    } else {
      None
    };

    Ok(Uri {
      scheme: s,
      authority: auth,
      path,
      query: q,
      fragment: f,
    })
  })
}

pub fn parse_uri(input: &str) -> Result<Uri<'_>, String> {
  let mut parser = uri_parser().zip_left(eof());
  let mut str_input = StrInputStream::new(input);
  parser.parse_next(&mut str_input).map_err(|e| format!("{:?}", e))
}
