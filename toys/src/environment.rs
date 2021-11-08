use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
  bindings: HashMap<String, i64>,
  pub(crate) next: Option<Rc<Environment>>,
}

impl Environment {
  pub fn as_bindings_mut(&mut self) -> &HashMap<String, i64> {
    &mut self.bindings
  }

  pub fn as_bindings(&self) -> &HashMap<String, i64> {
    &self.bindings
  }

  pub fn find_binding(&self, name: &str) -> Option<&HashMap<String, i64>> {
    match self.bindings.get(name) {
      Some(_) => Some(&self.bindings),
      None => match &self.next {
        Some(n) => {
          let r = (*n).find_binding(name);
          r
        }
        None => None,
      },
    }
  }

  pub fn new(bindings: HashMap<String, i64>, next: Option<Rc<Environment>>) -> Environment {
    Self { bindings, next }
  }
}
