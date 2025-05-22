#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  ILLEGAL,
  EOF,
  IDENT(String),
  INT(i32),
  EQ,
  NOTEQ,
  ASSIGN,
  PLUS,
  MINUS,
  BANG,
  ASTERISK,
  SLASH,
  LT,
  GT,
  COMMA,
  SEMICOLON,
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  FUNCTION,
  LET,
  TRUE,
  FALSE,
  IF,
  ELSE,
  RETURN
}


