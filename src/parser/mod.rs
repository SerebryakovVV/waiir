use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Expression, Program, Statement, Identifier};

pub struct Parser {
  lexer: Lexer,
  errors: Vec<String>,
  pub current_token: Token,
  pub peek_token: Token
}



impl Parser {
  pub fn new(source: &str) -> Self {
    let mut parser = Self {
      lexer: Lexer::new(source),
      errors: Vec::new(),
      current_token: Token::EOF,
      peek_token: Token::EOF
    };
    parser.next_token();
    parser.next_token();
    parser
  }

  fn next_token(&mut self) {
    self.current_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    let mut program = Program::new();
    while self.current_token != Token::EOF {
      let stmt = self.parse_statement();
      if let Some(s) = stmt {
        program.statements.push(s);
      }
      self.next_token();
    }
    program
  }

  fn parse_statement(&mut self) -> Option<Statement> {
    println!("{:?}", self.current_token);
    match self.current_token {
      Token::LET    => return self.parse_let_statement(),
      Token::RETURN => return self.parse_return_statement(),
      _          => panic!()
    }
  }
  

  fn parse_let_statement(&mut self) -> Option<Statement> {
    let name: Identifier;
    if let Token::IDENT(s) = &self.peek_token {
      name = Identifier{value:s.clone()};
      self.next_token();
    } else {
      return None;
    }
    let statement = Statement::LET { name, value: Expression::DUMMY }; 
    if !self.expect_peek(Token::ASSIGN) {
      return None;
    }
    while self.current_token != Token::SEMICOLON {
      self.next_token();
    }
    Some(statement)
  }

  fn parse_return_statement(&mut self) -> Option<Statement> {
    while !self.current_token_is(Token::SEMICOLON) {
      self.next_token();
    }
    let statement = Statement::RETURN { value: Expression::DUMMY };
    Some(statement)
  }

  fn expect_peek(&mut self, token: Token) -> bool {
    println!("entered expect peek, token={:?}, self.peek_token={:?}", token, self.peek_token);
    if self.peek_token_is(token) {
      self.next_token();
      true
    } else {
      // peek_error()    // TODO:
      false
    }
  }

  fn current_token_is(&self, token: Token) -> bool {
    if self.current_token == token {true} else {false} 
  }

  fn peek_token_is(&self, token: Token) -> bool {
    if self.peek_token == token {true} else {false} 
  }

  fn get_errors(&self) -> &Vec<String> {
    &self.errors
  }

  fn peek_error(&self, token: Token) {
    // self.errors.push(format!("Expected next token to be {}, got {}", token, self.peek_token)); // TODO: implement Display for Token
    todo!()
  }
}