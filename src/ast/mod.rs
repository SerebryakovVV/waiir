use crate::token::Token;

// both enums should have TokenLiteral() -> String

pub enum Statement {
  LET {
    name: String,
    value: Expression
  },
}



pub enum Expression {
  IDENTIFIER(String),
}




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