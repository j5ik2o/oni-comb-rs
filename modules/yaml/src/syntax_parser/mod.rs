mod cursor;
mod parser;
mod scalar;

use oni_comb_parser::error::{Expected, ParseError};

use crate::YamlSyntaxDocument;

use parser::SyntaxParser;

pub(crate) fn parse_syntax(src: &str) -> Result<YamlSyntaxDocument, ParseError> {
  let mut parser = SyntaxParser::new(src);
  let documents = parser.parse_documents()?;
  match documents.as_slice() {
    [] => Err(parser.error(Expected::Description("YAML document"))),
    [document] => Ok(document.clone()),
    _ => Err(parser.error(Expected::Description("single YAML document"))),
  }
}

pub(crate) fn parse_syntax_documents(src: &str) -> Result<Vec<YamlSyntaxDocument>, ParseError> {
  SyntaxParser::new(src).parse_documents()
}
