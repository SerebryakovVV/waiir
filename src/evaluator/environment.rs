use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::object::Object;


#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub store: HashMap<String, Object>,
  pub outer: Option<Rc<RefCell<Environment>>>
}

impl Environment {
  pub fn new() -> Self {
    Self {
      store: HashMap::new(),
      outer: None
    }
  }

  pub fn set(&mut self, name: String, value: Object) { // TODO: make it return value
    self.store.insert(name, value);
  }

  pub fn get(&self, name: String) -> Option<Object> {
    let mut get_res = self.store.get(&name);
    match get_res {
      Some(r) => Some(r.clone()),
      None    => {
        match &self.outer {
          Some(o) => o.borrow().get(name),
          None    => None
        }
      }
    }
  }
}


pub fn new_enclosed_environment(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
  let mut env = Environment::new();
  env.outer = Some(outer);
  Rc::new(RefCell::new(env))
}

