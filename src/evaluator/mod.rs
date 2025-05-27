
mod object;


use object::Object;

use crate::{ast::{BlockStatement, Expression, Node, Program, Statement}, token::Token};


// static TRUE: &'static Object = &Object::BOOLEAN(true);
// static FALSE: &'static Object = &Object::BOOLEAN(false);
// same for null






// TODO: change the enum wrapping to something not retarded
// TODO: add evaluation to repl
// TODO: add the inspect method, this is important
pub fn eval(node: Node) -> Object {
  // pattern match all that stuff here
  match node {
    Node::Expression(expr)   => {
                                  match expr {
                                    Expression::INT(i)                     => Object::INT(i), 
                                    Expression::BOOLEAN(b)                 => Object::BOOLEAN(b), // TODO: look into making two objects for the boolean values, cant implement same way as in the book, borrow checker, type mismatch
                                    Expression::PREFIX { operator, right } => eval_prefix_expression(operator, eval(right)),
                                    _                                      => panic!()
                                  }
    },
    Node::Program(pr)        => eval_statements(pr),
    Node::Statement(stmt)    => {
                                  match stmt {
                                    Statement::EXPRESSION(expr) => eval(Node::Expression(expr)),
                                    _                           => panic!()
                                  }
    },
    Node::BlockStatement(bs) => panic!(),
  }
}


fn eval_statements(pr: Program) -> Object {
  let mut result = Object::NULL;
  for s in pr.statements {
    result = eval(Node::Statement(s));
  };
  result
}

fn eval_prefix_expression(operator: Token, right: Object) -> Object {

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