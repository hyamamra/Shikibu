use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    /// もし
    If,
    /// なら
    Then,
    /// もしくは
    Elif,
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
    /// 関数
    Function,
    /// かえす
    Return,
    /// くりかえし
    Loop,
    /// つぎへ
    Continue,
    /// ぬける
    Break,
    /// 表示
    Print,
    /// 配列
    Array,
    /// 長さ
    Length,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "もし" => Ok(Keyword::If),
            "なら" => Ok(Keyword::Then),
            "もしくは" => Ok(Keyword::Elif),
            "ちがえば" => Ok(Keyword::Else),
            "または" => Ok(Keyword::Or),
            "かつ" => Ok(Keyword::And),
            "真" => Ok(Keyword::True),
            "偽" => Ok(Keyword::False),
            "無" => Ok(Keyword::Null),
            "関数" => Ok(Keyword::Function),
            "かえす" => Ok(Keyword::Return),
            "くりかえし" => Ok(Keyword::Loop),
            "つぎへ" => Ok(Keyword::Continue),
            "ぬける" => Ok(Keyword::Break),
            "表示" => Ok(Keyword::Print),
            "配列" => Ok(Keyword::Array),
            "長さ" => Ok(Keyword::Length),
            _ => Err(()),
        }
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::If => "もし",
            Keyword::Then => "なら",
            Keyword::Elif => "もしくは",
            Keyword::Else => "ちがえば",
            Keyword::Or => "または",
            Keyword::And => "かつ",
            Keyword::True => "真",
            Keyword::False => "偽",
            Keyword::Null => "無",
            Keyword::Function => "関数",
            Keyword::Return => "かえす",
            Keyword::Loop => "くりかえし",
            Keyword::Continue => "つぎへ",
            Keyword::Break => "ぬける",
            Keyword::Print => "表示",
            Keyword::Array => "配列",
            Keyword::Length => "長さ",
        }
        .to_string()
    }
}
