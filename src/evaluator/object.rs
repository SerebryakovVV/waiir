#![allow(dead_code, unused_imports, unused_variables, unreachable_patterns)]

use std::{cell::RefCell, collections::BTreeMap, fmt::{write, Display}, rc::Rc};
use crate::evaluator::builtin::BuiltIn;

use crate::ast::{BlockStatement, Identifier};

use super::environment::Environment;

// TODO: maybe will add objecttype and inspect, maybe not
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  INT(i32),
  STRING(String),
  BOOLEAN(bool),
  ARRAY(Vec<Object>),
  BUILTIN(BuiltIn),
  RETURN(Box<Object>),
  ERROR(String),  // TODO: check lifetimes
  HASH(BTreeMap<HashKey, Object>),
  FUNCTION {
    parameters: Vec<Identifier>,
    body: BlockStatement,
    env: Rc<RefCell<Environment>>
  },
  NULL
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HashableTypes {STRING, INT, BOOLEAN}

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Clone)]
pub struct HashKey {
  tp: HashableTypes,
  value: u32
}

pub trait Hashable {fn my_hash(&self) -> HashKey;}



impl Hashable for bool {
  fn my_hash(&self) -> HashKey {
    if *self {HashKey{tp:HashableTypes::BOOLEAN, value:1}} else {HashKey{tp:HashableTypes::BOOLEAN, value:0}}
  }
}


impl Hashable for i32 {
  fn my_hash(&self) -> HashKey {
    HashKey {tp:HashableTypes::INT, value: *self as u32}
  } 
}


impl Hashable for String {
  fn my_hash(&self) -> HashKey {
    const FNV_OFFSET_BASIS: u32 = 0x811c9dc5; // straight up from chatgpt, no idea whats happening
    const FNV_PRIME: u32 = 0x01000193;

        let mut hash = FNV_OFFSET_BASIS;
        for byte in self.as_bytes() {
            hash ^= *byte as u32;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        HashKey {tp:HashableTypes::STRING,  value: hash }
  } 
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
        Object::STRING(s)    => write!(f, "{}", s),
        Object::BUILTIN(_)   => write!(f, "builtin"),   // TODO: i need something like a todo!, but without panicking 
        Object::ARRAY(els)   => {
                                  for el in els {
                                    write!(f, "{}\n", el)?
                                  };
                                  Ok(())
                                },
                                _=>write!(f, "not implemented")
      }
  }
}


#[test]
fn test_hash() {
  assert_eq!(true.my_hash(), true.my_hash());
  assert_ne!(true.my_hash(), false.my_hash());
  assert_eq!(true.my_hash().value, 1 as u32);
  assert_eq!(String::from("one"), String::from("one"));
  assert_ne!(String::from("one"), String::from("two"));
  assert_eq!(1.my_hash(), 1.my_hash());
  assert_ne!(1.my_hash(), 3424.my_hash());
}