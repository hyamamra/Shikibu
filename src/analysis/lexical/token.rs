use super::{keyword::Keyword, lexeme::Lexeme, symbol::Symbol};

#[derive(Debug, Clone)]
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

    pub fn spaces(len: usize, position: usize) -> Token {
        Token {
            lexeme: Lexeme::Spaces(len),
            position,
        }
    }

    pub fn newline(position: usize) -> Token {
        Token {
            lexeme: Lexeme::Newline,
            position,
        }
    }

    pub fn comment(position: usize) -> Token {
        Token {
            lexeme: Lexeme::Comment,
            position,
        }
    }

    pub fn indent(position: usize) -> Token {
        Token {
            lexeme: Lexeme::Indent,
            position,
        }
    }

    pub fn dedent(position: usize) -> Token {
        Token {
            lexeme: Lexeme::Dedent,
            position,
        }
    }
}
