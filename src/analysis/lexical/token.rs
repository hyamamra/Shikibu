use super::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: Lexeme,
    pub position: Option<usize>,
}

impl Token {
    pub fn symbol(value: Symbol, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Symbol(value),
            position: Some(position),
        }
    }

    pub fn keyword(value: Keyword, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Keyword(value),
            position: Some(position),
        }
    }

    pub fn identifier(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Identifier(value),
            position: Some(position),
        }
    }

    pub fn string(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::String(value),
            position: Some(position),
        }
    }

    pub fn number(value: String, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Number(value),
            position: Some(position),
        }
    }

    pub fn spaces(len: usize, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Spaces(len),
            position: Some(position),
        }
    }

    pub fn newline() -> Token {
        Token {
            lexeme: Lexeme::Newline,
            position: None,
        }
    }

    pub fn comment(position: usize) -> Token {
        Token {
            lexeme: Lexeme::Comment,
            position: Some(position),
        }
    }

    pub fn indent() -> Token {
        Token {
            lexeme: Lexeme::Indent,
            position: None,
        }
    }

    pub fn dedent() -> Token {
        Token {
            lexeme: Lexeme::Dedent,
            position: None,
        }
    }

    pub fn invalid(c: char, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Invalid(c),
            position: Some(position),
        }
    }
}
