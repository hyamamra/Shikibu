use std::str::FromStr;

#[derive(Debug, Clone)]
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
    /// ではない
    Not,
    /// 真
    True,
    /// 偽
    False,
    /// 無
    Nil,
    /// 関数
    Func,
    /// 戻す
    Return,
    /// ループ
    Loop,
    /// 次へ
    Continue,
    /// 抜ける
    Break,
    /// ログ
    Log,
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
            "ではない" => Ok(Keyword::Not),
            "真" => Ok(Keyword::True),
            "偽" => Ok(Keyword::False),
            "無" => Ok(Keyword::Nil),
            "関数" => Ok(Keyword::Func),
            "戻す" => Ok(Keyword::Return),
            "ループ" => Ok(Keyword::Loop),
            "次へ" => Ok(Keyword::Continue),
            "抜ける" => Ok(Keyword::Break),
            "ログ" => Ok(Keyword::Log),
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
            Keyword::Not => "ではない",
            Keyword::True => "真",
            Keyword::False => "偽",
            Keyword::Nil => "無",
            Keyword::Func => "関数",
            Keyword::Return => "戻す",
            Keyword::Loop => "ループ",
            Keyword::Continue => "次へ",
            Keyword::Break => "抜ける",
            Keyword::Log => "ログ",
        }
        .to_string()
    }
}
