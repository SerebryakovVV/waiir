#![allow(dead_code, unused_imports, unused_variables)]

mod lexer;
mod token;
mod repl;
mod ast;
mod parser;
mod evaluator;
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
      assert_eq!(tokens, [Token::INT(1), Token::NOTEQ, Token::INT(2)]);
    }

    #[test]
    fn test_read_string() {
      let mut l = lexer::Lexer::new("let a = \"my string\"");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
          tokens.push(tkn);
        }
      }
      assert_eq!(tokens, [Token::LET, Token::IDENT(String::from("a")), Token::ASSIGN, Token::STRING(String::from("my string"))]);
    }

    #[test]
    fn test_read_open_string() {
      let mut l = lexer::Lexer::new("let a = \"my string; 5 + 5");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
          tokens.push(tkn);
        }
      }
      assert_eq!(tokens, [Token::LET, Token::IDENT(String::from("a")), Token::ASSIGN, Token::ILLEGAL]);
    }

    #[test]
    fn test_read_string_tokens_after() {
      let mut l = lexer::Lexer::new("let a = \"my string\"; 5 + 5");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
          tokens.push(tkn);
        }
      }
      assert_eq!(tokens, [Token::LET, Token::IDENT(String::from("a")), Token::ASSIGN, Token::STRING(String::from("my string")), Token::SEMICOLON, Token::INT(5), Token::PLUS, Token::INT(5)]);
    }

    #[test]
    fn test_read_array() {
      let mut l = lexer::Lexer::new("let a = [1, 3]");
      let mut tokens: Vec<Token> = Vec::new();
      loop {
        let tkn = l.next_token();
        if tkn == Token::EOF {
          break;
        } else {
          tokens.push(tkn);
        }
      }
      assert_eq!(tokens, [Token::LET, Token::IDENT(String::from("a")), Token::ASSIGN, Token::LBRACKET, Token::INT(1), Token::COMMA, Token::INT(3), Token::RBRACKET]);
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

    #[test]
    fn initialization_of_parser_fields_with_empty_source() {
      let p = parser::Parser::new("");
      assert_eq!(p.current_token, Token::EOF);
      assert_eq!(p.peek_token, Token::EOF);
    }

    //  cargo test parser_tests -- --nocapture
    #[test]
    fn parse_let_statement_bare() {
      let mut prsr = parser::Parser::new("
        let foo = 5;
        return foo + 5 * 8;
        let a = 1000;  
      ");
      let program = prsr.parse_program();
      assert_eq!(program.statements[0], ast::Statement::LET { name: ast::Identifier { value: String::from("foo") }, value: ast::Expression::DUMMY });
      println!("printing the statements now:");
      for p in program.statements.iter() {
        println!("{:#?}", p)
      }
      println!("stopped printing the statements");
    }

    #[test]
    fn parse_ident_int_bare() {
      // let mut prsr = parser::Parser::new("let b = fn (a, b) {return a + b}");
      let mut prsr = parser::Parser::new("let b = [1, 2, 3]; b[1];");
      // let mut prsr = parser::Parser::new("add(a, b, c);");
      // let mut prsr = parser::Parser::new("add(a + b + c * d / f + g, 456 + sum);");
      // let mut prsr = parser::Parser::new("fn(x, y, z) {};");
      // let mut prsr = parser::Parser::new("if (x < y) { x } else {5+4*9};");
      // let mut prsr = parser::Parser::new("1 + (2 + 3) + 4;");
      // let mut prsr = parser::Parser::new("3 + 4 * 5 == 3 * 1 + 4 * 5;");
      // let mut prsr = parser::Parser::new("* 1 + 2 * 3;");
      let program = prsr.parse_program();
            println!("printing the statements now:");
      for p in program.statements.iter() {
        println!("{:#?}", p)
      }
      println!("stopped printing the statements");
    }

  }
}