//! YAML 1.2 parser built on oni-comb-parser.

mod anchor;
mod block;
mod common;
mod context;
mod document;
mod flow;
mod multiline;
mod scalar;
mod tag;
mod value;

pub use tag::apply_tag;
pub use value::YamlValue;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;
use oni_comb_parser::prelude::*;

use context::ParseContext;

/// Parse a single YAML document.
pub fn parse(src: &str) -> Result<YamlValue, ParseError> {
  let mut input = StrInput::new(src);
  let mut ctx = ParseContext::new();
  document::yaml_document(&mut input, &mut ctx).map_err(fail_to_error)
}

/// Parse multiple YAML documents from a single input.
pub fn parse_documents(src: &str) -> Result<Vec<YamlValue>, ParseError> {
  let mut input = StrInput::new(src);
  let mut ctx = ParseContext::new();
  document::yaml_documents(&mut input, &mut ctx).map_err(fail_to_error)
}

fn fail_to_error(e: Fail<ParseError>) -> ParseError {
  match e {
    Fail::Backtrack(e) | Fail::Cut(e) => e,
    Fail::Incomplete => ParseError::from_expected(0, Expected::Description("incomplete input")),
    Fail::ZeroProgress => ParseError::from_expected(0, Expected::Description("zero progress")),
  }
}
