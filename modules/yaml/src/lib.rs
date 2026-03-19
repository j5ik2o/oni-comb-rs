//! YAML 1.2 parser built on oni-comb-parser.

mod block;
mod common;
mod document;
mod flow;
mod multiline;
mod scalar;
mod tag;
mod value;
pub(crate) mod yaml_combinators;
pub(crate) mod yaml_input;

pub use tag::apply_tag;
pub use value::YamlValue;

use oni_comb_parser::error::{ExpectError, Expected, ParseError};
use oni_comb_parser::fail::Fail;

use yaml_input::YamlInput;

/// Parse a single YAML document.
pub fn parse(src: &str) -> Result<YamlValue, ParseError> {
  let mut input = YamlInput::new(src);
  document::yaml_document(&mut input).map_err(fail_to_error)
}

/// Parse multiple YAML documents from a single input.
pub fn parse_documents(src: &str) -> Result<Vec<YamlValue>, ParseError> {
  let mut input = YamlInput::new(src);
  document::yaml_documents(&mut input).map_err(fail_to_error)
}

fn fail_to_error(e: Fail<ParseError>) -> ParseError {
  match e {
    Fail::Backtrack(e) | Fail::Cut(e) => e,
    Fail::Incomplete => ParseError::from_expected_with_location(0, 0, 0, Expected::Description("incomplete input")),
    Fail::ZeroProgress => ParseError::from_expected_with_location(0, 0, 0, Expected::Description("zero progress")),
  }
}
