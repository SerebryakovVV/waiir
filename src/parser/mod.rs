
#![allow(dead_code, unused_imports, unused_variables)]

use core::panic;
use std::env::consts;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{BlockStatement, Expression, Identifier, Program, Statement};


const LOWEST: usize = 1;
const EQUALS: usize = 2;
const LESSGREATER: usize = 3;
const SUM: usize = 4;
const PRODUCT: usize = 5;
const PREFIX: usize = 6;
const CALL: usize = 7;
const INDEX: usize = 7;


pub struct Parser {
  lexer: Lexer,
  errors: Vec<String>,
  pub current_token: Token,
  pub peek_token: Token
}

  // TODO: check on identifier struct and why have i even added it
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
    // println!("{:?}", self.current_token);
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
    if !self.expect_peek(Token::ASSIGN) {
      return None;
    }
    self.next_token();
    let value = self.parse_expression(LOWEST);
    if self.peek_token_is(Token::SEMICOLON) {
      self.next_token();
    }
    Some(Statement::LET {name, value})
  }

  fn parse_return_statement(&mut self) -> Option<Statement> {
    self.next_token();
    let value = self.parse_expression(LOWEST);
    if self.peek_token_is(Token::SEMICOLON) {
      self.next_token();
    }
    Some(Statement::RETURN {
      value
    })
  }

  fn parse_expression_statement(&mut self) -> Option<Statement> {
    let expr = self.parse_expression(LOWEST);
    if self.peek_token_is(Token::SEMICOLON) {
      self.next_token();
    }
    Some(Statement::EXPRESSION(expr))
  }

  fn parse_expression(&mut self, precedence: usize) -> Expression {
    let mut left_expression = match &self.current_token {
      // THIS IS PREFIXES!!!
      Token::IDENT(s)  => Expression::IDENT(Identifier { value: s.clone() }),
      Token::INT(i)    => Expression::INT(*i),
      Token::STRING(s) => Expression::STRING(s.clone()), // TODO: if i have the whole source code as an array, i can rewrite everything to slices and pointers instead of cloning(?)
      Token::BANG      => self.parse_prefix_expression(), // ^ matching on reference right now, look into matching on owned value
      Token::MINUS     => self.parse_prefix_expression(),
      Token::TRUE      => Expression::BOOLEAN(true),
      Token::FALSE     => Expression::BOOLEAN(false),
      Token::LPAREN    => self.parse_grouped_expression(),
      Token::LBRACKET  => self.parse_array_literal(),
      Token::IF        => self.parse_if_expression(),
      Token::FUNCTION  => self.parse_function_literal(),
      // _                => Expression::DUMMY // TODO: errors
      _ => panic!()
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
        Token::LPAREN    => {
                              self.next_token();
                              left_expression = self.parse_call_expression(left_expression);
                            },
        Token::LBRACKET  => {
                              self.next_token();
                              left_expression = self.parse_index_expression(left_expression);
                            },
        _                => return left_expression
      };
    }
    // TODO:  noPrefixParseFnError
    left_expression
  }

  fn parse_index_expression(&mut self, left: Expression) -> Expression {
    self.next_token();
    let index = self.parse_expression(LOWEST);
    if !self.expect_peek(Token::RBRACKET) {
      panic!();
    }
    Expression::INDEX { left: Box::new(left), index: Box::new(index) }
  }

  fn parse_array_literal(&mut self) -> Expression {
    Expression::ARRAY(self.parse_expression_list(Token::RBRACKET))
  }

  fn parse_expression_list(&mut self, end: Token) -> Vec<Expression> {
    let mut list = Vec::new();
    if self.peek_token_is(end.clone()) {
      self.next_token();
      return list;
    };
    self.next_token();
    list.push(self.parse_expression(LOWEST));
    while self.peek_token_is(Token::COMMA) {
      self.next_token();
      self.next_token();
      list.push(self.parse_expression(LOWEST));
    };
    if !self.expect_peek(end) {
      todo!();
    }
    list
  }

  fn parse_prefix_expression(&mut self) -> Expression {
    let operator = self.current_token.clone();
    self.next_token();
    let right = self.parse_expression(PREFIX);
    Expression::PREFIX { operator, right: Box::new(right) }
  }

  fn parse_infix_expression(&mut self, left: Expression) -> Expression {
    let operator = self.current_token.clone();
    let new_expr_left = left;
    let precedence = self.current_precedence();
    self.next_token();
    let new_expr_right = self.parse_expression(precedence);
    Expression::INFIX {
      left: Box::new(new_expr_left), 
      operator: operator, 
      right: Box::new(new_expr_right)
    }
  }

  fn parse_grouped_expression(&mut self) -> Expression {
    self.next_token();
    let expr = self.parse_expression(LOWEST);
    if !self.expect_peek(Token::RPAREN) {
      // return Expression::DUMMY;
      panic!();
    }
    expr
  }

  fn parse_if_expression(&mut self) -> Expression {
    if !self.expect_peek(Token::LPAREN) {
      // return Expression::DUMMY;
      panic!();
    }
    self.next_token();
    let condition = self.parse_expression(LOWEST);
    if !self.expect_peek(Token::RPAREN) {
      // return Expression::DUMMY;
      panic!();
    }
    if !self.expect_peek(Token::LBRACE) {
      // return Expression::DUMMY;
      panic!();
    }
    let consequence = self.parse_block_statement();
    let mut alternative = None;
    if self.peek_token_is(Token::ELSE) {
      self.next_token();
      if !self.expect_peek(Token::LBRACE) {
        // return Expression::DUMMY;
        panic!();
      }
      alternative = Some(self.parse_block_statement());
    }
    Expression::IF {
      condition: Box::new(condition), 
      consequence: consequence, 
      alternative: alternative
    } 
  }

  fn parse_block_statement(&mut self) -> BlockStatement {
    let mut block = BlockStatement {statements: Vec::new()};
    self.next_token();
    while !self.current_token_is(Token::RBRACE) && !self.current_token_is(Token::EOF) {
      let statement = self.parse_statement();
      if let Some(s) = statement {
        block.statements.push(s);
      }
      self.next_token();
    };
    block
  }

  fn parse_function_literal(&mut self) -> Expression {
    if !self.expect_peek(Token::LPAREN) {
      // return Expression::DUMMY;
      panic!();
    }
    let parameters = self.parse_function_parameters();
    if !self.expect_peek(Token::LBRACE) {
      // return Expression::DUMMY;
      panic!();
    }
    let body = self.parse_block_statement();
    Expression::FUNCTION {parameters, body}
  }

  fn parse_function_parameters(&mut self) -> Vec<Identifier> {
    let mut identifiers = Vec::new();
    if self.peek_token_is(Token::RPAREN) {
      self.next_token();
      return identifiers;
    };
    self.next_token();
    if let Token::IDENT(s) = &self.current_token {
      identifiers.push(Identifier{value: s.clone()});
    } else {
      // TODO: add error
      // the guy doesn't check for the tokens to be identifiers, for some reason
      panic!();
    };
    while self.peek_token_is(Token::COMMA) {
      self.next_token();
      self.next_token();
      if let Token::IDENT(s) = &self.current_token {
        identifiers.push(Identifier{value: s.clone()});
      } else {
        // TODO: add error
        panic!();
      }
    };
    if !self.expect_peek(Token::RPAREN) {
      // TODO: will need to add error
      panic!()
    }
    identifiers
  }

  fn parse_call_expression(&mut self, function: Expression) -> Expression {
    // let arguments = self.parse_call_arguments();
    let arguments = self.parse_expression_list(Token::RPAREN);
    Expression::CALL {function: Box::new(function), arguments}
  }


  // we implemented a broader method for parsing a list of expressions, this one is not needed anymore
  fn parse_call_arguments(&mut self) -> Vec<Expression> {
    let mut arguments = Vec::new();
    if self.peek_token_is(Token::RPAREN) {
      self.next_token();
      return arguments;
    };
    self.next_token();
    arguments.push(self.parse_expression(LOWEST));
    while self.peek_token_is(Token::COMMA) {
      self.next_token();
      self.next_token();
      arguments.push(self.parse_expression(LOWEST));
    };

    // println!("{:#?}", self.current_token);
    // println!("{:#?}", self.peek_token);
    

    if !self.expect_peek(Token::RPAREN) {
      // TODO: also add an error, or rewrite everything with result/option or something
      panic!();
    };
    arguments
  }

  fn expect_peek(&mut self, token: Token) -> bool {
    // println!("entered expect peek, token={:?}, self.peek_token={:?}", token, self.peek_token);
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
      Token::EQ       | Token::NOTEQ    => EQUALS,
      Token::LT       | Token::GT       => LESSGREATER,
      Token::PLUS     | Token::MINUS    => SUM,
      Token::SLASH    | Token::ASTERISK => PRODUCT,
      Token::LPAREN                     => CALL,
      Token::LBRACKET                   => INDEX,
      _                                 => LOWEST  // TODO: add the errors
    }
  }

  fn peek_precedence(&self) -> usize {
    self.get_precedence(self.peek_token.clone())
  }

  fn current_precedence(&self) -> usize {
    self.get_precedence(self.current_token.clone())
  }
}