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

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('=') => Token::new(TokenType::Assign, "=".to_string()),
            Some('+') => Token::new(TokenType::Plus, "+".to_string()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string()),
            Some(';') => Token::new(TokenType::Semicolon, ";".to_string()),
            Some('(') => Token::new(TokenType::Lparen, "(".to_string()),
            Some(')') => Token::new(TokenType::Rparen, ")".to_string()),
            Some('{') => Token::new(TokenType::Lbrace, "{".to_string()),
            Some('}') => Token::new(TokenType::Rbrace, "}".to_string()),
            None => Token::new(TokenType::Eof, "".to_string()),
            _ => Token::new(TokenType::Illegal, "".to_string()),
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = "let two = 2;";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Ident, "two".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Int, "2".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "".to_string())
        ];
        
        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }
}
