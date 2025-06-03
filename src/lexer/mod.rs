#![allow(dead_code, unused_imports, unused_variables)]

use crate::token::Token;


#[derive(Debug)]
pub struct Lexer {
  input: Vec<char>,
  position: usize,
  read_position: usize,
  ch: Option<char>
}


impl Lexer {
  pub fn new(source: &str) -> Self {
    let mut lexer = Self {
      input: source.chars().collect(),
      position: 0,					
      read_position: 0,			
      ch: None
    };
    lexer.read_char();
    lexer
  }

  pub fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = None;
    } else {
      self.ch = Some(self.input[self.read_position]);
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  pub fn next_token(&mut self) -> Token {
    let token;
    self.skip_whitespace();
    if let Some(t) = self.ch {
      token = match t {
        ';' => Token::SEMICOLON,
        '(' => Token::LPAREN,
        ')' => Token::RPAREN,
        ',' => Token::COMMA,
        '+' => Token::PLUS,
        '-' => Token::MINUS,
        '*' => Token::ASTERISK,
        '/' => Token::SLASH,
        '<' => Token::LT,
        '>' => Token::GT,
        '{' => Token::LBRACE,
        '}' => Token::RBRACE,
        '[' => Token::LBRACKET,
        ']' => Token::RBRACKET,
        '=' => {
          if let Some(c) = self.peek_char() {
            if c == '=' {
              self.read_char();
              Token::EQ
            } else {
              Token::ASSIGN
            }
          } else {
            Token::ASSIGN
          }
        },
        '!' => {
          if let Some(c) = self.peek_char() {
            if c == '=' {
              self.read_char();
              Token::NOTEQ
            } else {
              Token::BANG
            }
          } else {
            Token::BANG
          }
        },
        '"' => self.read_string(),
        _   => {
          if t.is_ascii_alphabetic() {
            let literal = self.read_identifier();
            return match literal.as_str() {
              "fn"     => Token::FUNCTION,
              "let"    => Token::LET,
              "true"   => Token::TRUE,
              "false"  => Token::FALSE,
              "if"     => Token::IF,
              "else"   => Token::ELSE,
              "return" => Token::RETURN,
              _        => Token::IDENT(literal)
            };
          } else if t.is_ascii_digit() {
            let num = self.read_number();
            let num = num.parse::<i32>().unwrap();
            return Token::INT(num);
          } else {
            Token::ILLEGAL
          }
        }
      }
    } else {
      // TODO: check this part
      token = Token::EOF;
    }
    self.read_char();
    token
  }

  fn read_string(&mut self) -> Token {
    self.read_char();
    let mut string_is_closed = false;
    let position = self.position;
    while let Some(c) = self.ch {
      if c != '"' {
        // position += 1;
        self.read_char();
      } else {
        string_is_closed = true;
        
        break;
      }
    }
    if string_is_closed {
      let tkn = Token::STRING(self.input[position..self.position].iter().collect::<String>());
      // self.read_char();
      return tkn;
    } else {
      // panic!()
      Token::ILLEGAL  // TODO: handling on the lexer stage, adding it in other places too
    }
  }

 fn read_identifier(&mut self) -> String {
    let position = self.position;
    while let Some(c) = self.ch {
      if c.is_ascii_alphabetic() || c == '_' {
        self.read_char();
      } else {break;}
    }
    return self.input[position..self.position].iter().collect::<String>()
  }

  fn read_number(&mut self) -> String {
    let position = self.position;
    while let Some(c) = self.ch {
      if c.is_ascii_digit() {
        self.read_char();
      } else {break;}
    }
    return self.input[position..self.position].iter().collect::<String>()
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.ch {
      if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
        self.read_char();
      } else {break;}
    }
  }
  
  fn peek_char(&self) -> Option<char> {
    if self.read_position >= self.input.len() {
      None
    } else {
      Some(self.input[self.read_position])
    }
  } 
}


