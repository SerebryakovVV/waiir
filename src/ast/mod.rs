use crate::token::Token;

// both enums should have TokenLiteral() -> String

#[derive(Debug, PartialEq)]
pub enum Statement {
  LET {
    name: Identifier,
    value: Expression
  },
  RETURN {
    value: Expression
  },
  EXPRESSION(Expression)
}


#[derive(Debug, PartialEq)]
pub enum Expression {
  IDENT(Identifier),
  DUMMY
}


#[derive(Debug, PartialEq)]
pub struct Identifier {pub value:String}


#[derive(Debug, PartialEq)]
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