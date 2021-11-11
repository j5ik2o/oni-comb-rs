use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Query {
  params: Vec<(String, Option<String>)>,
}

impl Default for Query {
  fn default() -> Self {
    Query { params: Vec::default() }
  }
}

impl PartialOrd for Query {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.to_string().partial_cmp(&other.to_string())
  }
}

impl std::fmt::Display for Query {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl From<Vec<(String, Option<String>)>> for Query {
  fn from(src: Vec<(String, Option<String>)>) -> Self {
    let params = src.into_iter().collect::<Vec<_>>();
    Self { params }
  }
}

impl From<Vec<(&str, Option<&str>)>> for Query {
  fn from(src: Vec<(&str, Option<&str>)>) -> Self {
    let params = src
      .into_iter()
      .map(|(k, v)| (k.to_string(), v.map(|v| v.to_string())))
      .collect::<Vec<_>>();
    Self { params }
  }
}

impl Query {
  pub fn new(params: Vec<(String, Option<String>)>) -> Self {
    Self { params }
  }

  pub fn params(&self) -> HashMap<&String, Vec<&String>> {
    let mut result: HashMap<&String, Vec<&String>> = HashMap::new();
    for (key, value) in self.params.iter() {
      match (result.get_mut(&key), value) {
        (None, None) => {
          result.insert(key, Vec::new());
        }
        (None, Some(v)) => {
          result.insert(key, vec![v]);
        }
        (Some(values), Some(v)) => {
          values.push(v);
        }
        _ => (),
      }
    }
    result
  }

  pub fn add(&mut self, key: String, value: String) {
    self.params.push((key, Some(value)));
  }

  pub fn add_opt(&mut self, key: String, value: Option<String>) {
    match value {
      Some(v) => {
        self.add(key, v);
        ()
      }
      None => (),
    }
  }

  pub fn get_param(&self, key: String) -> Option<Vec<&String>> {
    self.params().get(&key).map(|v| v.clone())
  }

  pub fn as_string(&self) -> String {
    self
      .params
      .iter()
      .map(|(k, v)| {
        if v.is_none() {
          k.clone()
        } else {
          format!(
            "{}{}",
            k,
            v.as_ref().map(|s| format!("={}", s)).unwrap_or("".to_string())
          )
        }
      })
      .collect::<Vec<_>>()
      .join("&")
  }
}
