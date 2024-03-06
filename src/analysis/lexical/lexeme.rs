use super::{keyword::Keyword, symbol::Symbol};

#[derive(Debug)]
pub enum Lexeme {
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(String),
    Spaces(String),
    Newline(String),
    Comment(String),
    Indent,
    Dedent,
}
