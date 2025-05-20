use crate::token::Token;

// both enums should have TokenLiteral() -> String

pub enum Statement {
  LET {
    name: Identifier,
    value: Expression
  },
  RETURN {
    value: Expression
  }
}



pub enum Expression {
  IDENT(Identifier),
  DUMMY
}


pub struct Identifier {pub value:String}


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