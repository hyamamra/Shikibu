#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(String),
    String(String),
    List(Vec<Node>),
    Bool(bool),
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
    /// syntax is (left) `または` (right)
    Or {
        left: Box<Node>,
        right: Box<Node>,
    },
    /// syntax is (left) `かつ` (right)
    And {
        left: Box<Node>,
        right: Box<Node>,
    },
    Equal {
        left: Box<Node>,
        right: Box<Node>,
    },
    NotEqual {
        left: Box<Node>,
        right: Box<Node>,
    },
    LessThan {
        left: Box<Node>,
        right: Box<Node>,
    },
    LessThanOrEqual {
        left: Box<Node>,
        right: Box<Node>,
    },
    GreaterThan {
        left: Box<Node>,
        right: Box<Node>,
    },
    GreaterThanOrEqual {
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
    /// syntax is `くりかえし` (body)
    Loop {
        body: Vec<Node>,
    },
    /// syntax is `もどす` (value)
    Return(Box<Node>),
    /// syntax is `ぬける`
    Break,
    /// syntax is `つぎへ`
    Continue,
    /// syntax is `表示` (value)
    Print(Box<Node>),
}
