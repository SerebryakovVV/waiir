#![allow(dead_code, unused_imports, unused_variables)]

mod object;


use object::Object;

use crate::{ast::{BlockStatement, Expression, Node, Program, Statement}, token::Token};


// static TRUE: &'static Object = &Object::BOOLEAN(true);
// static FALSE: &'static Object = &Object::BOOLEAN(false);
// same for null







trait Evaluable {
  fn eval(self) -> Object;
}

pub fn eval<T: Evaluable>(node: T) -> Object {   // TODO: i dont need neither trait nor this generic, unless i want to add something to it. Will leave for now
  node.eval()
}

impl Evaluable for Expression {
  fn eval(self) -> Object {
    match self {
      Expression::INT(i)                     => Object::INT(i), 
      Expression::BOOLEAN(b)                 => Object::BOOLEAN(b), // TODO: look into making two objects for the boolean values, cant implement same way as in the book, borrow checker, type mismatch
      Expression::PREFIX { operator, right } => eval_prefix_expression(operator, eval(*right)),
      _                                      => panic!()
  }
  }
}

impl Evaluable for Program {
  fn eval(self) -> Object {
    let mut result = Object::NULL;
    for s in self.statements {
      result = eval(s);
    };
    result
  }
}

impl Evaluable for Statement {
  fn eval(self) -> Object {
    match self {
      Statement::EXPRESSION(expr) => eval(expr),
      _                           => panic!()
    } 
  }
}

impl Evaluable for BlockStatement {
  fn eval(self) -> Object {
    todo!();
  }
}






// eval_program = eval_statements in the book


// TODO: change the enum wrapping to something not retarded
// TODO: add evaluation to repl
// TODO: add the inspect method, this is important












fn eval_prefix_expression(operator: Token, right: Object) -> Object {
  match operator {
    Token::BANG  => eval_bang_operator_expression(right),
    Token::MINUS => eval_minus_prefix_operator_expression(right),
    _            => Object::NULL
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
    _              => Object::NULL      
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