#[derive(Debug)]
pub enum Node {
    Number(String),
    String(String),
    True,
    False,
    Null,
    Assignment {
        name: String,
        value: Box<Node>,
    },
    Variable(String),
    /// syntax is `関数` (name) `（` (args) `）` (body)
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Node>,
    },
    Call {
        name: String,
        args: Vec<Node>,
    },
    Or {
        left: Box<Node>,
        right: Box<Node>,
    },
    And {
        left: Box<Node>,
        right: Box<Node>,
    },
    Equality {
        left: Box<Node>,
        right: Box<Node>,
    },
    Relational {
        left: Box<Node>,
        right: Box<Node>,
    },
    Add {
        left: Box<Node>,
        right: Box<Node>,
    },
    Subtract {
        left: Box<Node>,
        right: Box<Node>,
    },
    Multiply {
        left: Box<Node>,
        right: Box<Node>,
    },
    Divide {
        left: Box<Node>,
        right: Box<Node>,
    },
    /// syntax is `もし` (condition) `なら` (then_part) `ちがえば` (else_part)
    If {
        condition: Box<Node>,
        then_part: Vec<Node>,
        else_part: Vec<Node>,
    },
    /// syntax is `くりかえす` (body)
    Loop {
        body: Vec<Node>,
    },
    /// syntax is `かえす` (value)
    Return(Box<Node>),
    /// syntax is `ぬける`
    Break,
    /// syntax is `つぎへ`
    Continue,
    /// syntax is `表示` (value)
    Print(Box<Node>),
}
