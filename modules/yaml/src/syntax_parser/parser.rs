use oni_comb_parser::error::{Expected, ParseError};

use crate::{CollectionStyle, YamlSyntaxDocument, YamlSyntaxNode, YamlSyntaxScalar};

pub(super) struct SyntaxParser<'a> {
  pub(super) src: &'a str,
  pub(super) pos: usize,
}

impl<'a> SyntaxParser<'a> {
  pub(super) fn new(src: &'a str) -> Self {
    Self { src, pos: 0 }
  }

  pub(super) fn parse_documents(&mut self) -> Result<Vec<YamlSyntaxDocument>, ParseError> {
    let mut documents = Vec::new();
    self.skip_trivia();

    while !self.is_eof() {
      self.consume_document_marker("---");
      self.skip_trivia();

      if self.consume_document_marker("...") {
        self.skip_trivia();
        continue;
      }

      if self.is_eof() {
        break;
      }

      let root = self.parse_node()?;
      documents.push(YamlSyntaxDocument { root });

      self.skip_trivia();

      if self.consume_document_marker("...") {
        self.skip_trivia();
      }

      if self.is_eof() {
        break;
      }

      if self.peek_char() == Some(':') {
        return Err(self.unsupported("block mapping"));
      }

      if self.consume_document_marker("---") {
        self.skip_trivia();
        continue;
      }

      return Err(self.error(Expected::Description("document boundary")));
    }

    Ok(documents)
  }

  fn parse_node(&mut self) -> Result<YamlSyntaxNode, ParseError> {
    self.skip_trivia();

    match self.peek_char() {
      Some('[') => self.parse_sequence(),
      Some('{') => self.parse_mapping(),
      Some('\'') => self.parse_single_quoted_scalar(),
      Some('"') => self.parse_double_quoted_scalar(),
      Some('|') | Some('>') => Err(self.unsupported("block scalar")),
      Some('&') => Err(self.unsupported("anchor")),
      Some('*') => Err(self.unsupported("alias")),
      Some('!') => Err(self.unsupported("tag")),
      Some('-') if self.next_char_is_whitespace() => Err(self.unsupported("block sequence")),
      Some(_) => self.parse_plain_scalar(),
      None => Err(self.error(Expected::Description("YAML node"))),
    }
  }

  fn parse_sequence(&mut self) -> Result<YamlSyntaxNode, ParseError> {
    self.expect_char('[')?;
    self.skip_trivia();

    let mut items = Vec::new();
    if self.consume_char(']') {
      return Ok(YamlSyntaxNode::Sequence {
        style: CollectionStyle::Flow,
        items,
      });
    }

    loop {
      items.push(self.parse_node()?);
      self.skip_trivia();

      if self.consume_char(',') {
        self.skip_trivia();
        continue;
      }

      self.expect_char(']')?;
      break;
    }

    Ok(YamlSyntaxNode::Sequence {
      style: CollectionStyle::Flow,
      items,
    })
  }

  fn parse_mapping(&mut self) -> Result<YamlSyntaxNode, ParseError> {
    self.expect_char('{')?;
    self.skip_trivia();

    let mut entries = Vec::new();
    if self.consume_char('}') {
      return Ok(YamlSyntaxNode::Mapping {
        style: CollectionStyle::Flow,
        entries,
      });
    }

    loop {
      let key = self.parse_node()?;
      self.skip_trivia();
      self.expect_char(':')?;

      if matches!(&key, YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain(value)) if value == "<<") {
        return Err(self.unsupported("merge key"));
      }

      self.skip_trivia();
      let value = self.parse_node()?;
      entries.push((key, value));
      self.skip_trivia();

      if self.consume_char(',') {
        self.skip_trivia();
        continue;
      }

      self.expect_char('}')?;
      break;
    }

    Ok(YamlSyntaxNode::Mapping {
      style: CollectionStyle::Flow,
      entries,
    })
  }
}
