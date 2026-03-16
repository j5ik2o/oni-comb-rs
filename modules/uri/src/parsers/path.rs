use oni_comb_parser::error::ParseError;
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::models::path::Path;
use crate::parsers::common::pchar;

// segment = *pchar
fn segment<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    while pchar().attempt().parse_next(input).is_ok() {}
    Ok(input.slice_since(cp))
  })
}

// segment-nz = 1*pchar
fn segment_nz<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    let _pos = input.offset();
    pchar().parse_next(input)?;
    while pchar().attempt().parse_next(input).is_ok() {}
    Ok(input.slice_since(cp))
  })
}

// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" ) — no ":"
fn segment_nz_nc<'a>() -> impl Parser<StrInput<'a>, Output = &'a str, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let cp = input.checkpoint();
    let pos = input.offset();
    let ok = |input: &mut StrInput<'a>| -> bool {
      if crate::parsers::common::pct_encoded()
        .attempt()
        .parse_next(input)
        .is_ok()
      {
        return true;
      }
      satisfy(|c: char| {
        c.is_ascii_alphanumeric()
          || matches!(
            c,
            '-' | '.' | '_' | '~' | '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | '@'
          )
      })
      .attempt()
      .parse_next(input)
      .is_ok()
    };
    if !ok(input) {
      return Err(oni_comb_parser::fail::Fail::Backtrack(
        ParseError::expected_description(pos, "segment-nz-nc"),
      ));
    }
    while ok(input) {}
    Ok(input.slice_since(cp))
  })
}

// path-abempty = *( "/" segment )
pub fn path_abempty<'a>() -> impl Parser<StrInput<'a>, Output = Path<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let mut segs = Vec::new();
    while tag("/").attempt().parse_next(input).is_ok() {
      let seg = segment().parse_next(input)?;
      segs.push(seg);
    }
    if segs.is_empty() {
      Ok(Path::Empty)
    } else {
      Ok(Path::Abempty(segs))
    }
  })
}

// path-absolute = "/" [ segment-nz *( "/" segment ) ]
pub fn path_absolute<'a>() -> impl Parser<StrInput<'a>, Output = Path<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    tag("/").parse_next(input)?;
    let mut segs = Vec::new();
    if let Ok(first) = segment_nz().attempt().parse_next(input) {
      segs.push(first);
      while tag("/").attempt().parse_next(input).is_ok() {
        let seg = segment().parse_next(input)?;
        segs.push(seg);
      }
    }
    Ok(Path::Absolute(segs))
  })
}

// path-rootless = segment-nz *( "/" segment )
pub fn path_rootless<'a>() -> impl Parser<StrInput<'a>, Output = Path<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let first = segment_nz().parse_next(input)?;
    let mut segs = vec![first];
    while tag("/").attempt().parse_next(input).is_ok() {
      let seg = segment().parse_next(input)?;
      segs.push(seg);
    }
    Ok(Path::Rootless(segs))
  })
}

// path-noscheme = segment-nz-nc *( "/" segment )
pub fn path_noscheme<'a>() -> impl Parser<StrInput<'a>, Output = Path<'a>, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'a>| {
    let first = segment_nz_nc().parse_next(input)?;
    let mut segs = vec![first];
    while tag("/").attempt().parse_next(input).is_ok() {
      let seg = segment().parse_next(input)?;
      segs.push(seg);
    }
    Ok(Path::NoScheme(segs))
  })
}
