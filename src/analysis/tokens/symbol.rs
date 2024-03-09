#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    OpenParen,
    CloseParen,
    Comma,
    Tilde,
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::Plus => "+",
            Symbol::Minus => "-",
            Symbol::Star => "*",
            Symbol::Slash => "/",
            Symbol::Equal => "=",
            Symbol::EqualEqual => "==",
            Symbol::Bang => "!",
            Symbol::BangEqual => "!=",
            Symbol::Less => "<",
            Symbol::LessEqual => "<=",
            Symbol::Greater => ">",
            Symbol::GreaterEqual => ">=",
            Symbol::OpenParen => "(",
            Symbol::CloseParen => ")",
            Symbol::Comma => ",",
            Symbol::Tilde => "~",
        }
        .to_string()
    }
}
