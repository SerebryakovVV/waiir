#![allow(dead_code, unused_imports, unused_variables)]

mod object;


use object::Object;

use crate::{ast::{BlockStatement, Expression, Node, Program, Statement}, token::Token};


// static TRUE: &'static Object = &Object::BOOLEAN(true);
// static FALSE: &'static Object = &Object::BOOLEAN(false);
// same for null






// NIGHTTIME CODING SESSION START
trait Evaluable {
  fn eval(self) -> Object;
}

pub fn eval<T: Evaluable>(node: T) -> Object {
  node.eval()
}

impl Evaluable for Expression {
  fn eval(self) -> Object {
    match expr {
      Expression::INT(i)                     => Object::INT(i), 
      Expression::BOOLEAN(b)                 => Object::BOOLEAN(b), 
      Expression::PREFIX { operator, right } => eval_prefix_expression(operator, eval_expression(*right)),
      _                                      => panic!()
  }
  }
}

impl Evaluable for Program {
  fn eval(self) -> Object {
    let mut result = Object::NULL;
    for s in pr.statements {
      result = eval(Node::Statement(s));
    };
    result
  }
}

impl Evaluable for Statement {
  fn eval(self) -> Object {
    match stmt {
      Statement::EXPRESSION(expr) => eval(Node::Expression(expr)),
      _                           => panic!()
    } 
  }
}

impl Evaluable for BlockStatement {
  fn eval(self) -> Object {
    todo!();
  }
}
// NIGHTTIME CODING SESSION END





// eval_program = eval_statements in the book


// TODO: change the enum wrapping to something not retarded
// TODO: add evaluation to repl
// TODO: add the inspect method, this is important
pub fn eval(node: Node) -> Object {
  match node {
    Node::Expression(expr)   => eval_expression(expr),
    Node::Program(pr)        => eval_program(pr),
    Node::Statement(stmt)    => eval_statement(stmt),
    Node::BlockStatement(bs) => eval_block_statement(bs)
  }
}

// TODO: either do normal polymorhpism, or do something with the eval function, why is it even there. I will hink about deleting the node and just having 4 funcitons

fn eval_expression(expr: Expression) -> Object {
  match expr {
    Expression::INT(i)                     => Object::INT(i), 
    Expression::BOOLEAN(b)                 => Object::BOOLEAN(b), // TODO: look into making two objects for the boolean values, cant implement same way as in the book, borrow checker, type mismatch
    Expression::PREFIX { operator, right } => eval_prefix_expression(operator, eval_expression(*right)),
    _                                      => panic!()
  }
}


fn eval_program(pr: Program) -> Object {
  let mut result = Object::NULL;
  for s in pr.statements {
    result = eval(Node::Statement(s));
  };
  result
}


fn eval_statement(stmt: Statement) -> Object {
  match stmt {
    Statement::EXPRESSION(expr) => eval(Node::Expression(expr)),
    _                           => panic!()
  } 
}

fn eval_block_statement(bs: BlockStatement) -> Object {
  todo!()
}



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










#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eval_int() {
    let num = Node::Expression(Expression::INT(563));
    let num_evaled = eval(num);
    assert_eq!(num_evaled, Object::INT(563))
  }

  #[test]
  fn test_eval_statements() {
    let mut pr = Program::new();
    pr.statements = vec![Statement::EXPRESSION(Expression::INT(23)), Statement::EXPRESSION(Expression::INT(2)), Statement::EXPRESSION(Expression::INT(4))];
    assert_eq!(eval(Node::Program(pr)), Object::INT(4))
  }

}