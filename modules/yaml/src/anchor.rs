use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::value::YamlValue;

/// Resolve anchors and aliases in a parsed YAML tree.
/// This is a post-processing step after parsing.
#[allow(dead_code)]
pub fn resolve_anchors(value: &YamlValue) -> YamlValue {
  let mut anchors = HashMap::new();
  resolve_inner(value, &mut anchors)
}

#[allow(dead_code)]
fn resolve_inner(value: &YamlValue, anchors: &mut HashMap<String, YamlValue>) -> YamlValue {
  match value {
    YamlValue::Sequence(items) => {
      let resolved: Vec<_> = items.iter().map(|v| resolve_inner(v, anchors)).collect();
      YamlValue::Sequence(resolved)
    }
    YamlValue::Mapping(pairs) => {
      let mut resolved = BTreeMap::new();
      for (key, val) in pairs {
        // Handle merge key
        if key == "<<" {
          if let YamlValue::Mapping(merge_map) = resolve_inner(val, anchors) {
            for (mk, mv) in merge_map {
              resolved.entry(mk).or_insert(mv);
            }
          }
        } else {
          resolved.insert(key.clone(), resolve_inner(val, anchors));
        }
      }
      YamlValue::Mapping(resolved)
    }
    YamlValue::Tagged { tag, value } => YamlValue::Tagged {
      tag: tag.clone(),
      value: Box::new(resolve_inner(value, anchors)),
    },
    other => other.clone(),
  }
}
