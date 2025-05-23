use core::panic;

use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Expression, Program, Statement, Identifier};



const LOWEST: usize = 1;
const EQUALS: usize = 2;
const LESSGREATER: usize = 3;
const SUM: usize = 4;
const PRODUCT: usize = 5;
const PREFIX: usize = 6;
const CALL: usize = 7;



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
      _             => return self.parse_expression_statement()
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

  fn parse_expression_statement(&mut self) -> Option<Statement> {
    let expr = self.parse_expression(LOWEST);
    if self.peek_token_is(Token::SEMICOLON) {
      self.next_token();
    }
    Some(Statement::EXPRESSION(expr))
  }

  fn parse_expression(&mut self, precedence: usize) -> Expression {
    // for now - no idea what that guy was trying to do, he has no checks for nil after calls to this function anywhere
    let mut left_expression = match &self.current_token {
      Token::IDENT(s) => Expression::IDENT(Identifier { value: s.clone() }),
      Token::INT(i)   => Expression::INT(*i),
      Token::BANG     => self.parse_prefix_expression(),
      Token::MINUS    => self.parse_prefix_expression(),
      _               => Expression::DUMMY // TODO: errors, im putting this here, because i have no idea what im doing
    };
    
    while !self.peek_token_is(Token::SEMICOLON) && precedence < self.peek_precedence() {
      match self.peek_token {
        Token::PLUS     |
        Token::MINUS    |
        Token::SLASH    |
        Token::ASTERISK |
        Token::EQ       |
        Token::NOTEQ    |
        Token::LT       |
        Token::GT        => {
                              self.next_token();
                              left_expression = self.parse_infix_expression(left_expression);
                            },
        _                => return left_expression
      };
    }
    // TODO:  noPrefixParseFnError
    left_expression
  }

  fn parse_prefix(&mut self) -> Expression {
    
    let expr = match &self.current_token {
      Token::IDENT(s) => Expression::IDENT(Identifier { value: s.clone() }),
      Token::INT(i)   => Expression::INT(*i),
      Token::BANG     => self.parse_prefix_expression(),
      Token::MINUS    => self.parse_prefix_expression(),
      _               => panic!() // TODO: errors
    };
    println!("it do be parsing idents");
    expr
  }

  fn parse_prefix_expression(&mut self) -> Expression {
    let operator = self.current_token.clone();
    self.next_token();
    let right = self.parse_expression(PREFIX);
    Expression::PREFIX { operator, right: Box::new(right) }
  }

  // TODO:
  // this part is weird, for now
  // do i even need it?
  fn parse_infix(&mut self, left: Expression) -> Expression {
    let expr = match self.current_token {
      Token::PLUS     |
      Token::MINUS    |
      Token::SLASH    |
      Token::ASTERISK |
      Token::EQ       |
      Token::NOTEQ    |
      Token::LT       |
      Token::GT        => self.parse_infix_expression(left),
      _                => panic!()
    };
    expr
  }

  fn parse_infix_expression(&mut self, left: Expression) -> Expression {
    let operator = self.current_token.clone();
    let new_expr_left = left;
    let precedence = self.current_precedence();
    self.next_token();
    let new_expr_right = self.parse_expression(precedence);
    let expr = Expression::INFIX {
      left: Box::new(new_expr_left), 
      operator: operator, 
      right: Box::new(new_expr_right)
    };
    expr
  }

  // TODO: check on identifier struct and why have i even added it
  fn parse_ident(&self) -> Expression {
    // not needed, i inlined it into parse_prefix, will clean later
    unreachable!()
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
    // TODO:
    // self.errors.push(format!("Expected next token to be {}, got {}", token, self.peek_token)); // TODO: implement Display for Token
    todo!()
  }

  fn get_precedence(&self, tkn: Token) -> usize {
    match tkn {
      Token::EQ    | Token::NOTEQ    => EQUALS,
      Token::LT    | Token::GT       => LESSGREATER,
      Token::PLUS  | Token::MINUS    => SUM,
      Token::SLASH | Token::ASTERISK => PRODUCT,
      _                              => LOWEST  // TODO: add the errors
    }
  }

  fn peek_precedence(&self) -> usize {
    self.get_precedence(self.peek_token.clone())
  }

  fn current_precedence(&self) -> usize {
    self.get_precedence(self.current_token.clone())
  }
}