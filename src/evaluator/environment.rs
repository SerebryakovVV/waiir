use std::collections::HashMap;

use super::object::Object;

pub struct Environment {
  store: HashMap<String, Object>
}

impl Environment {
  pub fn new() -> Self {
    Self {
      store: HashMap::new()
    }
  }

  pub fn set(&mut self, name: String, value: Object) {
    self.store.insert(name, value);
  }

  pub fn get(&mut self, name: String) -> Option<&Object> {
    self.store.get(&name)
  }
}
