use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Lexer{input, position: 0, read_position: 0, ch: None};
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &str {
        let start_position: usize = self.position;
        while let Some(ch) = self.ch {
            if ch.is_alphabetic() || ch == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        println!("read identifier: {}", &self.input[start_position..self.position]);
        &self.input[start_position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let start_position: usize = self.position;
        while let Some(ch) = self.ch {
            if ch.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }
        &self.input[start_position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut multi_char_read: bool = false;

        let token = match self.ch {
            Some('=') => Token::new(TokenType::Assign, "=".to_string()),
            Some('+') => Token::new(TokenType::Plus, "+".to_string()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string()),
            Some(';') => Token::new(TokenType::Semicolon, ";".to_string()),
            Some('(') => Token::new(TokenType::Lparen, "(".to_string()),
            Some(')') => Token::new(TokenType::Rparen, ")".to_string()),
            Some('{') => Token::new(TokenType::Lbrace, "{".to_string()),
            Some('}') => Token::new(TokenType::Rbrace, "}".to_string()),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let literal = self.read_identifier();
                let token_type = Token::lookup_ident(literal);
                multi_char_read = true;
                Token::new(token_type, literal.to_string())
            }
            Some(ch) if ch.is_digit(10) => {
                let literal = self.read_number();
                multi_char_read = true;
                Token::new(TokenType::Int, literal.to_string())  
            }
            None => Token::new(TokenType::Eof, "".to_string()),
            _ => Token::new(TokenType::Illegal, self.ch.unwrap().to_string()),
        };
        
        if !multi_char_read {
            self.read_char();
        }

        println!("token: {:?}", token);
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input: &str = "
            let two = 2;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(two, two);
        ";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "two".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "2".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Lbrace, "{".to_string()),
            Token::new(TokenType::Ident, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Ident, "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Rbrace, "}".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Ident, "add".to_string()),
            Token::new(TokenType::Lparen, "(".to_string()),
            Token::new(TokenType::Ident, "two".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Ident, "two".to_string()),
            Token::new(TokenType::Rparen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        
        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }
}
