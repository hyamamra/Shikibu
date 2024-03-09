use super::tokens::token::Token;

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub token: Option<Token>,
}

impl SyntaxError {
    pub fn invalid_char(c: char, position: usize) -> Self {
        Self {
            message: format!("Invalid character: {}", c),
            token: Some(Token::invalid(c, position)),
        }
    }

    pub fn unexpected_token(token: Token) -> Self {
        Self {
            message: format!("Unexpected token: {:?}", token.lexeme),
            token: Some(token),
        }
    }
}
