use std::borrow::Cow;

/// MVP YAML value representation.
#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue<'a> {
  Null,
  Bool(bool),
  Integer(i64),
  String(Cow<'a, str>),
  Sequence(Vec<YamlValue<'a>>),
  Mapping(Vec<(YamlValue<'a>, YamlValue<'a>)>),
}
