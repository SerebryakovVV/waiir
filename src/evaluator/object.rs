#![allow(dead_code, unused_imports, unused_variables, unreachable_patterns)]

use std::{cell::RefCell, fmt::{write, Display}, rc::Rc};

use crate::ast::{BlockStatement, Identifier};

use super::environment::Environment;

// TODO: maybe will add objecttype and inspect, maybe not
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  INT(i32),
  BOOLEAN(bool),
  RETURN(Box<Object>),
  ERROR(String),  // TODO: check lifetimes
  FUNCTION {
    parameters: Vec<Identifier>,
    body: BlockStatement,
    env: Rc<RefCell<Environment>>
  },
  NULL
}


impl Display for Object {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Object::BOOLEAN(b)   => write!(f, "{}", if *b {"true"} else {"false"}),        
        Object::INT(i)       => write!(f, "{}", i),
        Object::NULL         => write!(f, "null"),
        Object::ERROR(e)     => write!(f, "{}", e),
        Object::RETURN(_)    => write!(f, "return"),
        Object::FUNCTION{..} => write!(f, "return"),
        
        // _                  => todo!()
      }
  }
}