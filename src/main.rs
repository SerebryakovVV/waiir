mod lexer;
mod token;
mod repl;
mod ast;
mod parser;
use token::Token;


fn main() {
    // let mut a = lexer::Lexer::new("
    //     let five = 5;
    //     let ten = 10;
    //     let add = fn(x, y) {
    //         x + y;
    //     };
    //     let result = add(five, ten);
    //     !-/*5;
    //     5 < 10 > 5;
    //     if (5 < 6) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    //     5 == 5;
    //     5 != 6;=");
    // let mut tokens: Vec<Token> = Vec::new();
    // loop {
    //     let tkn = a.next_token();
    //     if tkn == Token::EOF {
    //         break;
    //     } else {
    //         tokens.push(tkn);
    //     }
    // }
    // for t in tokens.iter() {
    //     println!("{:?}", t);
    // }

    repl::start();

}

#[cfg(test)]
mod tests {
  use super::*;

  mod lexer_tests {
    use super::*;
  
    #[test]
    fn test_lexer_empty_input() {
      let mut l = lexer::Lexer::new("");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
        tokens.push(tkn);
        }
      }
      assert_eq!(tokens, []);
    }

    #[test]
    fn test_lexer_nonempty_input() {
      let mut l = lexer::Lexer::new("+=hello");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
          tokens.push(tkn);
        }
      }
      assert_eq!(tokens, [Token::PLUS, Token::ASSIGN, Token::IDENT(String::from("hello"))]);
    }

    #[test]
    fn test_lexer_two_char_token() {
      let mut l = lexer::Lexer::new("1!=2");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
          tokens.push(tkn);
        }
      }
      assert_eq!(tokens, [Token::INT(String::from("1")), Token::NOTEQ, Token::INT(String::from("2"))]);
    }
  }

  
  mod parser_tests {
    use super::*;

    #[test]
    fn initialization_of_parser_fields() {
      let p = parser::Parser::new("let foo = 5;");
      assert_eq!(p.current_token, Token::LET);
      assert_eq!(p.peek_token, Token::IDENT(String::from("foo")));
    }
  }




}