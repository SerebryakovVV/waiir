#![allow(dead_code, unused_imports, unused_variables)]

mod object;
pub mod environment;

use std::sync::Once;

use environment::Environment;
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



trait Evaluable {
  fn eval(self, env: &mut Environment) -> Object;
}

pub fn eval<T: Evaluable>(node: T, env: &mut Environment) -> Object {   // TODO: i dont need neither trait nor this generic, unless i want to add something to it. Will leave for now
  node.eval(env)
}

impl Evaluable for Expression {
  fn eval(self, env: &mut Environment) -> Object {
    match self {
      Expression::INT(i)                                   => Object::INT(i), 
      Expression::BOOLEAN(b)                               => Object::BOOLEAN(b), // TODO: look into making two objects for the boolean values, cant implement same way as in the book, borrow checker, type mismatch
      Expression::IDENT(i)                                 => eval_identifier_expression(i, env),
      Expression::PREFIX {operator, right}                 => {
                                                                let val = eval(*right, env);
                                                                if is_error(&val) {
                                                                  return val;
                                                                }
                                                                eval_prefix_expression(operator, val)
                                                              },
      Expression::INFIX {left, operator, right}            => {
                                                                let val_left = eval(*left, env);
                                                                if is_error(&val_left) {
                                                                  return val_left;
                                                                }
                                                                let val_right = eval(*right, env);
                                                                if is_error(&val_right) {
                                                                  return val_right;
                                                                }
                                                                eval_infix_expression(val_left, operator, val_right)
                                                              },
      Expression::IF {condition, consequence, alternative} => eval_if_expression(condition, consequence, alternative, env),
      _                                                    => panic!()
  }
  }
}

impl Evaluable for Program {  // this is eval_statements
  fn eval(self, env: &mut Environment) -> Object {
    let mut result = Object::NULL;
    for s in self.statements {
      result = eval(s, env);
      match result {
        Object::RETURN(r) => return *r,
        Object::ERROR(_)  => return result,
        _                 => continue
      }
    };
    result
  }
}





impl Evaluable for Statement {
  fn eval(self, env: &mut Environment) -> Object {
    match self {
      Statement::EXPRESSION(expr)  => eval(expr, env),
      Statement::RETURN {value}    => {
                                        let val = eval(value, env);
                                        if is_error(&val) {
                                          return val;
                                        } else {
                                          return Object::RETURN(Box::new(val));
                                        }
                                      },
      Statement::LET {name, value} => {
                                        let val = eval(value, env);
                                        if is_error(&val) {
                                          return val
                                        } else {
                                          env.set(name.value, val.clone()); // TODO: clone
                                          val
                                        }
                                      },
      _                            => panic!()
    } 
  }
}

impl Evaluable for BlockStatement {
  fn eval(self, env: &mut Environment) -> Object {
    let mut result = Object::NULL;
    for s in self.statements {
      result = eval(s, env);
      match result {
        Object::RETURN(_) | Object::ERROR(_) => return result,
        _                                    => continue
      };
    };
    result
  }
}


//  func evalBlockStatement(block *ast.BlockStatement) object.Object {
//  var result object.Object
//  for _, statement := range block.Statements {
//  result = Eval(statement)
//  if result != nil && result.Type() == object.RETURN_VALUE_OBJ {
//  return result
//  }
//  }
//  return result
//  }



fn is_error(obj: &Object) -> bool {
  match obj {
    Object::ERROR(_) => true,
    _                => false 
  }
} 


fn eval_identifier_expression(ident: Identifier, env: &mut Environment) -> Object {
  match env.get(ident.value) {
    Some(v) => (*v).clone(), // TODO: clone again...
    None    => Object::ERROR(String::from("Identifier not found"))
  }
}



fn eval_if_expression(condition: Box<Expression>, consequence: BlockStatement, alternative: Option<BlockStatement>, env: &mut Environment) -> Object {
  let evaluated_condition = eval(*condition, env);
  if is_error(&evaluated_condition) {
    return evaluated_condition;
  }
  match (is_truthy(evaluated_condition), alternative) {  // look at this sexy pattern matching, oh my god! can your go do this? hmmm??
    (true, _)          => eval(consequence, env),
    (false, Some(alt)) => eval(alt, env),
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