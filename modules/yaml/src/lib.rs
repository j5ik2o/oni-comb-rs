//! YAML 1.2 parser built on oni-comb-parser.
//!
//! # 公開 API 方針
//!
//! このクレートは 2 系統の公開 API を提供する。
//!
//! ## Syntax API（低レベル）
//!
//! 意味解決前の構文木を返す。plain scalar を `int` や `bool` に解釈せず、
//! anchor / alias / tag も未解決のまま保持する。
//!
//! - [`parse_syntax`] — 単一ドキュメントの syntax tree を返す
//! - [`parse_syntax_documents`] — 複数ドキュメントの syntax tree を返す
//!
//! ## Resolved API（高レベル）
//!
//! 最終的な [`YamlValue`] を返す。内部で `parse_syntax` + `resolve` を合成する。
//! `parse_syntax` 系の導入によって、これらの API を syntax-only に格下げしない。
//!
//! - [`parse`] — 単一ドキュメントを解釈済みの値として返す
//! - [`parse_documents`] — 複数ドキュメントを解釈済みの値の列として返す
//!
//! ## エラー型
//!
//! - [`ParseError`] — syntax parsing の失敗
//! - `ResolveError`（将来導入）— resolver 段の失敗（未定義 anchor、tag 適用失敗等）
//!
//! ## Phase 1 の制限
//!
//! 現在は Phase 1（Syntax Foundation）であり、syntax API と
//! flow subset を対象にした resolved API を提供する。
//! Phase 1 で対応する構文は以下に限定される:
//!
//! - plain / single-quoted / double-quoted scalar
//! - flow sequence / flow mapping
//! - 行コメント
//! - 基本 document marker（`---` / `...`）
//!
//! 上記以外の構文（block syntax、alias、merge key、tag）に遭遇した場合、
//! syntax API / resolved API は [`ParseError`] を返す。

use oni_comb_parser::error::{ContextError, ExpectError, Expected};

mod collection_style;
mod syntax_parser;
mod yaml_syntax_document;
mod yaml_syntax_node;
mod yaml_syntax_scalar;
mod yaml_value;

pub use collection_style::CollectionStyle;
pub use oni_comb_parser::error::ParseError;
pub use yaml_syntax_document::YamlSyntaxDocument;
pub use yaml_syntax_node::YamlSyntaxNode;
pub use yaml_syntax_scalar::YamlSyntaxScalar;
pub use yaml_value::YamlValue;

/// Parse a single YAML document into an unresolved syntax tree.
pub fn parse_syntax(src: &str) -> Result<YamlSyntaxDocument, ParseError> {
  syntax_parser::parse_syntax(src)
}

/// Parse multiple YAML documents into unresolved syntax trees.
pub fn parse_syntax_documents(src: &str) -> Result<Vec<YamlSyntaxDocument>, ParseError> {
  syntax_parser::parse_syntax_documents(src)
}

/// Parse a single YAML document into a resolved value.
///
/// This Phase 1 implementation resolves the supported flow subset on top of
/// [`parse_syntax`]. Unsupported syntax still returns [`ParseError`].
pub fn parse(src: &str) -> Result<YamlValue, ParseError> {
  parse_syntax(src)
    .and_then(resolve_document)
    .map_err(|error| error.fill_location_from_src(src))
}

/// Parse multiple YAML documents into resolved values.
///
/// This Phase 1 implementation resolves the supported flow subset on top of
/// [`parse_syntax_documents`].
pub fn parse_documents(src: &str) -> Result<Vec<YamlValue>, ParseError> {
  parse_syntax_documents(src)?
    .into_iter()
    .map(resolve_document)
    .collect::<Result<Vec<_>, _>>()
    .map_err(|error| error.fill_location_from_src(src))
}

/// Apply a YAML core tag to a resolved value.
///
/// Phase 1 exposes the same manual escape hatch described in `docs/known-issues.md`
/// while tag parsing itself remains out of scope.
pub fn apply_tag(tag: &str, value: YamlValue) -> Result<YamlValue, ParseError> {
  match tag {
    "!!str" => Ok(YamlValue::String(stringify_value(&value))),
    "!!int" => match value {
      YamlValue::Integer(number) => Ok(YamlValue::Integer(number)),
      YamlValue::String(text) => text
        .parse()
        .map(YamlValue::Integer)
        .map_err(|_| tag_application_error("!!int", "integer-compatible YAML value")),
      _ => Err(tag_application_error("!!int", "integer-compatible YAML value")),
    },
    "!!bool" => match value {
      YamlValue::Bool(flag) => Ok(YamlValue::Bool(flag)),
      YamlValue::String(text) => match text.as_str() {
        "true" => Ok(YamlValue::Bool(true)),
        "false" => Ok(YamlValue::Bool(false)),
        _ => Err(tag_application_error("!!bool", "boolean-compatible YAML value")),
      },
      _ => Err(tag_application_error("!!bool", "boolean-compatible YAML value")),
    },
    "!!float" => match value {
      YamlValue::Float(number) => Ok(YamlValue::Float(number)),
      YamlValue::Integer(number) => Ok(YamlValue::Float(number as f64)),
      YamlValue::String(text) => text
        .parse()
        .map(YamlValue::Float)
        .map_err(|_| tag_application_error("!!float", "float-compatible YAML value")),
      _ => Err(tag_application_error("!!float", "float-compatible YAML value")),
    },
    "!!null" => match value {
      YamlValue::Null => Ok(YamlValue::Null),
      YamlValue::String(text) if text == "null" || text == "~" => Ok(YamlValue::Null),
      _ => Err(tag_application_error("!!null", "null-compatible YAML value")),
    },
    _ => Err(unsupported_yaml_tag().fill_location_from_src("")),
  }
}

