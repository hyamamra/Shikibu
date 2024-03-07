#[derive(Debug, Clone)]
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
    LeftParen,
    RightParen,
    Ampersand,
    Pipe,
    Comma,
    Backslash,
    Bullet,
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
            Symbol::LeftParen => "(",
            Symbol::RightParen => ")",
            Symbol::Ampersand => "&",
            Symbol::Pipe => "|",
            Symbol::Comma => ",",
            Symbol::Backslash => "\\",
            Symbol::Bullet => "•",
        }
        .to_string()
    }
}
