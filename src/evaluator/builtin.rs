use crate::evaluator::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltIn {
  LEN,
  FIRST,
  LAST,
  REST,
  PUSH,
  PUTS
}


impl BuiltIn {
  pub fn look_up_by_ident(ident: &str) -> Option<BuiltIn> {  // this is probably not optimal, i return an object with builtin variant in it, and then match on it(?), can match on the str from the beginning
    match ident {
      "len" => Some(BuiltIn::LEN),
      "first" => Some(BuiltIn::FIRST),                       // also, why is every builtin has to go through (..)->Object, kinda... ok if all the stuff in this language is objects then maybe no that weird 
      "last" => Some(BuiltIn::LAST),
      "rest" => Some(BuiltIn::REST),
      "push" => Some(BuiltIn::PUSH),
      "puts" => Some(BuiltIn::PUTS),
      _     => None
    }
  }

  // this one will match on self and call the needed function with given array of arguments
  pub fn apply(&self, args: Vec<Object>) -> Object {
    match *self {
      BuiltIn::LEN => self.apply_len(args),
      BuiltIn::FIRST => self.apply_first(args),
      BuiltIn::LAST => self.apply_last(args),
      BuiltIn::REST => self.apply_rest(args),
      BuiltIn::PUSH => self.apply_push(args),
      BuiltIn::PUTS => self.apply_puts(args),
      _ => Object::NULL
    }
  }

  fn apply_puts(&self, args: Vec<Object>) -> Object {
    for a in args {
      print!("{} ", a)
    };
    Object::NULL
  }

  fn apply_push(&self, args: Vec<Object>) -> Object {
    match &args[0] {
      Object::ARRAY(a)  => {
        match args.get(1) {
          Some(v) => {
            let mut new_arr = a.clone();
            new_arr.push(v.clone());
            Object::ARRAY(new_arr)
          } ,
          None => Object::ERROR(String::from("no second argument"))
        
        }
      }, 
      _ => Object::ERROR(String::from("push() only takes array as first argument"))
    }
  }

  fn apply_rest(&self, args: Vec<Object>) -> Object {
          if args.len() != 1 {
      return Object::ERROR(String::from("rest() takes one argument"))
    };
    match &args[0] {
      // Object::STRING(s) => Object::INT(s.len() as i32),

      Object::ARRAY(a)  => Object::ARRAY(a[1..].to_vec()), 
      _ => Object::ERROR(String::from("rest() only takes array argument"))
    }
  }

  fn apply_last(&self, args: Vec<Object>) -> Object {
      if args.len() != 1 {
      return Object::ERROR(String::from("last() takes one argument"))
    };
    match &args[0] {
      // Object::STRING(s) => Object::INT(s.len() as i32),
      Object::ARRAY(a)  => match a.last() {None => Object::NULL, Some(el) => el.clone()}, 
      _ => Object::ERROR(String::from("last() only takes array argument"))
    }
  }

  fn apply_first(&self, args: Vec<Object>) -> Object {
    if args.len() != 1 {
      return Object::ERROR(String::from("first() takes one argument"))
    };
    match &args[0] {
      // Object::STRING(s) => Object::INT(s.len() as i32),
      Object::ARRAY(a)  => a[0].clone(), // TODO: this whole project needs to be rewritten, why am i cloning everything, this doesn't even make sense, why all these functions return a clone and not the value in the array
      _ => Object::ERROR(String::from("first() only takes array argument"))
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
      Object::ARRAY(a)  => Object::INT(a.len() as i32),
      _ => Object::ERROR(String::from("len() only takes string and array arguments"))
    }

  }
}


// TODO: note, matches! macro