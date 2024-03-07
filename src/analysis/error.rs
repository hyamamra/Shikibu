use super::lexical::token::Token;

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub token: Token,
}

impl SyntaxError {
    pub fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }

    pub fn invalid_symbol(c: char, position: usize) -> Self {
        Self {
            message: format!("Invalid symbol: {}", c),
            token: Token::invalid(c, position),
        }
    }
}
