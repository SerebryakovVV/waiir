#![allow(dead_code, unused_imports, unused_variables)]

use std::io::{self, Write};
use crate::evaluator::environment::Environment;
use crate::lexer;
use crate::parser::Parser;
use crate::token::Token;
use crate::evaluator::eval;


pub const PROMPT: &str = ">> ";

// pub fn start() {
//   let mut input_buffer = String::new();
//   loop {
//     print!("{}", PROMPT);
//     io::stdout().flush().expect("Failed to flush stdout");
//     io::stdin().read_line(&mut input_buffer).unwrap();
//     if input_buffer.trim() == "q" {return;}
//     let mut a = lexer::Lexer::new(&input_buffer);
//     let mut tokens: Vec<Token> = Vec::new();
//     loop {
//     let tkn = a.next_token();
//       if tkn == Token::EOF {
//         break;
//       } else {
//         tokens.push(tkn);
//       }
//     }
// 		for t in tokens.iter() {
// 			println!("{:?}", t);
// 		}
// 		input_buffer.clear();
//   }
// }

pub fn start() {
  let mut input_buffer = String::new();
  let mut env = Environment::new();
  loop {
    print!("{}", PROMPT);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input_buffer).unwrap();
    if input_buffer.trim() == "q" {return;}
    let mut prsr = Parser::new(&input_buffer);
    let prgrm = prsr.parse_program();
    let res = eval(prgrm, &mut env);   // TODO: again, i need to do something with this enum wrapping stuff 
    println!("{}", res);


		input_buffer.clear();
  }
}