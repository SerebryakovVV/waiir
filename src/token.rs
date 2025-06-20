#[derive(Eq, Hash, Debug, PartialEq, Clone, Ord, PartialOrd)]
pub enum Token {
  ILLEGAL,
  EOF,
  IDENT(String),
  INT(i32),
  STRING(String),
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
  COLON,
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  LBRACKET,
  RBRACKET,
  FUNCTION,
  LET,
  TRUE,
  FALSE,
  IF,
  ELSE,
  RETURN
}


