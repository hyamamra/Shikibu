use super::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: Lexeme,
    pub position: i32,
}

impl Token {
    pub fn symbol(value: Symbol, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Symbol(value),
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn keyword(value: Keyword, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Keyword(value),
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn identifier(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Identifier(value),
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn string(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::String(value),
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn number(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Number(value),
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn spaces(len: usize, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Spaces(len),
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn newline() -> Token {
        Token {
            lexeme: Lexeme::Newline,
            position: -1,
        }
    }

    pub fn comment(position: usize) -> Token {
        Token {
            lexeme: Lexeme::Comment,
            position: min(position, i32::MAX as usize) as i32,
        }
    }

    pub fn indent() -> Token {
        Token {
            lexeme: Lexeme::Indent,
            position: -1,
        }
    }

    pub fn dedent() -> Token {
        Token {
            lexeme: Lexeme::Dedent,
            position: -1,
        }
    }

    pub fn invalid(c: char, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Invalid(c),
            position: min(position, i32::MAX as usize) as i32,
        }
    }
}
