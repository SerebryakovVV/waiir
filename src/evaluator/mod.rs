#![allow(dead_code, unused_imports, unused_variables)]

mod object;
pub mod environment;

use std::{cell::RefCell, rc::Rc, sync::Once};

use environment::{new_enclosed_environment, Environment};
use object::Object;

use crate::{ast::{BlockStatement, Expression, Identifier, Node, Program, Statement}, token::Token};


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

      _                                                    => panic!()
  }
  }
}


fn apply_function(func: Object, args: Vec<Object>) -> Object {
  if let Object::FUNCTION { parameters, body, env } = func {
    let extended_env = extend_function_env(parameters, env, args);
    let evaluated = body.eval(extended_env);
    return unwrap_return_value(evaluated);
  } else {
    return Object::ERROR(String::from("Not a function"));
  }
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
                                      },
      _                            => panic!()
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
  match env.borrow_mut().get(ident.value) {
    Some(v) => v,
    None    => Object::ERROR(String::from("Identifier not found"))
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
    _                                        => Object::ERROR(String::from("Infix expression operands aren't ints or booleans"))
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