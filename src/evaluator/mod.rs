#![allow(dead_code, unused_imports, unused_variables)]

mod object;
pub mod environment;
mod builtin;

use builtin::BuiltIn;

use std::{cell::RefCell, collections::BTreeMap, hash::Hash, rc::Rc, sync::Once};

use environment::{new_enclosed_environment, Environment};
use object::Object;


use crate::{ast::{BlockStatement, Expression, Identifier, Node, Program, Statement}, evaluator::object::{HashKey, Hashable}, token::Token};


// static TRUE: &'static Object = &Object::BOOLEAN(true);
// static FALSE: &'static Object = &Object::BOOLEAN(false);
// same for null


// eval_program = eval_statements in the book


// TODO: change the enum wrapping to something not retarded
// TODO: add evaluation to repl
// TODO: add the inspect method, this is important
// TODO: error messages with parameters




// the final boss of code formatting
impl Expression {
  fn eval(self, env: Rc<RefCell<Environment>>) -> Object {
    match self {
      Expression::INT(i)                                   => Object::INT(i),
      Expression::STRING(s)                                => Object::STRING(s),
      Expression::BOOLEAN(b)                               => Object::BOOLEAN(b), // TODO: look into making two objects for the boolean values, cant implement same way as in the book, borrow checker, type mismatch
      Expression::IDENT(i)                                 => eval_identifier_expression(i, env),
      Expression::PREFIX {operator, right}                 => {
                                                                // let val = eval(*right, env);
                                                                let val = (*right).eval(Rc::clone(&env));
                                                                if is_error(&val) {
                                                                  return val;
                                                                }
                                                                eval_prefix_expression(operator, val)
                                                              },
      Expression::INFIX {left, operator, right}            => {
                                                                let val_left = (*left).eval(Rc::clone(&env));
                                                                if is_error(&val_left) {
                                                                  return val_left;
                                                                }
                                                                let val_right = (*right).eval(Rc::clone(&env));
                                                                if is_error(&val_right) {
                                                                  return val_right;
                                                                }
                                                                eval_infix_expression(val_left, operator, val_right)
                                                              },
      Expression::IF {condition, consequence, alternative} => eval_if_expression(condition, consequence, alternative, env),
      Expression::FUNCTION {parameters, body}              => Object::FUNCTION {parameters, body, env},
      
      Expression::CALL {function, arguments}               => {
                                                                let func = function.eval(Rc::clone(&env));
                                                                if is_error(&func) {
                                                                  return func
                                                                };

                                                                let args = eval_expressions(arguments, Rc::clone(&env));

                                                                if let Err(e) = args {
                                                                  return e;
                                                                } else {
                                                                  return apply_function(func, args.unwrap());
                                                                }
                                                              },

      Expression::DUMMY                                    => {println!("dummy"); Object::NULL}, //  TODO: get rid of this one
      Expression::ARRAY(v)                                 => {
                                                                // println!("{:#?}", v); Object::NULL
                                                                // let elements = eval_expressions(v, env);
                                                                match eval_expressions(v, Rc::clone(&env)) {
                                                                  Ok(els)  => Object::ARRAY(els),
                                                                  Err(err) => err
                                                                }
                                                              },  // add eval for these two
      Expression::INDEX {left, index}                      => {
                                                                // println!("index expr eval function"); Object::NULL
                                                                let arr_left = left.eval(Rc::clone(&env));
                                                                if is_error(&arr_left) {return arr_left};
                                                                let arr_index = index.eval(Rc::clone(&env));
                                                                if is_error(&arr_index) {return arr_index};
                                                                eval_index_expression(arr_left, arr_index)
                                                                // Object::INT(1)
                                                              }        , 
      Expression::HASH(h) => eval_hash_literal(h, Rc::clone(&env)),
                                                              _=> Object::NULL
    }
  }
}



// this is the most retarded code in this project, i have no idea what is happening here, its 04:47 right now
fn eval_hash_literal(hsh: BTreeMap<Expression, Expression>, env: Rc<RefCell<Environment>>) -> Object {
  let mut pairs = BTreeMap::<HashKey, Object>::new();
  for (k, v) in hsh {
    // let fin_key = k.eval(Rc::clone(&env));
    // if is_error(&fin_key) {
    //   return Object::ERROR("some error".to_string());
    // };
    // let mut hashed_key = 
    // match &v {
    //   Expression::BOOLEAN(b) => b.my_hash(), 
    //   Expression::INT(i)  => i.my_hash(),
    //    Expression::STRING(s)  => s.my_hash(),// hash it here,
    //   _ => return Object::ERROR("some other error hash".to_string())
    // };
    // let fin_val = v.eval(Rc::clone(&env));
    // if is_error(&fin_val) {
    //   return Object::ERROR("some another error".to_string());
    // }
    // pairs.insert(hashed_key, fin_val);

    let fin_key = k.eval(Rc::clone(&env));
    if is_error(&fin_key) {
      return Object::ERROR("some error".to_string());
    };
    let mut hashed_key = 
    match fin_key {
      Object::BOOLEAN(b) => b.my_hash(), 
      Object::INT(i)  => i.my_hash(),
       Object::STRING(s)  => s.my_hash(),// hash it here,
      _ => return Object::ERROR("some other error hash".to_string())
    };
    let fin_val = v.eval(Rc::clone(&env));
    if is_error(&fin_val) {
      return Object::ERROR("some another error".to_string());
    }
    pairs.insert(hashed_key, fin_val);

    
  }
  Object::HASH(pairs)
}

