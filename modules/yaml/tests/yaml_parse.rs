use oni_comb_yaml::{
  apply_tag, parse, parse_documents, parse_syntax, parse_syntax_documents, CollectionStyle, YamlSyntaxDocument,
  YamlSyntaxNode, YamlSyntaxScalar, YamlValue,
};
use std::collections::BTreeMap;

fn assert_phase1_unsupported(src: &str, feature: &str) {
  let error = parse_syntax(src).unwrap_err();
  assert!(error.context.contains(&"unsupported in YAML Phase 1"));
  assert!(error.line > 0);
  assert!(error.column > 0);
  assert!(error
    .expected
    .iter()
    .any(|expected| { matches!(expected, oni_comb_parser::error::Expected::Description(value) if *value == feature) }));
}

#[test]
fn parse_syntax_preserves_plain_scalar() {
  let document = parse_syntax("42").unwrap();
  assert_eq!(
    document,
    YamlSyntaxDocument {
      root: YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("42".to_string())),
    }
  );
}

#[test]
fn parse_syntax_preserves_single_quoted_scalar() {
  let document = parse_syntax("'it''s'").unwrap();
  assert_eq!(
    document,
    YamlSyntaxDocument {
      root: YamlSyntaxNode::Scalar(YamlSyntaxScalar::SingleQuoted("it's".to_string())),
    }
  );
}

#[test]
fn parse_syntax_preserves_double_quoted_scalar() {
  let document = parse_syntax("\"hello\\nworld\"").unwrap();
  assert_eq!(
    document,
    YamlSyntaxDocument {
      root: YamlSyntaxNode::Scalar(YamlSyntaxScalar::DoubleQuoted("hello\nworld".to_string())),
    }
  );
}

#[test]
fn parse_syntax_flow_sequence() {
  let document = parse_syntax("[1, 2, 3]").unwrap();
  assert_eq!(
    document,
    YamlSyntaxDocument {
      root: YamlSyntaxNode::Sequence {
        style: CollectionStyle::Flow,
        items: vec![
          YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("1".to_string())),
          YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("2".to_string())),
          YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("3".to_string())),
        ],
      },
    }
  );
}

#[test]
fn parse_syntax_flow_mapping() {
  let document = parse_syntax("{name: oni-comb, version: 2}").unwrap();
  assert_eq!(
    document,
    YamlSyntaxDocument {
      root: YamlSyntaxNode::Mapping {
        style: CollectionStyle::Flow,
        entries: vec![
          (
            YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("name".to_string())),
            YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("oni-comb".to_string())),
          ),
          (
            YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("version".to_string())),
            YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("2".to_string())),
          ),
        ],
      },
    }
  );
}

#[test]
fn parse_syntax_flow_nested() {
  let document = parse_syntax("{a: [1, 2], b: {c: true}}").unwrap();
  let YamlSyntaxNode::Mapping { entries, .. } = document.root else {
    panic!("expected mapping");
  };

  assert_eq!(entries.len(), 2);
}

#[test]
fn parse_syntax_ignores_comment() {
  let document = parse_syntax("{key: value} # trailing comment").unwrap();
  assert_eq!(
    document,
    YamlSyntaxDocument {
      root: YamlSyntaxNode::Mapping {
        style: CollectionStyle::Flow,
        entries: vec![(
          YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("key".to_string())),
          YamlSyntaxNode::Scalar(YamlSyntaxScalar::Plain("value".to_string())),
        )],
      },
    }
  );
}

#[test]
fn parse_syntax_documents_support_document_markers() {
  let documents = parse_syntax_documents("---\n[1, 2]\n---\n{name: oni-comb}").unwrap();
  assert_eq!(documents.len(), 2);
}

#[test]
fn parse_syntax_documents_support_document_end_marker() {
  let documents = parse_syntax_documents("---\n{name: oni-comb}\n...").unwrap();
  assert_eq!(documents.len(), 1);
}

#[test]
fn parse_syntax_documents_ignores_comment_only_input() {
  let documents = parse_syntax_documents("# comment only").unwrap();
  assert!(documents.is_empty());
}

#[test]
fn parse_syntax_documents_ignores_whitespace_only_input() {
  let documents = parse_syntax_documents("  \n\t").unwrap();
  assert!(documents.is_empty());
}

#[test]
fn parse_syntax_rejects_block_mapping() {
  assert_phase1_unsupported("parent:\n  child: value", "block mapping");
}

#[test]
fn parse_syntax_rejects_block_scalar() {
  assert_phase1_unsupported("|\n  value", "block scalar");
}

#[test]
fn parse_syntax_rejects_anchor() {
  assert_phase1_unsupported("&anchor value", "anchor");
}

#[test]
fn parse_syntax_rejects_alias() {
  assert_phase1_unsupported("*anchor", "alias");
}

#[test]
fn parse_syntax_rejects_tag() {
  assert_phase1_unsupported("!custom value", "tag");
}

#[test]
fn parse_syntax_rejects_block_sequence() {
  assert_phase1_unsupported("- item", "block sequence");
}

