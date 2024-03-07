use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Keyword {
    /// もし
    If,
    /// なら
    Then,
    /// ちがえば
    Else,
    /// または
    Or,
    /// かつ
    And,
    /// 真
    True,
    /// 偽
    False,
    /// 無
    Null,
    /// しごと
    Function,
    /// かえす
    Return,
    /// くりかえす
    Loop,
    /// つぎへ
    Continue,
    /// ぬける
    Break,
    /// 表示
    Print,
    /// 例外
    Exception,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "もし" => Ok(Keyword::If),
            "なら" => Ok(Keyword::Then),
            "ちがえば" => Ok(Keyword::Else),
            "または" => Ok(Keyword::Or),
            "かつ" => Ok(Keyword::And),
            "真" => Ok(Keyword::True),
            "偽" => Ok(Keyword::False),
            "無" => Ok(Keyword::Null),
            "しごと" => Ok(Keyword::Function),
            "かえす" => Ok(Keyword::Return),
            "くりかえす" => Ok(Keyword::Loop),
            "つぎへ" => Ok(Keyword::Continue),
            "ぬける" => Ok(Keyword::Break),
            "表示" => Ok(Keyword::Print),
            "例外" => Ok(Keyword::Exception),
            _ => Err(()),
        }
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::If => "もし",
            Keyword::Then => "なら",
            Keyword::Else => "ちがえば",
            Keyword::Or => "または",
            Keyword::And => "かつ",
            Keyword::True => "真",
            Keyword::False => "偽",
            Keyword::Null => "無",
            Keyword::Function => "しごと",
            Keyword::Return => "かえす",
            Keyword::Loop => "くりかえす",
            Keyword::Continue => "つぎへ",
            Keyword::Break => "ぬける",
            Keyword::Print => "表示",
            Keyword::Exception => "例外",
        }
        .to_string()
    }
}
