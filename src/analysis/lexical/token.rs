use super::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol};

#[derive(Debug)]
pub struct Token {
    pub lexeme: Lexeme,
    pub position: usize,
}

impl Token {
    pub fn symbol(value: Symbol, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Symbol(value),
            position,
        }
    }

    pub fn keyword(value: Keyword, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Keyword(value),
            position,
        }
    }

    pub fn identifier(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Identifier(value),
            position,
        }
    }

    pub fn string(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::String(value),
            position,
        }
    }

    pub fn number(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Number(value),
            position,
        }
    }

    pub fn spaces(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Spaces(value),
            position,
        }
    }

    pub fn newline(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Newline(value),
            position,
        }
    }

    pub fn comment(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Comment(value),
            position,
        }
    }
}
