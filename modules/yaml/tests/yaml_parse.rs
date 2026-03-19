use oni_comb_yaml::{parse, parse_documents, YamlValue};
use std::collections::BTreeMap;

// ── Scalars ─────────────────────────────────────

#[test]
fn parse_null() {
  assert_eq!(parse("null").unwrap(), YamlValue::Null);
}

#[test]
fn parse_null_tilde() {
  assert_eq!(parse("~").unwrap(), YamlValue::Null);
}

#[test]
fn parse_bool_true() {
  assert_eq!(parse("true").unwrap(), YamlValue::Bool(true));
}

#[test]
fn parse_bool_false() {
  assert_eq!(parse("false").unwrap(), YamlValue::Bool(false));
}

#[test]
fn parse_integer() {
  assert_eq!(parse("42").unwrap(), YamlValue::Integer(42));
}

#[test]
fn parse_hex_integer() {
  assert_eq!(parse("0xFF").unwrap(), YamlValue::Integer(255));
}

#[test]
fn parse_octal_integer() {
  assert_eq!(parse("0o77").unwrap(), YamlValue::Integer(63));
}

#[test]
fn parse_float() {
  assert_eq!(parse("3.14").unwrap(), YamlValue::Float(3.14));
}

#[test]
fn parse_infinity() {
  assert_eq!(parse(".inf").unwrap(), YamlValue::Float(f64::INFINITY));
}

#[test]
fn parse_neg_infinity() {
  assert_eq!(parse("-.inf").unwrap(), YamlValue::Float(f64::NEG_INFINITY));
}

#[test]
fn parse_nan() {
  match parse(".nan").unwrap() {
    YamlValue::Float(f) => assert!(f.is_nan()),
    other => panic!("Expected NaN, got {:?}", other),
  }
}

#[test]
fn parse_plain_string() {
  assert_eq!(
    parse("hello world").unwrap(),
    YamlValue::String("hello world".to_string())
  );
}