#[test]
fn parse_syntax_rejects_merge_key() {
  assert_phase1_unsupported("{<<: *defs}", "merge key");
}

#[test]
fn parse_resolves_plain_scalar_types() {
  assert_eq!(parse("42").unwrap(), YamlValue::Integer(42));
  assert_eq!(parse("true").unwrap(), YamlValue::Bool(true));
  assert_eq!(parse(".inf").unwrap(), YamlValue::Float(f64::INFINITY));
}

#[test]
fn parse_resolves_flow_sequence() {
  assert_eq!(
    parse("[1, 2, 3]").unwrap(),
    YamlValue::Sequence(vec![
      YamlValue::Integer(1),
      YamlValue::Integer(2),
      YamlValue::Integer(3),
    ])
  );
}

#[test]
fn parse_resolves_flow_mapping() {
  let result = parse("{name: oni-comb, version: 2}").unwrap();
  let mut expected = BTreeMap::new();
  expected.insert("name".to_string(), YamlValue::String("oni-comb".to_string()));
  expected.insert("version".to_string(), YamlValue::Integer(2));
  assert_eq!(result, YamlValue::Mapping(expected));
}

#[test]
fn parse_documents_resolve_multiple_documents() {
  let documents = parse_documents("---\n[1, 2]\n---\n{name: oni-comb}").unwrap();
  assert_eq!(documents.len(), 2);
}

#[test]
fn apply_str_tag() {
  let value = YamlValue::Integer(42);
  let tagged = apply_tag("!!str", value).unwrap();
  assert_eq!(tagged, YamlValue::String("42".to_string()));
}

#[test]
fn apply_int_tag_accepts_integer_compatible_values() {
  assert_eq!(apply_tag("!!int", YamlValue::Integer(42)).unwrap(), YamlValue::Integer(42));
  assert_eq!(
    apply_tag("!!int", YamlValue::String("42".to_string())).unwrap(),
    YamlValue::Integer(42)
  );
}

#[test]
fn apply_tag_returns_error_for_invalid_int_payload() {
  let error = apply_tag("!!int", YamlValue::String("abc".to_string())).unwrap_err();
  assert!(error.context.contains(&"invalid YAML tag application"));
  assert!(error.context.contains(&"!!int"));
  assert_eq!((error.line, error.column), (1, 1));
}

#[test]
fn apply_bool_tag_accepts_bool_compatible_values() {
  assert_eq!(
    apply_tag("!!bool", YamlValue::String("true".to_string())).unwrap(),
    YamlValue::Bool(true)
  );
  assert_eq!(
    apply_tag("!!bool", YamlValue::Bool(false)).unwrap(),
    YamlValue::Bool(false)
  );
}

#[test]
fn apply_bool_tag_rejects_invalid_payload() {
  let error = apply_tag("!!bool", YamlValue::String("yes".to_string())).unwrap_err();
  assert!(error.context.contains(&"!!bool"));
  assert_eq!((error.line, error.column), (1, 1));
}

#[test]
fn apply_float_tag_accepts_float_compatible_values() {
  assert_eq!(
    apply_tag("!!float", YamlValue::Integer(42)).unwrap(),
    YamlValue::Float(42.0)
  );
  assert_eq!(
    apply_tag("!!float", YamlValue::String("3.14".to_string())).unwrap(),
    YamlValue::Float(3.14)
  );
}

#[test]
fn apply_float_tag_rejects_invalid_payload() {
  let error = apply_tag("!!float", YamlValue::String("abc".to_string())).unwrap_err();
  assert!(error.context.contains(&"!!float"));
  assert_eq!((error.line, error.column), (1, 1));
}

#[test]
fn apply_null_tag_accepts_only_null_compatible_values() {
  assert_eq!(apply_tag("!!null", YamlValue::Null).unwrap(), YamlValue::Null);
  assert_eq!(
    apply_tag("!!null", YamlValue::String("null".to_string())).unwrap(),
    YamlValue::Null
  );
  assert_eq!(
    apply_tag("!!null", YamlValue::String("~".to_string())).unwrap(),
    YamlValue::Null
  );

  let error = apply_tag("!!null", YamlValue::Integer(42)).unwrap_err();
  assert!(error.context.contains(&"!!null"));
  assert_eq!((error.line, error.column), (1, 1));
}

#[test]
fn apply_unknown_tag_reports_location() {
  let error = apply_tag("!!unknown", YamlValue::Null).unwrap_err();
  assert!(error.context.contains(&"invalid YAML tag application"));
  assert_eq!((error.position, error.line, error.column), (0, 1, 1));
}

#[test]
fn parse_returns_error_for_non_scalar_mapping_key() {
  let error = parse("{[1]: 2}").unwrap_err();
  assert!(error.context.contains(&"unsupported YAML mapping key"));
  assert_eq!((error.position, error.line, error.column), (0, 1, 1));
}

#[test]
fn error_reports_position() {
  let error = parse("{key: [invalid}").unwrap_err();
  assert!(error.position > 0);
  assert!(error.line > 0);
  assert!(error.column > 0);
}
