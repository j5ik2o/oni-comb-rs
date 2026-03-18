use std::collections::HashMap;

use crate::value::YamlValue;

/// YAML パース状態。アンカーマップを保持する。
pub(crate) struct ParseContext {
  pub anchors: HashMap<String, YamlValue>,
}

impl ParseContext {
  pub fn new() -> Self {
    Self {
      anchors: HashMap::new(),
    }
  }

  pub fn set_anchor(&mut self, name: String, value: YamlValue) {
    self.anchors.insert(name, value);
  }

  pub fn get_anchor(&self, name: &str) -> Option<&YamlValue> {
    self.anchors.get(name)
  }
}
