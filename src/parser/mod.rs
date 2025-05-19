use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Program, Statement};

pub struct Parser {
  lexer: Lexer,
  current_token: Option<Token>,
  peek_token: Option<Token>
}

impl Parser {
  pub fn new(source: &str) -> Self {
    let mut parser = Self {
      lexer: Lexer::new(source),
      current_token: None,
      peek_token: None
    };
    parser.next_token();
    parser.next_token();
    parser
  }

  fn next_token(&mut self) {
    self.current_token = self.peek_token.take();
    self.peek_token = Some(self.lexer.next_token());
  }

  fn parse_program(&mut self) -> Program {
    let mut program = Program::new();
    while let Some(t) = &self.current_token {
      if *t != Token::EOF {
        let stmt = self.parse_statement();
        if let Some(s) = stmt {
          program.statements.push(s);
        }
      }
      self.next_token();
    }
    program
  }

  fn parse_statement(&mut self) -> Option<Statement> {
    match self.current_token.as_ref() {
      Some(t) => {
        match t {
          Token::LET => return self.parse_let_statement(),
          _          => panic!()
        }
      },
      None    => panic!()
    }
  }

  // TODO: reread the chapter, write tests, think of alternative way to the go brainrot
  fn parse_let_statement(&self) -> Option<Statement> {
    // let statement = Statement::LET { name: (), value: () } 
    // if !self.expect_peek(Token::IDENT(_)) {
      // return None;
    // }
    None
  }

  fn expect_peek(&mut self, token: Token) -> bool {
    todo!()
  }
}