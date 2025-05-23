use std::io::{self, Write};
use crate::lexer;
use crate::token::Token;

pub const PROMPT: &str = ">> ";

pub fn start() {
  let mut input_buffer = String::new();
  loop {
    print!("{}", PROMPT);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input_buffer).unwrap();
    if input_buffer.trim() == "q" {return;}
    let mut a = lexer::Lexer::new(&input_buffer);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
    let tkn = a.next_token();
      if tkn == Token::EOF {
        break;
      } else {
        tokens.push(tkn);
      }
    }
		for t in tokens.iter() {
			println!("{:?}", t);
		}
		input_buffer.clear();
  }
}