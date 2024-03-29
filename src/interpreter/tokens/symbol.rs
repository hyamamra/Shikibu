#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Asterisk,
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
    OpenBracket,
    CloseBracket,
    Comma,
    Bullet,
    Tilde,
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::Plus => "+",
            Symbol::Minus => "-",
            Symbol::Asterisk => "*",
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
            Symbol::OpenBracket => "[",
            Symbol::CloseBracket => "]",
            Symbol::Comma => ",",
            Symbol::Bullet => ".",
            Symbol::Tilde => "~",
        }
        .to_string()
    }
}
