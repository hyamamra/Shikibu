use super::{keyword::Keyword, symbol::Symbol};

#[derive(Debug, Clone)]
pub enum Lexeme {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(String),
    Spaces(usize),
    Newline,
    Comment,
    Indent,
    Dedent,
    Invalid(char),
}

impl ToString for Lexeme {
    fn to_string(&self) -> String {
        match self {
            Lexeme::Symbol(symbol) => symbol.to_string(),
            Lexeme::Keyword(keyword) => keyword.to_string(),
            Lexeme::Identifier(identifier) => identifier.to_string(),
            Lexeme::String(string) => string.to_string(),
            Lexeme::Number(number) => number.to_string(),
            Lexeme::Spaces(len) => " ".repeat(*len),
            Lexeme::Invalid(c) => c.to_string(),
            _ => "".to_string(),
        }
    }
}
