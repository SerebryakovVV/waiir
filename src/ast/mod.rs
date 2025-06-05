// TODO: clean up all these implemenations


#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::BTreeMap;


use crate::token::Token;

// TODO: add some kind of displaying

// we have statement, expression, program and blockstatement which can be evaluated

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Statement(Statement),
  Expression(Expression),
  Program(Program),
  BlockStatement(BlockStatement)
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Statement {
  LET {
    name: Identifier,
    value: Expression
  },
  RETURN {
    value: Expression
  },
  EXPRESSION(Expression),
  // BLOCK(Vec<Statement>)
}

pub enum PrefixOperator {
  NOT,
  NEG
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Expression {
  IDENT(Identifier),
  INT(i32),
  STRING(String),
  ARRAY(Vec<Expression>),
  INDEX {
    left: Box<Expression>,
    index: Box<Expression>
  },
  HASH(BTreeMap<Expression, Expression>),
  PREFIX {
    operator:Token, 
    right:Box<Expression>
  },
  INFIX {
    left: Box<Expression>, 
    operator: Token, 
    right: Box<Expression>
  },
  BOOLEAN(bool),
  IF {
    condition: Box<Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>
  },
  FUNCTION {
    parameters: Vec<Identifier>,
    body: BlockStatement
  },
  CALL {
    function: Box<Expression>,
    arguments: Vec<Expression>
  },
  DUMMY
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub struct BlockStatement {
  pub statements: Vec<Statement>
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub struct Identifier {pub value:String}


#[derive(Debug, PartialEq, Clone)]
pub struct Program {
  pub statements: Vec<Statement>
}

impl Program {
  pub fn new() -> Self {
    Self {
      statements: Vec::new()
    }
  }
}