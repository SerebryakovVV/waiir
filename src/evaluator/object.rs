#![allow(dead_code, unused_imports, unused_variables, unreachable_patterns)]

use std::fmt::Display;

// TODO: maybe will add objecttype and inspect, maybe not
#[derive(Debug, PartialEq)]
pub enum Object {
  INT(i32),
  BOOLEAN(bool),
  RETURN(Box<Object>),
  NULL
}


impl Display for Object {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Object::BOOLEAN(b) => write!(f, "{}", if *b {"true"} else {"false"}),        
        Object::INT(i)     => write!(f, "{}", i),
        Object::NULL       => write!(f, "null"),
        _                  => todo!()
      }
  }
}