fn eval_index_expression(left: Object, index: Object) -> Object {
  match (left, &index) {
    (Object::ARRAY(els), Object::INT(i)) => eval_array_index_expression(els, *i),
    (Object::HASH(hsh), _)  => eval_hash_index_expression(hsh, index),
    _                                    => Object::ERROR(String::from("Wrong array indexing expression"))
  }
}


fn eval_hash_index_expression(hsh: BTreeMap<HashKey, Object>, index: Object) -> Object {
  let key = match index {
    Object::INT(i) => i.my_hash(),
    Object::BOOLEAN(b) => b.my_hash(),
    Object::STRING(s) => s.my_hash(),
    _ => return Object::ERROR("non hashable".to_string())
  };

  // if let Object::HASH(h) = hsh {
  //   match h.get(&key) {
  //     Some(v) => v.clone(),
  //     None => Object::NULL
  //   } 
  // } else {
  //   Object::ERROR("idk, some error".to_string())
  // }


    println!("{:#?}", key);

    match hsh.get(&key) {
      Some(v) => v.clone(),
      None => {println!("error here");Object::NULL}
    } 

  
}

fn eval_array_index_expression(els: Vec<Object>, index: i32) -> Object {
  if index < 0 || index > els.len() as i32 - 1 {
    return Object::ERROR(String::from("index out of bounds"));
  };
  els[index as usize].clone()  // TODO: :(
}


fn apply_function(func: Object, args: Vec<Object>) -> Object {


  match func {
    Object::FUNCTION {parameters, body, env} => {
      let extended_env = extend_function_env(parameters, env, args);
      let evaluated = body.eval(extended_env);
      return unwrap_return_value(evaluated);
    },
    Object::BUILTIN(b) => {
      b.apply(args)
    },
    _ => return Object::ERROR(String::from("Not a function"))
  }



  // if let Object::FUNCTION { parameters, body, env } = func {
  //   let extended_env = extend_function_env(parameters, env, args);
  //   let evaluated = body.eval(extended_env);
  //   return unwrap_return_value(evaluated);
  // } else {
  //   return Object::ERROR(String::from("Not a function"));
  // }
}

fn extend_function_env(params: Vec<Identifier>, env: Rc<RefCell<Environment>>, args: Vec<Object>) -> Rc<RefCell<Environment>> {
  let new_env = new_enclosed_environment(env);
  for (i, p) in params.iter().enumerate() {
    new_env.borrow_mut().set(p.value.clone(), args[i].clone());
  };
  new_env
}

fn unwrap_return_value(obj: Object) -> Object {
  match obj {
    Object::RETURN(r) => *r,
    _                 => obj
  } 
}

fn eval_expressions(exprs: Vec<Expression>, env: Rc<RefCell<Environment>>) -> Result<Vec<Object>, Object> {
  let mut res = Vec::<Object>::new();
  for e in exprs {
    let evaled_expr = e.eval(Rc::clone(&env));
    if is_error(&evaled_expr) {
      return Err(evaled_expr);
    }
    res.push(evaled_expr);
  };
  Ok(res)
}


impl Statement {
  fn eval(self, env: Rc<RefCell<Environment>>) -> Object {
    match self {
      Statement::EXPRESSION(expr)  => expr.eval(Rc::clone(&env)),
      Statement::RETURN {value}    => {
                                        let val = value.eval(Rc::clone(&env));
                                        if is_error(&val) {
                                          return val;
                                        } else {
                                          return Object::RETURN(Box::new(val));
                                        }
                                      },
      Statement::LET {name, value} => {
                                        let val = value.eval(Rc::clone(&env));
                                        if is_error(&val) {
                                          return val
                                        } else {
                                          env.borrow_mut().set(name.value, val.clone()); // TODO: clone
                                          val
                                        }
                                      }
    } 
  }
}


impl Program {  // this is eval_statements
  pub fn eval(self, env: Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::NULL;
    for s in self.statements {
      result = s.eval(Rc::clone(&env));
      match result {
        Object::RETURN(r) => return *r,
        Object::ERROR(_)  => return result,
        _                 => continue
      }
    };
    result
  }
}


