use crate::evaluator::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltIn {
  LEN
}


impl BuiltIn {
  pub fn look_up_by_ident(ident: &str) -> Option<BuiltIn> {  // this is probably not optimal, i return an object with builtin variant in it, and then match on it(?), can match on the str from the beginning
    match ident {
      "len" => Some(BuiltIn::LEN),
      _     => None
    }
  }

  // this one will match on self and call the needed function with given array of arguments
  pub fn apply(&self, args: Vec<Object>) -> Object {
    match *self {
      BuiltIn::LEN => self.apply_len(args),
      _ => Object::NULL
    }
  }

  fn apply_len(&self, args: Vec<Object>) -> Object {
    if args.len() != 1 {
      return Object::ERROR(String::from("len() takes one argument"))
    };
    // if !matches!(args[0], Object::STRING(_)) { // TODO: note, useful macro
    //   return Object::ERROR(String::from("len() only takes string argument"));
    // };

    match &args[0] {
      Object::STRING(s) => Object::INT(s.len() as i32),
      _ => Object::ERROR(String::from("len() only takes string argument"))
    }

  }
}


// TODO: note, matches! macro