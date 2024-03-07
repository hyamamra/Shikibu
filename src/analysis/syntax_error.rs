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
}