#[test]
fn parse_double_quoted_string() {
  assert_eq!(
    parse(r#""hello world""#).unwrap(),
    YamlValue::String("hello world".to_string())
  );
}

#[test]
fn parse_single_quoted_string() {
  assert_eq!(
    parse("'hello world'").unwrap(),
    YamlValue::String("hello world".to_string())
  );
}

#[test]
fn parse_single_quoted_non_ascii() {
  assert_eq!(parse("'café'").unwrap(), YamlValue::String("café".to_string()));
}

#[test]
fn parse_single_quoted_escaped_quote() {
  assert_eq!(parse("'it''s'").unwrap(), YamlValue::String("it's".to_string()));
}

// ── Flow Style ──────────────────────────────────

#[test]
fn parse_flow_sequence() {
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
fn parse_flow_mapping() {
  let result = parse("{name: oni-comb, version: 2}").unwrap();
  let mut expected = BTreeMap::new();
  expected.insert("name".to_string(), YamlValue::String("oni-comb".to_string()));
  expected.insert("version".to_string(), YamlValue::Integer(2));
  assert_eq!(result, YamlValue::Mapping(expected));
}

#[test]
fn parse_flow_nested() {
  let result = parse("{a: [1, 2], b: {c: true}}").unwrap();
  if let YamlValue::Mapping(map) = result {
    assert_eq!(
      map["a"],
      YamlValue::Sequence(vec![YamlValue::Integer(1), YamlValue::Integer(2)])
    );
    let mut inner = BTreeMap::new();
    inner.insert("c".to_string(), YamlValue::Bool(true));
    assert_eq!(map["b"], YamlValue::Mapping(inner));
  } else {
    panic!("Expected mapping");
  }
}

// ── Block Style ─────────────────────────────────

#[test]
fn parse_block_mapping() {
  let input = "key1: value1\nkey2: value2";
  let result = parse(input).unwrap();
  let mut expected = BTreeMap::new();
  expected.insert("key1".to_string(), YamlValue::String("value1".to_string()));
  expected.insert("key2".to_string(), YamlValue::String("value2".to_string()));
  assert_eq!(result, YamlValue::Mapping(expected));
}

#[test]
fn parse_nested_block_mapping() {
  let input = "parent:\n  child1: value1\n  child2: value2";
  let result = parse(input).unwrap();
  let mut children = BTreeMap::new();
  children.insert("child1".to_string(), YamlValue::String("value1".to_string()));
  children.insert("child2".to_string(), YamlValue::String("value2".to_string()));
  let mut expected = BTreeMap::new();
  expected.insert("parent".to_string(), YamlValue::Mapping(children));
  assert_eq!(result, YamlValue::Mapping(expected));
}

#[test]
fn parse_block_sequence() {
  let input = "- item1\n- item2\n- item3";
  let result = parse(input).unwrap();
  assert_eq!(
    result,
    YamlValue::Sequence(vec![
      YamlValue::String("item1".to_string()),
      YamlValue::String("item2".to_string()),
      YamlValue::String("item3".to_string()),
    ])
  );
}

#[test]
fn parse_mapping_with_flow_value() {
  let input = "items: [1, 2, 3]";
  let result = parse(input).unwrap();
  let mut expected = BTreeMap::new();
  expected.insert(
    "items".to_string(),
    YamlValue::Sequence(vec![
      YamlValue::Integer(1),
      YamlValue::Integer(2),
      YamlValue::Integer(3),
    ]),
  );
  assert_eq!(result, YamlValue::Mapping(expected));
}

// ── Comments ────────────────────────────────────

#[test]
fn parse_with_comment() {
  let input = "key: value # this is a comment";
  let result = parse(input).unwrap();
  let mut expected = BTreeMap::new();
  expected.insert("key".to_string(), YamlValue::String("value".to_string()));
  assert_eq!(result, YamlValue::Mapping(expected));
}

#[test]
fn parse_comment_only_lines() {
  let input = "# header comment\nkey: value";
  let result = parse(input).unwrap();
  let mut expected = BTreeMap::new();
  expected.insert("key".to_string(), YamlValue::String("value".to_string()));
  assert_eq!(result, YamlValue::Mapping(expected));
}

// ── Multiline Strings ───────────────────────────

#[test]
fn parse_literal_block() {
  let input = "text: |\n  line1\n  line2\n";
  let result = parse(input).unwrap();
  if let YamlValue::Mapping(map) = result {
    assert_eq!(map["text"], YamlValue::String("line1\nline2\n".to_string()));
  } else {
    panic!("Expected mapping");
  }
}

#[test]
fn parse_literal_block_with_leading_blank_line() {
  let input = "text: |\n\n  hello\n";
  let result = parse(input).unwrap();
  if let YamlValue::Mapping(map) = result {
    assert_eq!(map["text"], YamlValue::String("\nhello\n".to_string()));
  } else {
    panic!("Expected mapping");
  }
}

#[test]
fn parse_strip_chomping() {
  let input = "text: |-\n  line1\n  line2\n";
  let result = parse(input).unwrap();
  if let YamlValue::Mapping(map) = result {
    assert_eq!(map["text"], YamlValue::String("line1\nline2".to_string()));
  } else {
    panic!("Expected mapping");
  }
}

// ── Document Markers ────────────────────────────

#[test]
fn parse_with_document_start() {
  let input = "---\nkey: value";
  let result = parse(input).unwrap();
  let mut expected = BTreeMap::new();
  expected.insert("key".to_string(), YamlValue::String("value".to_string()));
  assert_eq!(result, YamlValue::Mapping(expected));
}

#[test]
fn parse_multiple_documents() {
  let input = "---\ndoc1: value1\n---\ndoc2: value2";
  let result = parse_documents(input).unwrap();
  assert_eq!(result.len(), 2);
}

// ── Tags ────────────────────────────────────────

// ── Anchors & Aliases ───────────────────────────

#[test]
fn parse_anchor_and_alias() {
  let input = "- &anchor value\n- *anchor";
  let result = parse(input).unwrap();
  assert_eq!(
    result,
    YamlValue::Sequence(vec![
      YamlValue::String("value".to_string()),
      YamlValue::String("value".to_string()),
    ])
  );
}

#[test]
fn parse_block_anchor_on_next_line() {
  let input = "defaults: &defs\n  adapter: postgres\n  host: localhost";
  let result = parse(input).unwrap();
  if let YamlValue::Mapping(map) = &result {
    if let YamlValue::Mapping(defaults) = &map["defaults"] {
      assert_eq!(defaults["adapter"], YamlValue::String("postgres".to_string()));
      assert_eq!(defaults["host"], YamlValue::String("localhost".to_string()));
    } else {
      panic!("Expected defaults to be a mapping, got {:?}", map["defaults"]);
    }
  } else {
    panic!("Expected mapping, got {:?}", result);
  }
}

#[test]
fn parse_merge_key() {
  // Test merge with flow-style anchor value (simpler case)
  let input = "defaults: &defs {adapter: postgres}\ndev:\n  <<: *defs\n  db: mydb";
  let result = parse(input).unwrap();
  if let YamlValue::Mapping(map) = &result {
    if let YamlValue::Mapping(dev) = &map["dev"] {
      assert_eq!(dev["adapter"], YamlValue::String("postgres".to_string()));
      assert_eq!(dev["db"], YamlValue::String("mydb".to_string()));
    } else {
      panic!("Expected dev to be a mapping, got {:?}", map.get("dev"));
    }
  } else {
    panic!("Expected mapping, got {:?}", result);
  }
}

// ── Tags ────────────────────────────────────────

#[test]
fn apply_str_tag() {
  let value = YamlValue::Integer(42);
  let tagged = oni_comb_yaml::apply_tag("!!str", value);
  assert_eq!(tagged, YamlValue::String("42".to_string()));
}

// ── Error Reporting ─────────────────────────────

#[test]
fn error_reports_position() {
  let input = "key: [invalid}";
  let err = parse(input).unwrap_err();
  assert!(err.position > 0);
}
