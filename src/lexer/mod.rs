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
		// println!("{:#?}", lexer);
		lexer.read_char();
		// println!("{:#?}", lexer);
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
		if let Some(t) = self.ch {
			token = match t {
				'=' => Token::ASSIGN,
				';' => Token::SEMICOLON,
				'(' => Token::LPAREN,
				')' => Token::RPAREN,
				',' => Token::COMMA,
				'+' => Token::PLUS,
				'{' => Token::LBRACE,
				'}' => Token::RBRACE,
				_ => Token::ILLEGAL,
			}
		} else {
			// TODO: book interpretes this case as the end of file, need to check later
			// he also have some newToken function, mb can be replaced with just enum with values
			token = Token::EOF;
		}
		self.read_char();
		println!("{:?}", token);
		token
	}
}


