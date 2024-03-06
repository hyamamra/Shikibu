use std::str::FromStr;

#[derive(Debug)]
pub enum Keyword {
    /// もし
    If,
    /// あるいは
    Elif,
    /// ちがえば
    Else,
    /// 真
    True,
    /// 偽
    False,
    /// 無
    Nil,
    /// 関数
    Func,
    /// 戻る
    Return,
    /// ループ
    Loop,
    /// 次へ
    Continue,
    /// 抜ける
    Break,
    /// 出力
    Print,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "もし" => Ok(Keyword::If),
            "あるいは" => Ok(Keyword::Elif),
            "ちがえば" => Ok(Keyword::Else),
            "真" => Ok(Keyword::True),
            "偽" => Ok(Keyword::False),
            "無" => Ok(Keyword::Nil),
            "関数" => Ok(Keyword::Func),
            "戻る" => Ok(Keyword::Return),
            "ループ" => Ok(Keyword::Loop),
            "次へ" => Ok(Keyword::Continue),
            "抜ける" => Ok(Keyword::Break),
            "出力" => Ok(Keyword::Print),
            _ => Err(()),
        }
    }
}
