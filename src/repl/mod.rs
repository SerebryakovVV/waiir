#![allow(dead_code, unused_imports, unused_variables)]

use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use crate::evaluator::environment::Environment;
use crate::lexer;
use crate::parser::Parser;
use crate::token::Token;



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
  let mut env = Rc::new(RefCell::new(Environment::new()));
  loop {
    print!("{}", PROMPT);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input_buffer).unwrap();
    if input_buffer.trim() == "q" {return;}
    let mut prsr = Parser::new(&input_buffer);
    let prgrm = prsr.parse_program();
    let res = prgrm.eval(Rc::clone(&env));   
    println!("{}", res);


		input_buffer.clear();
  }
}