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