use std::borrow::Cow;
use std::collections::BTreeMap;

/// JSON value representation.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue<'a> {
  Null,
  Bool(bool),
  Number(f64),
  String(Cow<'a, str>),
  Array(Vec<JsonValue<'a>>),
  Object(BTreeMap<Cow<'a, str>, JsonValue<'a>>),
}
