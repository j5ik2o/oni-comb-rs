use std::collections::BTreeMap;

/// Resolved YAML value after schema interpretation and alias resolution.
///
/// This type is the output of the high-level `parse` / `parse_documents` API.
/// Currently a minimal definition for Phase 1; the resolver that produces
/// these values will be implemented in a later phase.
#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
  Null,
  Bool(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Sequence(Vec<YamlValue>),
  Mapping(BTreeMap<String, YamlValue>),
}