impl BlockStatement {
  fn eval(self, env: Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::NULL;
    for s in self.statements {
      result = s.eval(Rc::clone(&env));
      match result {
        Object::RETURN(_) | Object::ERROR(_) => return result,
        _                                    => continue
      };
    };
    result
  }
}





fn is_error(obj: &Object) -> bool {
  match obj {
    Object::ERROR(_) => true,
    _                => false 
  }
} 


fn eval_identifier_expression(ident: Identifier, env: Rc<RefCell<Environment>>) -> Object {
  match env.borrow_mut().get(ident.value.clone()) {
    Some(v) => v,
    None    => {
      // not found in env, try builtins
      match BuiltIn::look_up_by_ident(&ident.value) {
        Some(b) => Object::BUILTIN(b),
        None    => Object::ERROR(String::from("Identifier not found"))
      }
    }
  }
}



fn eval_if_expression(condition: Box<Expression>, consequence: BlockStatement, alternative: Option<BlockStatement>, env: Rc<RefCell<Environment>>) -> Object {
  let evaluated_condition = (*condition).eval(Rc::clone(&env));
  if is_error(&evaluated_condition) {
    return evaluated_condition;
  }
  match (is_truthy(evaluated_condition), alternative) {  // look at this sexy pattern matching, oh my god! can your go do this? hmmm??
    (true, _)          => consequence.eval(Rc::clone(&env)),
    (false, Some(alt)) => alt.eval(env),
    (false, None)      => Object::NULL
  }
}



fn is_truthy(condition: Object) -> bool {
  match condition {
    Object::NULL       => false,
    Object::BOOLEAN(b) => if b {true} else {false},
    _                  => true
  }
}

fn eval_infix_expression(left: Object, operator: Token, right: Object) -> Object {
  match (left, right) {
    (Object::INT(l), Object::INT(r))         => eval_integer_infix_expression(l, operator, r),
    (Object::BOOLEAN(l), Object::BOOLEAN(r)) => eval_boolean_infix_expression(l, operator, r), 
    (Object::STRING(sl), Object::STRING(sr)) => eval_string_infix_expression(sl, operator, sr),
    _                                        => Object::ERROR(String::from("Infix expression operands aren't ints or booleans"))
  }
}

fn eval_string_infix_expression(left: String, operator: Token, right: String) -> Object {
  match operator {
    Token::PLUS => Object::STRING(String::from(left + &right)),
    _           => Object::ERROR(String::from("Unsupported string infix operator"))
  }
}

fn eval_integer_infix_expression(left: i32, operator: Token, right: i32) -> Object {
  match operator {
    Token::PLUS     => Object::INT(left + right),
    Token::MINUS    => Object::INT(left - right),
    Token::ASTERISK => Object::INT(left * right),
    Token::SLASH    => if right == 0 {Object::ERROR(String::from("Zero division error"))} else {Object::INT(left/right)},
    Token::GT       => Object::BOOLEAN(left > right),
    Token::LT       => Object::BOOLEAN(left < right),
    Token::EQ       => Object::BOOLEAN(left == right),
    Token::NOTEQ    => Object::BOOLEAN(left != right),
    _               => Object::ERROR(String::from("Unknown operator for integer infix expression"))
  }
}

fn eval_boolean_infix_expression(left: bool, operator: Token, right: bool) -> Object {
  match operator {
      Token::EQ    => Object::BOOLEAN(left == right),
      Token::NOTEQ => Object::BOOLEAN(left != right),
      _            => Object::ERROR(String::from("Unknown operator for boolean infix expression"))
  }
}

fn eval_prefix_expression(operator: Token, right: Object) -> Object {
  match operator {
    Token::BANG  => eval_bang_operator_expression(right),
    Token::MINUS => eval_minus_prefix_operator_expression(right),
    _            => Object::ERROR(String::from("Unknown prefix operator"))
  }
  
}

fn eval_bang_operator_expression(right: Object) -> Object {
  match right {
    Object::BOOLEAN(b) => if b {Object::BOOLEAN(false)} else {Object::BOOLEAN(true)},
    Object::NULL       => Object::BOOLEAN(true),
    _                  => Object::BOOLEAN(false)
  }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
  match right {
    Object::INT(i) => Object::INT(-i),
    _              => Object::ERROR(String::from("Only ints are supported as prefix minus operand"))      
  }
}










// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_eval_int() {
//     let num = Node::Expression(Expression::INT(563));
//     let num_evaled = eval(num);
//     assert_eq!(num_evaled, Object::INT(563))
//   }

//   #[test]
//   fn test_eval_statements() {
//     let mut pr = Program::new();
//     pr.statements = vec![Statement::EXPRESSION(Expression::INT(23)), Statement::EXPRESSION(Expression::INT(2)), Statement::EXPRESSION(Expression::INT(4))];
//     assert_eq!(eval(Node::Program(pr)), Object::INT(4))
//   }

// }