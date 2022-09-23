use crate::model::config_array_value::ConfigArrayValue;
use crate::model::config_duration_value::ConfigDurationValue;
use crate::model::config_number_value::ConfigNumberValue;
use crate::model::config_object_value::ConfigObjectValue;
use crate::model::ConfigFactory;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigIncludeValue {
  method: String,
  file_name: String,
}

impl ConfigIncludeValue {
  pub fn new(method: String, file_name: String) -> Self {
    Self { method, file_name }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
  Null,
  Bool(bool),
  String(String),
  Number(ConfigNumberValue),
  Duration(ConfigDurationValue),
  Array(ConfigArrayValue),
  Object(ConfigObjectValue),
  Reference(String, bool),
  Include(ConfigIncludeValue),
  Link(Rc<ConfigValueLink>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigValueLink {
  prev: Rc<ConfigValue>,
  value: ConfigValue,
}

impl ConfigValueLink {
  pub fn new(prev: Rc<ConfigValue>, value: ConfigValue) -> Self {
    Self { prev, value }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValueList {
  Cons(ConfigValue, Rc<ConfigValueList>),
  Nil,
}

impl ConfigValue {
  pub fn push(&mut self, cv: ConfigValue) {
    let cvl = ConfigValueLink::new(Rc::new(self.clone()), cv);
    *self = ConfigValue::Link(Rc::new(cvl))
  }

  pub fn to_vec(&self) -> Vec<ConfigValue> {
    match self {
      ConfigValue::Link(cv) => {
        let mut cur = cv.clone();
        let mut result = vec![cur.value.clone()];
        while let ConfigValue::Link(prev_cur) = &*cur.prev {
          cur = prev_cur.clone();
          result.push(cur.value.clone());
        }
        result.push((*cur.prev).clone());
        result.reverse();
        result
      }
      cv => vec![cv.clone()],
    }
  }

  pub fn combine(&mut self, other: Self) {
    match &other {
      o @ ConfigValue::Link(_cv) => {
        for e in o.to_vec() {
          self.clone().push(e);
        }
      }
      _ => {}
    }
  }

  pub fn latest(&self) -> &Self {
    match self {
      ConfigValue::Link(cv) => &cv.value,
      cv => cv,
    }
  }

  pub fn prev_latest(&self) -> &Self {
    println!("self = {:?}", self);
    match self {
      ConfigValue::Link(cv) => match &*(cv.prev) {
        ConfigValue::Link(prev_cv) => {
          let ret = &(prev_cv.value);
          ret
        }
        cv => cv,
      },
      cv => cv,
    }
  }

  pub fn has_child(&self) -> bool {
    match self {
      ConfigValue::Object(..) => true,
      ConfigValue::Array(..) => true,
      _ => false,
    }
  }

  pub fn is_include(&self) -> bool {
    match self {
      ConfigValue::Include(_m) => true,
      _ => false,
    }
  }

  pub fn include(&self) -> Option<&ConfigIncludeValue> {
    match self {
      ConfigValue::Include(m) => Some(m),
      _ => None,
    }
  }

  pub fn ref_name(&self) -> Option<&String> {
    match self {
      ConfigValue::Reference(ref_name, ..) => Some(ref_name),
      _ => None,
    }
  }

  pub fn ref_missing(&self) -> Option<bool> {
    match self {
      ConfigValue::Reference(.., missing) => Some(*missing),
      _ => None,
    }
  }

  pub fn get_include_value(&self) -> Option<&ConfigIncludeValue> {
    match self {
      ConfigValue::Include(civ) => Some(civ),
      _ => None,
    }
  }

  pub fn get_object_value(&self) -> Option<&ConfigObjectValue> {
    match self {
      ConfigValue::Object(cov) => Some(cov),
      _ => None,
    }
  }

  pub fn get_array_value(&self) -> Option<&ConfigArrayValue> {
    match self {
      ConfigValue::Array(cav) => Some(cav),
      _ => None,
    }
  }

  pub fn get_value_link(&self) -> Option<&ConfigValueLink> {
    match self {
      ConfigValue::Link(cvl) => Some(&*cvl),
      _ => None,
    }
  }

  pub fn resolve(&mut self, source: Option<&Self>, parent: Option<&Self>) {
    match (self, source) {
      (cvi @ ConfigValue::Include(..), ..) => {
        let mut config_factory = ConfigFactory::new();
        let c = config_factory
          .load_from_file(&cvi.get_include_value().unwrap().file_name)
          .unwrap();
        *cvi = c.to_config_value().clone();
      }
      (cvl @ ConfigValue::Link(..), Some(..)) => {
        let cvs = cvl.to_vec();
        let mut head = cvs[0].clone();
        head.resolve(source, Some(cvl));

        for e in &cvs[1..] {
          let mut ee = e.clone();
          ee.resolve(source, Some(cvl));
          head.push(ee.clone());
        }

        *cvl = head;
      }
      (cva @ ConfigValue::Array(..), Some(..)) => {
        let av = cva.get_array_value().unwrap();
        let mut m = vec![];
        for mut cv in av.0.clone().into_iter() {
          cv.resolve(source, None);
          m.push(cv);
        }
        *cva = ConfigValue::Array(ConfigArrayValue::new(m));
      }
      (cvo @ ConfigValue::Object(..), Some(..)) => {
        let ov = cvo.get_object_value().unwrap();
        let mut m = HashMap::new();
        for (k, mut cv) in ov.0.clone().into_iter() {
          cv.resolve(source, None);
          m.insert(k, cv);
        }
        *cvo = ConfigValue::Object(ConfigObjectValue::new(m));
      }
      (cvr @ ConfigValue::Reference(..), Some(src)) => {
        let ref_value = src
          .get_value(cvr.ref_name().unwrap())
          .cloned()
          .or_else(|| env::var(cvr.ref_name().unwrap()).ok().map(|s| ConfigValue::String(s)));
        if cvr.ref_missing().unwrap() {
          if ref_value.is_some() {
            *cvr = ref_value.unwrap();
          } else {
            *cvr = parent.unwrap().prev_latest().clone();
          }
        } else {
          if ref_value.is_none() {
            panic!("Cannot resolve the reference: {}", cvr.ref_name().unwrap())
          }
          *cvr = ref_value.unwrap();
        }
      }
      _ => {}
    }
  }

  pub fn get_value(&self, path: &str) -> Option<&ConfigValue> {
    let keys = path.split(".").collect::<Vec<_>>();
    let key = keys[0];
    let child_count = keys.len() - 1;
    match self {
      ConfigValue::Object(map) => match map.0.get(key) {
        Some(cv) if child_count > 0 => {
          let next_path = &path[(key.len() + 1) as usize..];
          cv.latest().get_value(next_path)
        }
        Some(cv) => Some(cv.latest()),
        None => None,
      },
      _ => None,
    }
  }

  pub fn contains(&self, key: &str) -> bool {
    match self {
      ConfigValue::Object(map) => map.0.contains_key(key),
      _ => false,
    }
  }

  fn link_rc(&self) -> Option<Rc<ConfigValueLink>> {
    match self {
      ConfigValue::Link(cvl) => Some(cvl.clone()),
      _ => None,
    }
  }

  pub fn with_fallback(&mut self, other: Self) {
    match (self, other) {
      (ConfigValue::Object(l), ConfigValue::Object(r)) => {
        l.with_fallback(r);
      }
      (ConfigValue::Array(l), ConfigValue::Array(r)) => {
        l.with_fallback(r);
      }
      (ConfigValue::Link(..), ConfigValue::Link(..)) => {}
      (re @ ConfigValue::Link(..), r) => {
        let mut n = re.get_value_link().unwrap().value.clone();
        n.with_fallback(r);
        re.push(n);
      }
      (..) => {}
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::model::config_value::ConfigValue;

  #[test]
  fn test_push() {
    let mut config_value = ConfigValue::Bool(true);
    config_value.push(ConfigValue::String("ABC".to_string()));
    config_value.push(ConfigValue::Null);

    println!("{:?}", config_value);
    println!("{:?}", config_value.to_vec());

    assert_eq!(config_value.latest().clone(), ConfigValue::Null);
    assert_eq!(
      config_value.prev_latest().clone(),
      ConfigValue::String("ABC".to_string())
    );
  }

  #[test]
  fn test_combine() {
    let mut first = ConfigValue::Bool(true);
    first.push(ConfigValue::String("ABC".to_string()));
    first.push(ConfigValue::Null);
    let mut second = ConfigValue::Bool(false);
    second.push(ConfigValue::String("XYZ".to_string()));
    second.push(ConfigValue::Reference("ABC".to_string(), false));

    let mut t = first.clone();

    t.combine(second);

    // [Bool(true), "ABC", Null]
    println!("{:?}", t);
    println!("{:?}", t.to_vec());
  }
}
