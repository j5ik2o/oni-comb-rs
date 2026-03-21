use std::borrow::Cow;

use oni_comb_parser::input_stream::InputStream;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::prelude::StrInputStream;
use oni_comb_yaml::{parse, parse_value, yaml_value, YamlValue};

fn string(value: &'static str) -> YamlValue<'static> {
  YamlValue::String(Cow::Borrowed(value))
}

fn mapping(entries: Vec<(YamlValue<'static>, YamlValue<'static>)>) -> YamlValue<'static> {
  YamlValue::Mapping(entries)
}

fn sequence(values: Vec<YamlValue<'static>>) -> YamlValue<'static> {
  YamlValue::Sequence(values)
}

#[test]
fn parse_plain_scalar_mapping() {
  assert_eq!(
    parse("title: hello world").unwrap(),
    mapping(vec![(string("title"), string("hello world"))])
  );
}

#[test]
fn parse_quoted_scalars() {
  assert_eq!(
    parse("single: 'hello'\ndouble: \"world\"").unwrap(),
    mapping(vec![
      (string("single"), string("hello")),
      (string("double"), string("world"))
    ])
  );
}

#[test]
fn parse_null_bool_and_integer() {
  assert_eq!(
    parse("a: null\nb: true\nc: 42").unwrap(),
    mapping(vec![
      (string("a"), YamlValue::Null),
      (string("b"), YamlValue::Bool(true)),
      (string("c"), YamlValue::Integer(42)),
    ])
  );
}

#[test]
fn parse_block_mapping_with_nested_block_sequence() {
  assert_eq!(
    parse("items:\n  - milk\n  - eggs\n").unwrap(),
    mapping(vec![(string("items"), sequence(vec![string("milk"), string("eggs")]))])
  );
}

#[test]
fn parse_block_mapping_with_four_space_nested_block_sequence() {
  assert_eq!(
    parse("items:\n    - milk\n    - eggs\n").unwrap(),
    mapping(vec![(string("items"), sequence(vec![string("milk"), string("eggs")]))])
  );
}

#[test]
fn parse_top_level_block_sequence_item_with_colon_as_sequence() {
  assert_eq!(
    parse("- name: milk\n- eggs\n").unwrap(),
    sequence(vec![string("name: milk"), string("eggs")])
  );
}

#[test]
fn parse_flow_sequence_inside_block_mapping() {
  assert_eq!(
    parse("items: [one, two]\nnested:\n  key: value\n").unwrap(),
    mapping(vec![
      (string("items"), sequence(vec![string("one"), string("two")])),
      (string("nested"), mapping(vec![(string("key"), string("value"))])),
    ])
  );
}

#[test]
fn parse_top_level_flow_mapping() {
  assert_eq!(
    parse("{name: oni-comb, version: 2}").unwrap(),
    mapping(vec![
      (string("name"), string("oni-comb")),
      (string("version"), YamlValue::Integer(2))
    ])
  );
}

#[test]
fn parse_ignores_line_comment() {
  assert_eq!(
    parse("key: value # comment").unwrap(),
    mapping(vec![(string("key"), string("value"))])
  );
}

#[test]
fn parse_ignores_comment_after_closed_flow_value() {
  assert_eq!(
    parse("items: [one, two] # comment").unwrap(),
    mapping(vec![(string("items"), sequence(vec![string("one"), string("two")]))])
  );
}

#[test]
fn parse_rejects_trailing_non_comment_text() {
  assert!(parse("[one, two], trailing").is_err());
}

#[test]
fn parse_value_prefix_parses_closed_flow_collection() {
  assert_eq!(
    parse_value("[one, two], trailing").unwrap(),
    sequence(vec![string("one"), string("two")])
  );
}

#[test]
fn yaml_value_parser_leaves_trailing_input() {
  let mut input = StrInputStream::new("[one, two], trailing");
  let parsed = yaml_value().parse_next(&mut input).unwrap();

  assert_eq!(parsed, sequence(vec![string("one"), string("two")]));
  assert_eq!(input.remaining(), ", trailing");
}

#[test]
fn indentation_mismatch_reports_line_and_column() {
  let err = parse("root:\n  child: ok\n next: wrong\n").unwrap_err();
  assert_eq!(err.line, 3);
  assert_eq!(err.column, 2);
  assert!(!err.expected.is_empty());
  assert!(err.context.iter().any(|ctx| ctx.contains("indentation")));
}

#[test]
fn unterminated_flow_collection_reports_context() {
  let err = parse("items: [one, two").unwrap_err();
  assert!(err.position > 0);
  assert!(err.expected.contains(&oni_comb_parser::error::Expected::Char(']')));
  assert!(!err.context.is_empty());
}