fn resolve_document(document: YamlSyntaxDocument) -> Result<YamlValue, ParseError> {
  resolve_node(document.root)
}

fn resolve_node(node: YamlSyntaxNode) -> Result<YamlValue, ParseError> {
  match node {
    YamlSyntaxNode::Scalar(scalar) => Ok(resolve_scalar(scalar)),
    YamlSyntaxNode::Sequence { items, .. } => items
      .into_iter()
      .map(resolve_node)
      .collect::<Result<Vec<_>, _>>()
      .map(YamlValue::Sequence),
    YamlSyntaxNode::Mapping { entries, .. } => {
      let mut mapping = std::collections::BTreeMap::new();
      for (key, value) in entries {
        mapping.insert(resolve_mapping_key(key)?, resolve_node(value)?);
      }
      Ok(YamlValue::Mapping(mapping))
    }
  }
}

fn resolve_mapping_key(node: YamlSyntaxNode) -> Result<String, ParseError> {
  match resolve_node(node)? {
    YamlValue::Null => Ok("null".to_string()),
    YamlValue::Bool(flag) => Ok(if flag { "true" } else { "false" }.to_string()),
    YamlValue::Integer(number) => Ok(number.to_string()),
    YamlValue::Float(number) => Ok(number.to_string()),
    YamlValue::String(text) => Ok(text),
    YamlValue::Sequence(_) | YamlValue::Mapping(_) => Err(unsupported_mapping_key()),
  }
}

fn resolve_scalar(scalar: YamlSyntaxScalar) -> YamlValue {
  match scalar {
    YamlSyntaxScalar::Plain(text) => resolve_plain_scalar(&text),
    YamlSyntaxScalar::SingleQuoted(text) | YamlSyntaxScalar::DoubleQuoted(text) => YamlValue::String(text),
  }
}

fn resolve_plain_scalar(text: &str) -> YamlValue {
  match text {
    "null" | "~" => YamlValue::Null,
    "true" => YamlValue::Bool(true),
    "false" => YamlValue::Bool(false),
    ".inf" | "+.inf" => YamlValue::Float(f64::INFINITY),
    "-.inf" => YamlValue::Float(f64::NEG_INFINITY),
    ".nan" => YamlValue::Float(f64::NAN),
    _ => {
      if let Some(stripped) = text.strip_prefix("0x") {
        if let Ok(number) = i64::from_str_radix(stripped, 16) {
          return YamlValue::Integer(number);
        }
      }

      if let Some(stripped) = text.strip_prefix("0o") {
        if let Ok(number) = i64::from_str_radix(stripped, 8) {
          return YamlValue::Integer(number);
        }
      }

      if let Ok(number) = text.parse::<i64>() {
        return YamlValue::Integer(number);
      }

      if let Ok(number) = text.parse::<f64>() {
        return YamlValue::Float(number);
      }

      YamlValue::String(text.to_string())
    }
  }
}

fn stringify_value(value: &YamlValue) -> String {
  match value {
    YamlValue::Null => "null".to_string(),
    YamlValue::Bool(flag) => flag.to_string(),
    YamlValue::Integer(number) => number.to_string(),
    YamlValue::Float(number) => number.to_string(),
    YamlValue::String(text) => text.clone(),
    YamlValue::Sequence(values) => format!("{values:?}"),
    YamlValue::Mapping(entries) => format!("{entries:?}"),
  }
}

fn invalid_tag_application(tag: &'static str, expected: &'static str) -> ParseError {
  ParseError::from_expected(0, Expected::Description(expected))
    .add_context(tag)
    .add_context("invalid YAML tag application")
}

fn tag_application_error(tag: &'static str, expected: &'static str) -> ParseError {
  invalid_tag_application(tag, expected).fill_location_from_src("")
}

fn unsupported_yaml_tag() -> ParseError {
  ParseError::from_expected(0, Expected::Description("supported YAML core tag"))
    .add_context("invalid YAML tag application")
}

fn unsupported_mapping_key() -> ParseError {
  ParseError::from_expected(0, Expected::Description("scalar YAML mapping key"))
    .add_context("unsupported YAML mapping key")
}
