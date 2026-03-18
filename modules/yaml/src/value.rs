use std::collections::BTreeMap;

/// YAML value representation.
#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
  Null,
  Bool(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Sequence(Vec<YamlValue>),
  Mapping(BTreeMap<String, YamlValue>),
  Tagged { tag: String, value: Box<YamlValue> },
}
