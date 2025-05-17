mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
    let mut a = lexer::Lexer::new("(=)+)");
    a.next_token();
    a.next_token();
    a.next_token();
    a.next_token();
    a.next_token();
    a.next_token();
    a.next_token();
}
